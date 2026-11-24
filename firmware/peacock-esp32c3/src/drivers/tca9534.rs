use embassy_time::{Duration, Instant};
use esp_hal::i2c::master::Error;

use crate::{board::TCA9534, drivers::bus::Bus};

const DEBOUNCE: Duration = Duration::from_millis(20);
const DOUBLE_WINDOW: Duration = Duration::from_millis(250);
const LONG_HOLD: Duration = Duration::from_millis(600);

pub fn init(bus: &mut Bus<'_>) -> Result<(), Error> {
    bus.set_byte(TCA9534, 0x01, 0)?;
    bus.set_byte(TCA9534, 0x02, 0)?;
    bus.set_byte(TCA9534, 0x03, 0x01)
}

pub fn pressed(bus: &mut Bus<'_>) -> Result<bool, Error> {
    Ok(bus.byte(TCA9534, 0x00)? & 0x01 == 0)
}

#[derive(Clone, Copy)]
pub enum Event {
    SingleClick,
    DoubleClick,
    LongPress,
}

pub struct Debounce {
    raw: bool,
    since: Instant,
    stable: Option<bool>,
}

impl Debounce {
    pub fn new(now: Instant) -> Self {
        Self {
            raw: true,
            since: now,
            stable: None,
        }
    }

    pub fn stable(&self) -> Option<bool> {
        self.stable
    }

    /// Some(value) means a confirmed edge or the first confirmed sample.
    pub fn update(&mut self, now: Instant, pressed: bool) -> Option<bool> {
        if pressed != self.raw {
            self.raw = pressed;
            self.since = now;
        }
        if self.stable != Some(pressed) && now - self.since >= DEBOUNCE {
            self.stable = Some(pressed);
            return self.stable;
        }
        None
    }
}

pub struct Gestures {
    debounce: Debounce,
    armed: bool,
    down: Option<Instant>,
    released: Option<Instant>,
    second: bool,
}

impl Gestures {
    pub fn new(now: Instant) -> Self {
        Self {
            debounce: Debounce::new(now),
            armed: false,
            down: None,
            released: None,
            second: false,
        }
    }

    pub fn pressed(&self) -> Option<bool> {
        self.debounce.stable()
    }

    pub fn update(&mut self, now: Instant, pressed: bool) -> Option<Event> {
        let edge = self.debounce.update(now, pressed);
        if !self.armed {
            self.armed = self.pressed() == Some(false);
            return None;
        }
        let click_age = self.released.map(|t| now - t);
        if edge == Some(true) {
            self.second = click_age.is_some_and(|age| age <= DOUBLE_WINDOW);
            self.released = None;
            self.down = Some(now);
            return click_age
                .is_some_and(|age| age > DOUBLE_WINDOW)
                .then_some(Event::SingleClick);
        }
        if self.down.is_some_and(|t| now - t >= LONG_HOLD) {
            self.down = None;
            self.second = false;
            self.released = None;
            return Some(Event::LongPress);
        }
        if edge == Some(false) && self.down.take().is_some() {
            if self.second {
                self.second = false;
                return Some(Event::DoubleClick);
            }
            self.released = Some(now);
        } else if click_age.is_some_and(|age| age >= DOUBLE_WINDOW) {
            self.released = None;
            return Some(Event::SingleClick);
        }
        None
    }
}
