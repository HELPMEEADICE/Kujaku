use core::cell::Cell;

use embassy_sync::{
    blocking_mutex::{Mutex, raw::CriticalSectionRawMutex},
    signal::Signal,
};
use embassy_time::Instant;

use crate::{
    board::CHANNEL_COUNT,
    drivers::{bq25798::Charge, power_mon::Power, tca9534::Event},
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    PeacockWave,
    BeatFanBurst,
    AudioVuFan,
    CallStrobe,
    Custom,
}

impl Mode {
    const MODES: [Self; 5] = [
        Self::PeacockWave,
        Self::BeatFanBurst,
        Self::AudioVuFan,
        Self::CallStrobe,
        Self::Custom,
    ];

    pub fn from_byte(value: u8) -> Option<Self> {
        Self::MODES.get(usize::from(value)).copied()
    }
}

#[derive(Clone, Copy)]
pub struct Controls {
    pub mode: Mode,
    pub master: u8,
    pub sensitivity: u8,
    pub custom: [u16; CHANNEL_COUNT],
    pub burst: u16,
}

impl Controls {
    const INITIAL: Self = Self {
        mode: Mode::PeacockWave,
        master: 50,
        sensitivity: 1,
        custom: [0; CHANNEL_COUNT],
        burst: 0,
    };

    pub fn event(&mut self, event: Event) {
        match event {
            Event::SingleClick => {
                self.mode = Mode::MODES[(self.mode as usize + 1) % Mode::MODES.len()]
            }
            Event::DoubleClick => self.burst = self.burst.wrapping_add(1),
            Event::LongPress => self.sensitivity = (self.sensitivity + 1) % 3,
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WorkMode {
    #[default]
    Standby,
    ChargeOnly,
    Show,
}

#[derive(Clone, Copy)]
pub struct Status {
    pub work: WorkMode,
    pub charge_wake: bool,
    pub mode_button: Option<bool>,
    pub flash_pressed: bool,
    pub lights_enabled: bool,
    pub cap: u16,
    pub power: Power,
    pub charge: Option<Charge>,
}

impl Status {
    pub const INITIAL: Self = Self {
        work: WorkMode::Standby,
        charge_wake: false,
        mode_button: None,
        flash_pressed: false,
        lights_enabled: false,
        cap: 0,
        power: Power {
            bus_mv: None,
            total: None,
            banks: [None, None],
        },
        charge: None,
    };
}

/// Q12 envelope (0..=4095 maps to 0..=1), with a counter for transient beats.
#[derive(Clone, Copy)]
pub struct Audio {
    pub envelope: u16,
    pub beat_counter: u16,
    pub updated_at: Option<Instant>,
}

impl Audio {
    pub const SILENT: Self = Self {
        envelope: 0,
        beat_counter: 0,
        updated_at: None,
    };
}

pub struct Shared {
    controls: Mutex<CriticalSectionRawMutex, Cell<Controls>>,
    status: Mutex<CriticalSectionRawMutex, Cell<Status>>,
    pub audio: Mutex<CriticalSectionRawMutex, Cell<Audio>>,
    pub frame: Mutex<CriticalSectionRawMutex, Cell<Option<[u16; CHANNEL_COUNT]>>>,
    pub wake_audio: Signal<CriticalSectionRawMutex, ()>,
    pub wake_render: Signal<CriticalSectionRawMutex, ()>,
    pub wake_advertiser: Signal<CriticalSectionRawMutex, ()>,
}

impl Shared {
    pub const fn new() -> Self {
        Self {
            controls: Mutex::new(Cell::new(Controls::INITIAL)),
            status: Mutex::new(Cell::new(Status::INITIAL)),
            audio: Mutex::new(Cell::new(Audio::SILENT)),
            frame: Mutex::new(Cell::new(None)),
            wake_audio: Signal::new(),
            wake_render: Signal::new(),
            wake_advertiser: Signal::new(),
        }
    }

    pub fn controls(&self) -> Controls {
        self.controls.lock(Cell::get)
    }

    pub fn edit(&self, update: impl FnOnce(&mut Controls)) {
        self.controls.lock(|cell| {
            let mut controls = cell.get();
            update(&mut controls);
            cell.set(controls);
        });
    }

    pub fn status(&self) -> Status {
        self.status.lock(Cell::get)
    }

    pub fn set_status(&self, status: Status) {
        let old = self.status.lock(|cell| cell.replace(status));
        if old.work != status.work {
            if status.work != WorkMode::Show {
                self.frame.lock(|cell| cell.set(None));
            }
            self.wake_audio.signal(());
            self.wake_render.signal(());
            self.wake_advertiser.signal(());
        } else if old.charge_wake != status.charge_wake
            || old.charge.map(|c| (c.status, c.fast)) != status.charge.map(|c| (c.status, c.fast))
        {
            self.wake_advertiser.signal(());
        }
    }
}

pub static SHARED: Shared = Shared::new();
