use esp_hal::{delay::Delay, i2c::master::Error};

use crate::{
    board::{CHANNEL_COUNT, MAX_SIMULTANEOUS_CHANNELS, PCA9685, PWM_CHANNELS, PWM_MAX},
    drivers::bus::Bus,
};

const MODE1: u8 = 0x00;
const MODE2: u8 = 0x01;
const LED0_ON_L: u8 = 0x06;
const PRESCALE: u8 = 0xfe;
const SLEEP: u8 = 1 << 4;
const AUTO_INCREMENT: u8 = 1 << 5;
const RESTART: u8 = 1 << 7;
const PWM_COUNTS: u16 = PWM_MAX + 1;
const GAMMA: [u16; 257] = include!(concat!(env!("OUT_DIR"), "/gamma.rs"));

pub fn init(bus: &mut Bus<'_>) -> Result<(), Error> {
    bus.set_byte(PCA9685, MODE1, SLEEP | AUTO_INCREMENT)?;
    // Push-pull, non-inverting, update on STOP, and LOW when nOE is high.
    bus.set_byte(PCA9685, MODE2, 0x04)?;
    bus.set_byte(PCA9685, PRESCALE, 3)?;
    bus.set_byte(PCA9685, MODE1, AUTO_INCREMENT)?;
    Delay::new().delay_micros(550);
    bus.set_byte(PCA9685, MODE1, RESTART | AUTO_INCREMENT)?;
    write_frame(bus, &[0; CHANNEL_COUNT], PWM_MAX)
}

pub fn configured(bus: &mut Bus<'_>) -> bool {
    bus.byte(PCA9685, MODE1)
        .is_ok_and(|mode| mode & 0x71 == AUTO_INCREMENT)
        && bus.byte(PCA9685, MODE2) == Ok(0x04)
        && bus.byte(PCA9685, PRESCALE) == Ok(3)
}

fn gamma(linear: u16) -> u16 {
    let position = u32::from(linear) * 256;
    let index = (position / u32::from(PWM_MAX)) as usize;
    let fraction = position % u32::from(PWM_MAX);
    let low = GAMMA[index];
    let high = GAMMA[(index + 1).min(256)];
    low + ((u32::from(high - low) * fraction + 2047) / u32::from(PWM_MAX)) as u16
}

pub fn write_frame(
    bus: &mut Bus<'_>,
    light: &[u16; CHANNEL_COUNT],
    electrical_limit: u16,
) -> Result<(), Error> {
    let mut packet = [0; 1 + 4 * CHANNEL_COUNT];
    packet[0] = LED0_ON_L;
    let mut duties = [0; CHANNEL_COUNT];
    for (&level, &led) in light.iter().zip(PWM_CHANNELS.iter()) {
        duties[led] = gamma(level);
    }
    let ceiling = limit_overlap(&duties, electrical_limit);
    for (led, level) in duties.into_iter().enumerate() {
        let duty = scale(level, ceiling);
        let base = 1 + 4 * led;
        // Spread rising edges across the 4096-count cycle.
        let on = phase(led);
        let off = (on + duty) & PWM_MAX;
        let mut counts = [on as u8, (on >> 8) as u8, off as u8, (off >> 8) as u8];
        if duty == 0 {
            counts = [0, 0, 0, 0x10];
        } else if duty == PWM_MAX {
            counts = [0, 0x10, 0, 0];
        }
        packet[base..base + 4].copy_from_slice(&counts);
    }
    bus.write(PCA9685, &packet)
}

fn phase(led: usize) -> u16 {
    led as u16 * PWM_COUNTS / CHANNEL_COUNT as u16
}

fn scale(level: u16, cap: u16) -> u16 {
    (u32::from(level) * u32::from(cap) / u32::from(PWM_MAX)) as u16
}

/// Keep the nominal 2 A per-channel model under 18 A even on a sudden white
/// frame. Overlap can increase only at a PWM rising edge.
fn limit_overlap(duties: &[u16; CHANNEL_COUNT], cap: u16) -> u16 {
    const ALWAYS_SAFE: u16 = PWM_COUNTS * MAX_SIMULTANEOUS_CHANNELS as u16 / CHANNEL_COUNT as u16;
    if cap <= ALWAYS_SAFE
        || duties.iter().filter(|&&duty| duty != 0).count() <= MAX_SIMULTANEOUS_CHANNELS
    {
        return cap;
    }
    let safe = |cap| {
        let pulse = duties.map(|duty| scale(duty, cap));
        (0..CHANNEL_COUNT).all(|edge| {
            let high = pulse.iter().enumerate().filter(|&(led, &duty)| {
                let elapsed = (phase(edge) + PWM_COUNTS - phase(led)) & PWM_MAX;
                duty == PWM_MAX || elapsed < duty
            });
            high.count() <= MAX_SIMULTANEOUS_CHANNELS
        })
    };
    if safe(cap) {
        return cap;
    }
    let (mut low, mut high) = (ALWAYS_SAFE, cap - 1);
    while low < high {
        let mid = low + (high - low).div_ceil(2);
        if safe(mid) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}
