use embassy_time::{Duration, Instant};
use esp_println::println;

use crate::{
    board::{
        BUS_OFF_MV, BUS_ON_MV, CHANNEL_COUNT, CHANNEL_CUTOFF_MA, CHANNEL_SOFT_MA,
        CHARGE_DERATE_HALF_C, PWM_MAX, PY32_LEFT, PY32_RIGHT, SHOW_THERMAL_CUTOFF_HALF_C,
        TOTAL_CUTOFF_MA, TOTAL_MONITOR, TOTAL_SOFT_MA, TOTAL_SUSTAINED_MA,
    },
    drivers::bus::Bus,
};

#[derive(Clone, Copy)]
pub struct Total {
    pub ma: i16,
    pub mw: u32,
    pub estimated: bool,
    pub overflow: bool,
}

#[derive(Clone, Copy)]
pub struct Bank {
    pub mv: u16,
    pub ma: [u16; 6],
}

#[derive(Clone, Copy, Default)]
pub struct Power {
    pub bus_mv: Option<u16>,
    pub total: Option<Total>,
    pub banks: [Option<Bank>; 2],
}

impl Power {
    pub fn can_show(&self, was_showing: bool) -> bool {
        let min = if was_showing { BUS_OFF_MV } else { BUS_ON_MV };
        self.bus_mv.is_some_and(|mv| (min..=13_500).contains(&mv))
    }

    pub fn channels(&self) -> [u16; CHANNEL_COUNT] {
        core::array::from_fn(|i| self.banks[i / 6].map_or(0, |bank| bank.ma[i % 6]))
    }
}

#[derive(Default)]
struct Ina226 {
    ready_at: Option<Instant>,
    retry_at: Option<Instant>,
    unknown_id_logged: bool,
}

impl Ina226 {
    fn poll(
        &mut self,
        bus: &mut Bus<'_>,
        now: Instant,
        show: bool,
    ) -> Option<(u16, Option<Total>)> {
        if self.retry_at.is_some_and(|t| now < t) {
            return None;
        }
        let data = self.read(bus, now, show);
        if data.is_none() && self.ready_at.is_none() {
            self.retry_at = Some(now + Duration::from_secs(1));
        }
        data
    }

    fn read(
        &mut self,
        bus: &mut Bus<'_>,
        now: Instant,
        show: bool,
    ) -> Option<(u16, Option<Total>)> {
        if let Some(ready_at) = self.ready_at {
            if now < ready_at {
                return None;
            }
            let data = (|| {
                // Calibration resets to zero on device reset.
                if bus.word(TOTAL_MONITOR, 0x05).ok()? != 2560 {
                    return None;
                }
                let flags = bus.word(TOTAL_MONITOR, 0x06).ok()?;
                if flags & 0x08 == 0 {
                    return None;
                }
                let mv = (u32::from(bus.word(TOTAL_MONITOR, 0x02).ok()?) * 5 / 4) as u16;
                let total = if show {
                    let overflow = flags & 0x04 != 0;
                    let (ma, mw) = if overflow {
                        (0, 0)
                    } else {
                        (
                            bus.word(TOTAL_MONITOR, 0x04).ok()? as i16,
                            u32::from(bus.word(TOTAL_MONITOR, 0x03).ok()?) * 25,
                        )
                    };
                    Some(Total {
                        ma,
                        mw,
                        estimated: false,
                        overflow,
                    })
                } else {
                    None
                };
                Some((mv, total))
            })();
            if data.is_none() {
                self.ready_at = None;
            }
            return data;
        }

        let manufacturer = bus.word(TOTAL_MONITOR, 0xfe).ok()?;
        let device = bus.word(TOTAL_MONITOR, 0xff).ok()?;
        if manufacturer != 0x5449 || device & 0xfff0 != 0x2260 {
            if !self.unknown_id_logged {
                println!("U7 0x44: unverified ID {:04x}/{:04x}", manufacturer, device);
                self.unknown_id_logged = true;
            }
            return None;
        }
        // Four samples averaged, 1.1 ms each for shunt and bus, continuous.
        bus.set_word(TOTAL_MONITOR, 0x00, 0x4327).ok()?;
        // CAL = 0.00512 / (0.001 A * 0.002 ohm), power LSB = 25 mW.
        bus.set_word(TOTAL_MONITOR, 0x05, 2560).ok()?;
        self.ready_at = Some(now + Duration::from_millis(15));
        println!("U7 0x44: INA226 register map verified");
        None
    }
}

#[derive(Default)]
struct Py32 {
    last: Option<(u16, Instant)>,
}

impl Py32 {
    fn poll(&mut self, bus: &mut Bus<'_>, now: Instant, addr: u8, bank: u8) -> Option<Bank> {
        // Proposed V1 contract. The supplier's register map is still needed.
        let mut frame = [0; 21];
        bus.read(addr, 0x00, &mut frame).ok()?;
        if frame[..4] != [b'P', b'Y', 1, bank] || crc8(&frame[..20]) != frame[20] {
            return None;
        }
        let le = |i| u16::from_le_bytes([frame[i], frame[i + 1]]);
        let seq = le(4);
        match self.last {
            Some((old, last_update)) if old == seq => {
                if now - last_update >= Duration::from_millis(150) {
                    return None;
                }
            }
            _ => self.last = Some((seq, now)),
        }
        let mv = le(6);
        (mv <= 16_000).then(|| Bank {
            mv,
            ma: core::array::from_fn(|i| le(8 + 2 * i)),
        })
    }
}

fn crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0;
    for &byte in bytes {
        crc ^= byte;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 {
                (crc << 1) ^ 0x07
            } else {
                crc << 1
            };
        }
    }
    crc
}

#[derive(Default)]
pub struct Monitor {
    ina: Ina226,
    py32: [Py32; 2],
}

impl Monitor {
    pub fn poll(&mut self, bus: &mut Bus<'_>, now: Instant, show: bool) -> Power {
        let measured = self.ina.poll(bus, now, show);
        let banks = if show || measured.is_none() {
            [
                self.py32[0].poll(bus, now, PY32_RIGHT, 0),
                self.py32[1].poll(bus, now, PY32_LEFT, 1),
            ]
        } else {
            [None, None]
        };
        let estimate = match banks {
            [Some(a), Some(b)] if show && a.mv.abs_diff(b.mv) <= 500 => {
                let mv = a.mv.min(b.mv);
                let ma: u32 = a.ma.into_iter().chain(b.ma).map(u32::from).sum();
                Some(Total {
                    ma: ma.min(i16::MAX as u32) as i16,
                    mw: (u64::from(ma) * u64::from(mv) / 1_000) as u32,
                    estimated: true,
                    overflow: false,
                })
            }
            _ => None,
        };
        let bus_mv = measured.map(|(mv, _)| mv).or(match banks {
            [Some(a), Some(b)] => Some(a.mv.min(b.mv)),
            [Some(a), None] | [None, Some(a)] => Some(a.mv),
            [None, None] => None,
        });
        Power {
            bus_mv,
            total: measured.and_then(|(_, total)| total).or(estimate),
            banks: if show { banks } else { [None, None] },
        }
    }
}

#[derive(Default)]
pub struct Limiter {
    applied: u16,
    sustained_ma: u32,
    cooldown_until: Option<Instant>,
}

fn proportional(current: u32, target: u32) -> u16 {
    (u32::from(PWM_MAX) * target / current.max(target)) as u16
}

impl Limiter {
    /// One update per power poll during show mode (20 ms cadence).
    pub fn update(&mut self, power: Power, temp: Option<i16>, now: Instant) -> u16 {
        let channels = power.channels();
        let sum: u32 = channels.iter().map(|&ma| u32::from(ma)).sum();
        let total = power
            .total
            .map_or(0, |total| total.ma.max(0) as u32)
            .max(sum);
        let peak = channels.into_iter().max().unwrap_or(0) as u32;
        // About five seconds to track sustained load, in fixed-point mA.
        self.sustained_ma = (249 * self.sustained_ma + 256 * total.min(TOTAL_CUTOFF_MA)) / 250;
        if total >= TOTAL_CUTOFF_MA
            || peak >= CHANNEL_CUTOFF_MA
            || power.total.is_some_and(|sample| sample.overflow)
            || temp.is_some_and(|t| t >= SHOW_THERMAL_CUTOFF_HALF_C)
        {
            self.cooldown_until = Some(now + Duration::from_millis(500));
        }

        if power.total.is_none() || self.cooldown_until.is_some_and(|t| now < t) {
            self.applied = 0;
            return 0;
        }
        let sustained = self.sustained_ma / 256;
        let mut target = proportional(total, TOTAL_SOFT_MA)
            .min(proportional(sustained, TOTAL_SUSTAINED_MA))
            .min(proportional(peak, CHANNEL_SOFT_MA));
        if power.banks.iter().any(Option::is_none) || power.total.is_some_and(|t| t.estimated) {
            target = target.min(PWM_MAX / 4);
        }
        if let Some(temp) = temp {
            let heat_range = i32::from(SHOW_THERMAL_CUTOFF_HALF_C - CHARGE_DERATE_HALF_C);
            let remaining =
                (i32::from(SHOW_THERMAL_CUTOFF_HALF_C) - i32::from(temp)).clamp(0, heat_range);
            target = target.min((i32::from(PWM_MAX) * remaining / heat_range) as u16);
        }

        // Over-limit readings reduce the current ceiling, so proportional
        // feedback also works when the previous frame was already derated.
        if total > TOTAL_SOFT_MA || sustained > TOTAL_SUSTAINED_MA || peak > CHANNEL_SOFT_MA {
            target = ((u32::from(target) * u32::from(self.applied)) / u32::from(PWM_MAX)) as u16;
        }
        self.applied = if target < self.applied {
            self.applied - (self.applied - target).div_ceil(2)
        } else {
            self.applied.saturating_add(80).min(target)
        };
        self.applied
    }
}
