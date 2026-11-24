#![no_std]
#![no_main]

mod audio;
mod ble;
mod board;
mod drivers;
mod effects;
mod state;

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::{
    Blocking,
    analog::adc::{Adc, AdcCalCurve, AdcConfig, AdcPin, Attenuation},
    clock::CpuClock,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    peripherals::{ADC1, TIMG0},
    rng::Rng,
    time::Duration as HalDuration,
    timer::timg::{MwdtStage, MwdtStageAction, TimerGroup, Wdt},
};
use esp_println::println;

use crate::{
    board::{CHG_ABSENT_MV, CHG_PRESENT_MV, ChgDet},
    drivers::{
        bq25798::Charger,
        bus::{Bus, BusPins},
        pca9685,
        power_mon::{Limiter, Monitor},
        tca9534::{self, Debounce, Event, Gestures},
    },
    state::{SHARED, Status, WorkMode},
};

const IDLE_PERIOD: Duration = Duration::from_millis(50);
const POWER_PERIOD: Duration = Duration::from_millis(20);
const CHARGER_PERIOD: Duration = Duration::from_millis(250);
const PROBE_RETRY: Duration = Duration::from_millis(500);

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let p = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::_80MHz));
    let oe_pin: board::PwmOe = p.GPIO1;
    let flash_pin: board::FlashButton = p.GPIO3;
    let oe = Output::new(oe_pin, Level::High, OutputConfig::default());
    let flash = Input::new(flash_pin, InputConfig::default().with_pull(Pull::None));
    let mut adc_config = AdcConfig::new();
    let wake = adc_config
        .enable_pin_with_cal::<_, AdcCalCurve<ADC1<'static>>>(p.GPIO0, Attenuation::_11dB);
    let adc = Adc::new(p.ADC1, adc_config);

    let timg0 = TimerGroup::new(p.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    let mut watchdog = timg0.wdt;
    watchdog.enable();
    watchdog.set_timeout(MwdtStage::Stage0, HalDuration::from_secs(2));
    watchdog.set_stage_action(MwdtStage::Stage0, MwdtStageAction::ResetSystem);
    watchdog.feed();

    println!(
        "Peacock Main V2.1/A2, USB-Serial-JTAG GPIO {:?}, boot straps {:?}",
        board::USB_SERIAL_JTAG,
        board::BOOT_STRAPS
    );
    esp_alloc::heap_allocator!(size: 64 * 1024);
    let radio_timer = TimerGroup::new(p.TIMG1).timer0;
    let radio = esp_wifi::init(radio_timer, Rng::new(p.RNG));

    spawner
        .spawn(audio::capture(
            p.I2S0, p.DMA_CH0, p.GPIO6, p.GPIO7, p.GPIO10,
        ))
        .expect("single audio task");
    spawner
        .spawn(effects::render())
        .expect("single render task");
    match radio {
        Ok(controller) => spawner
            .spawn(ble::run(controller, p.BT))
            .expect("single BLE task"),
        Err(error) => println!("BLE unavailable: {:?}", error),
    }

    supervise(
        BusPins {
            controller: p.I2C0,
            sda: p.GPIO4,
            scl: p.GPIO5,
        },
        BoardIo {
            oe,
            flash,
            adc,
            wake,
        },
        watchdog,
    )
    .await;
}

struct BoardIo {
    oe: Output<'static>,
    flash: Input<'static>,
    adc: Adc<'static, ADC1<'static>, Blocking>,
    wake: AdcPin<ChgDet, ADC1<'static>, AdcCalCurve<ADC1<'static>>>,
}

impl BoardIo {
    fn charge_wake(&mut self, previous: bool) -> bool {
        let started = esp_hal::time::Instant::now();
        loop {
            match self.adc.read_oneshot(&mut self.wake) {
                Ok(mv) if previous => return mv >= CHG_ABSENT_MV,
                Ok(mv) => return mv >= CHG_PRESENT_MV,
                Err(nb::Error::WouldBlock) if started.elapsed().as_micros() < 250 => {}
                Err(_) => return previous,
            }
        }
    }
}

async fn supervise(mut pins: BusPins, mut io: BoardIo, mut watchdog: Wdt<TIMG0<'static>>) {
    let mut scanned = false;
    loop {
        watchdog.feed();
        if !pins.clear() {
            let charge_wake = io.charge_wake(SHARED.status().charge_wake);
            SHARED.set_status(disabled_status(charge_wake));
            println!("I2C held low; retrying bus clear");
            Timer::after_millis(250).await;
            continue;
        }
        let mut bus = pins.connect();
        if !scanned {
            bus.scan().await;
            scanned = bus.fault().is_none();
        }
        let mut runtime = Runtime::new(Instant::now(), SHARED.status().charge_wake);
        while bus.fault().is_none() {
            watchdog.feed();
            let start = Instant::now();
            runtime.step(&mut bus, &mut io, start);
            if bus.fault().is_none() {
                let period = if runtime.status.work == WorkMode::Show {
                    effects::FRAME_PERIOD
                } else {
                    IDLE_PERIOD
                };
                Timer::at((start + period).max(Instant::now() + Duration::from_millis(1))).await;
            }
        }
        io.oe.set_high();
        SHARED.set_status(disabled_status(runtime.status.charge_wake));
        println!("I2C {:?}; outputs disabled, clearing bus", bus.fault());
        Timer::after_millis(100).await;
    }
}

fn disabled_status(charge_wake: bool) -> Status {
    let work = if charge_wake {
        WorkMode::ChargeOnly
    } else {
        WorkMode::Standby
    };
    Status {
        work,
        charge_wake,
        ..Status::INITIAL
    }
}

struct Runtime {
    status: Status,
    monitor: Monitor,
    limiter: Limiter,
    charger: Charger,
    gestures: Gestures,
    flash: Debounce,
    flash_armed: bool,
    tca_online: bool,
    pwm_ready: bool,
    next_power: Instant,
    next_charger: Instant,
    next_tca_probe: Instant,
    next_pwm_probe: Instant,
}

impl Runtime {
    fn new(now: Instant, charge_wake: bool) -> Self {
        Self {
            status: disabled_status(charge_wake),
            monitor: Monitor::default(),
            limiter: Limiter::default(),
            charger: Charger::default(),
            gestures: Gestures::new(now),
            flash: Debounce::new(now),
            flash_armed: false,
            tca_online: false,
            pwm_ready: false,
            next_power: now,
            next_charger: now,
            next_tca_probe: now,
            next_pwm_probe: now,
        }
    }

    fn step(&mut self, bus: &mut Bus<'_>, io: &mut BoardIo, now: Instant) {
        let was_charging = self.status.charge_wake;
        self.status.charge_wake = io.charge_wake(was_charging);
        let was_showing = self.status.work == WorkMode::Show;
        let power_sampled = now >= self.next_power;
        if power_sampled {
            self.status.power = self.monitor.poll(bus, now, was_showing);
            self.next_power = now + POWER_PERIOD;
        }

        let work = if self.status.power.can_show(was_showing) {
            WorkMode::Show
        } else if self.status.charge_wake {
            WorkMode::ChargeOnly
        } else {
            WorkMode::Standby
        };
        if work != self.status.work {
            println!("Mode: {:?}", work);
            self.status.work = work;
            self.reset_show(io, now);
        }

        if self.status.charge_wake {
            if !was_charging || now >= self.next_charger {
                self.status.charge = self.charger.poll(bus, now);
                self.next_charger = now + CHARGER_PERIOD;
            }
        } else if was_charging {
            self.charger.disconnected(bus);
            self.status.charge = None;
        }

        if work == WorkMode::Show && bus.fault().is_none() {
            if power_sampled {
                self.status.cap = self.limiter.update(
                    self.status.power,
                    self.status.charge.map(|c| c.temp_half_c),
                    now,
                );
            }
            if self.status.cap == 0 {
                io.oe.set_high();
                self.status.lights_enabled = false;
            }
            self.poll_buttons(bus, io, now);
            self.refresh_pwm(bus, io, now);
        }
        if bus.fault().is_none() {
            SHARED.set_status(self.status);
        }
    }

    fn reset_show(&mut self, io: &mut BoardIo, now: Instant) {
        io.oe.set_high();
        self.status.lights_enabled = false;
        self.status.cap = 0;
        self.status.mode_button = None;
        self.status.flash_pressed = false;
        self.limiter = Limiter::default();
        self.gestures = Gestures::new(now);
        self.flash = Debounce::new(now);
        self.flash_armed = false;
        self.tca_online = false;
        self.pwm_ready = false;
        self.next_tca_probe = now;
        self.next_pwm_probe = now;
    }

    fn poll_buttons(&mut self, bus: &mut Bus<'_>, io: &BoardIo, now: Instant) {
        if !self.tca_online && now >= self.next_tca_probe {
            self.tca_online = tca9534::init(bus).is_ok();
            self.next_tca_probe = now + PROBE_RETRY;
        }
        if self.tca_online {
            match tca9534::pressed(bus) {
                Ok(pressed) => {
                    if let Some(event) = self.gestures.update(now, pressed) {
                        SHARED.edit(|controls| controls.event(event));
                    }
                    self.status.mode_button = self.gestures.pressed();
                }
                Err(_) => {
                    self.tca_online = false;
                    self.next_tca_probe = now + PROBE_RETRY;
                    self.gestures = Gestures::new(now);
                    self.status.mode_button = None;
                }
            }
        }
        if let Some(pressed) = self.flash.update(now, io.flash.is_low()) {
            if !pressed {
                self.flash_armed = true;
            } else if self.flash_armed {
                SHARED.edit(|controls| controls.event(Event::DoubleClick));
            }
        }
        self.status.flash_pressed = self.flash.stable() == Some(true);
    }

    fn refresh_pwm(&mut self, bus: &mut Bus<'_>, io: &mut BoardIo, now: Instant) {
        if now >= self.next_pwm_probe {
            if !self.pwm_ready || !pca9685::configured(bus) {
                io.oe.set_high();
                self.pwm_ready = pca9685::init(bus).is_ok();
            }
            self.next_pwm_probe = now
                + if self.pwm_ready {
                    Duration::from_secs(1)
                } else {
                    PROBE_RETRY
                };
        }
        let frame = SHARED.frame.lock(|slot| slot.get());
        let mut written = false;
        if self.pwm_ready
            && self.status.cap > 0
            && let Some(frame) = frame
        {
            written = pca9685::write_frame(bus, &frame, self.status.cap).is_ok();
            if !written {
                self.pwm_ready = false;
                self.next_pwm_probe = now + PROBE_RETRY;
            }
        }
        self.status.lights_enabled = written;
        io.oe
            .set_level(if written { Level::Low } else { Level::High });
    }
}
