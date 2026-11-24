use embassy_time::{Duration, Instant};
use esp_hal::i2c::master::Error;

use crate::{
    board::{BQ25798, CHARGE_DERATE_HALF_C, CHARGE_RESUME_HALF_C, HUSB238},
    drivers::bus::Bus,
};

const ICHG: u8 = 0x03;
const VINDPM: u8 = 0x05;
const IINDPM: u8 = 0x06;
const CONTROL1: u8 = 0x10;
const CONTROL2: u8 = 0x11;
const CONTROL5: u8 = 0x14;
const STATUS0: u8 = 0x1b;
const STATUS1: u8 = 0x1c;
const ADC_CONTROL: u8 = 0x2e;
const ADC_ENABLE_CONTINUOUS: u8 = 0x80; // Continuous, 15 effective bits.
const ADC_FUNCTION_DISABLE_0: u8 = 0x2f;
const ADC_FUNCTION_DISABLE_1: u8 = 0x30;
const IBUS: u8 = 0x31;
const IBAT: u8 = 0x33;
const VBUS: u8 = 0x35;
const VBAT: u8 = 0x3b;
const TDIE: u8 = 0x41;
const EN_IINDPM: u8 = 1 << 2;
const EN_EXTILIM: u8 = 1 << 1;
const WATCHDOG_EXPIRED: u8 = 1 << 5;
const WATCHDOG_KICK_1S: u8 = 0x0a;
const INPUT_GOOD: u8 = 0x09; // PG_STAT and VBUS_PRESENT_STAT.

#[derive(Clone, Copy, Default)]
pub struct Charge {
    pub vbus_mv: u16,
    pub vbat_mv: u16,
    pub ibus_ma: i16,
    pub ibat_ma: i16,
    pub temp_half_c: i16,
    pub status: u8,
    pub fast: bool,
}

impl Charge {
    pub fn input_mw(self) -> u16 {
        (u32::from(self.vbus_mv) * u32::from(self.ibus_ma.max(0) as u16) / 1_000)
            .min(u32::from(u16::MAX)) as u16
    }
}

#[derive(Default)]
pub struct Charger {
    initialized: bool,
    fast: bool,
    hot: bool,
    good_pd_samples: u8,
    last_request: Option<Instant>,
}

impl Charger {
    pub fn disconnected(&mut self, bus: &mut Bus<'_>) {
        if self.fast {
            let _ = apply_standard(bus);
        }
        *self = Self::default();
    }

    pub fn poll(&mut self, bus: &mut Bus<'_>, now: Instant) -> Option<Charge> {
        match self.cycle(bus, now) {
            Ok(sample) => sample,
            Err(_) => {
                // Start each write independently so a partial transaction also
                // tries to restore the input clamp and the conservative limit.
                let _ = bus.set_word(BQ25798, ICHG, 100);
                let _ = set_input_clamp(bus, true);
                let _ = bus.set_word(BQ25798, IINDPM, 10);
                self.initialized = false;
                self.fast = false;
                self.good_pd_samples = 0;
                None
            }
        }
    }

    fn cycle(&mut self, bus: &mut Bus<'_>, now: Instant) -> Result<Option<Charge>, Error> {
        let status = bus.byte(BQ25798, STATUS0)?;
        if !self.initialized
            || status & WATCHDOG_EXPIRED != 0
            || bus.byte(BQ25798, ADC_CONTROL)? != ADC_ENABLE_CONTINUOUS
        {
            self.fast = false;
            self.good_pd_samples = 0;
            apply_standard(bus)?;
            let control = bus.byte(BQ25798, CONTROL2)?;
            // The six-pin USB-C receptacle has no D+/D- for detection.
            bus.set_byte(BQ25798, CONTROL2, control & !0xc0)?;
            bus.set_byte(BQ25798, ADC_FUNCTION_DISABLE_0, 0x0c)?;
            bus.set_byte(BQ25798, ADC_FUNCTION_DISABLE_1, 0xf0)?;
            bus.set_byte(BQ25798, ADC_CONTROL, ADC_ENABLE_CONTINUOUS)?;
            kick_watchdog(bus)?;
            self.initialized = true;
            // The next 250 ms poll takes the first settled ADC sample.
            return Ok(None);
        }

        let mut charge = Charge {
            status: bus.byte(BQ25798, STATUS1)? >> 5,
            vbus_mv: bus.word(BQ25798, VBUS)?,
            vbat_mv: bus.word(BQ25798, VBAT)?,
            ibus_ma: bus.word(BQ25798, IBUS)? as i16,
            ibat_ma: bus.word(BQ25798, IBAT)? as i16,
            temp_half_c: bus.word(BQ25798, TDIE)? as i16,
            fast: false,
        };
        let pd = self.poll_pd(bus, now);
        let sane_temp = (-80..=300).contains(&charge.temp_half_c);
        if !sane_temp || charge.temp_half_c > CHARGE_DERATE_HALF_C {
            self.hot = true;
        } else if charge.temp_half_c <= CHARGE_RESUME_HALF_C {
            self.hot = false;
        }
        let vbus_min = if self.fast { 13_400 } else { 14_000 };
        let good_pd = pd
            && status & INPUT_GOOD == INPUT_GOOD
            && (vbus_min..=16_500).contains(&charge.vbus_mv)
            && !self.hot;
        self.good_pd_samples = if good_pd {
            self.good_pd_samples.saturating_add(1)
        } else {
            0
        };
        let fast = self.good_pd_samples >= 2;
        if fast != self.fast {
            if fast {
                apply_fast(bus)?;
            } else {
                apply_standard(bus)?;
            }
            self.fast = fast;
        }
        kick_watchdog(bus)?;
        charge.fast = self.fast;
        Ok(sane_temp.then_some(charge))
    }

    fn poll_pd(&mut self, bus: &mut Bus<'_>, now: Instant) -> bool {
        let (Ok(contract), Ok(state)) = (bus.byte(HUSB238, 0x00), bus.byte(HUSB238, 0x01)) else {
            return false;
        };
        if state & 0x40 == 0 {
            return false;
        }
        // Explicit 15 V contract, >=2.25 A, attached and successful response.
        let volts = contract >> 4;
        let current_code = contract & 0x0f;
        if volts == 4 && current_code >= 7 && state & 0x38 == 0x08 {
            return true;
        }
        if self
            .last_request
            .is_some_and(|t| now - t < Duration::from_secs(2))
        {
            return false;
        }
        self.last_request = Some(now);
        if let Ok(pdo) = bus.byte(HUSB238, 0x05)
            && pdo & 0x80 != 0
            && pdo & 0x0f >= 7
            && bus.set_byte(HUSB238, 0x08, 0x80).is_ok()
        {
            let _ = bus.set_byte(HUSB238, 0x09, 0x01);
        }
        false
    }
}

fn kick_watchdog(bus: &mut Bus<'_>) -> Result<(), Error> {
    let control = bus.byte(BQ25798, CONTROL1)?;
    bus.set_byte(BQ25798, CONTROL1, (control & 0xf0) | WATCHDOG_KICK_1S)
}

fn set_input_clamp(bus: &mut Bus<'_>, use_external: bool) -> Result<(), Error> {
    let control = bus.byte(BQ25798, CONTROL5)? | EN_IINDPM;
    let control = if use_external {
        control | EN_EXTILIM
    } else {
        control & !EN_EXTILIM
    };
    bus.set_byte(BQ25798, CONTROL5, control)
}

fn apply_standard(bus: &mut Bus<'_>) -> Result<(), Error> {
    bus.set_word(BQ25798, ICHG, 100)?; // 1 A charge-current limit.
    set_input_clamp(bus, true)?;
    bus.set_word(BQ25798, IINDPM, 10)?; // 100 mA from an unqualified input.
    bus.set_byte(BQ25798, VINDPM, 44) // 4.4 V input floor.
}

fn apply_fast(bus: &mut Bus<'_>) -> Result<(), Error> {
    bus.set_byte(BQ25798, VINDPM, 134)?;
    // TI clamps writes above the ILIM_HIZ setting while EN_EXTILIM is set.
    // Stage the minimum before removing the clamp, then program and verify the
    // actual limit before increasing the charge-current limit.
    bus.set_word(BQ25798, IINDPM, 10)?;
    set_input_clamp(bus, false)?;
    bus.set_word(BQ25798, IINDPM, 205)?; // 2.05 A + 0.20 A wake budget.
    if bus.word(BQ25798, IINDPM)? != 205 {
        return Err(Error::ExecutionIncomplete);
    }
    bus.set_word(BQ25798, ICHG, 200)
}
