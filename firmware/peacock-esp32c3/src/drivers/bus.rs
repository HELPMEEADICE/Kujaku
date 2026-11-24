use embassy_time::Timer;
use esp_hal::{
    Blocking,
    delay::Delay,
    gpio::{DriveMode, Flex, InputConfig, Level, OutputConfig, Pull},
    i2c::master::{BusTimeout, Config, Error, I2c, SoftwareTimeout},
    peripherals::I2C0,
    time::{Duration, Rate},
};
use esp_println::println;

use crate::board::{I2C_HZ, Scl, Sda, i2c_name};

pub struct BusPins {
    pub controller: I2C0<'static>,
    pub sda: Sda,
    pub scl: Scl,
}

impl BusPins {
    pub fn connect(&mut self) -> Bus<'_> {
        let config = Config::default()
            .with_frequency(Rate::from_hz(I2C_HZ))
            .with_timeout(BusTimeout::BusCycles(64))
            .with_software_timeout(SoftwareTimeout::Transaction(Duration::from_millis(5)));
        let i2c = I2c::new(self.controller.reborrow(), config)
            .expect("fixed 250 kHz I2C configuration")
            .with_sda(self.sda.reborrow())
            .with_scl(self.scl.reborrow());
        Bus { i2c, fault: None }
    }

    /// Call with the hardware controller dropped. Open-drain highs release the lines.
    pub fn clear(&mut self) -> bool {
        let mut sda = Flex::new(self.sda.reborrow());
        let mut scl = Flex::new(self.scl.reborrow());
        let config = OutputConfig::default().with_drive_mode(DriveMode::OpenDrain);
        for line in [&mut sda, &mut scl] {
            line.set_level(Level::High);
            line.apply_input_config(&InputConfig::default().with_pull(Pull::None));
            line.apply_output_config(&config);
            line.set_input_enable(true);
            line.set_output_enable(true);
        }
        let delay = Delay::new();
        let wait_scl = |scl: &Flex<'_>| {
            for _ in 0..50 {
                if scl.is_high() {
                    return true;
                }
                delay.delay_micros(4);
            }
            false
        };
        if !wait_scl(&scl) {
            return false;
        }
        if sda.is_high() {
            return true;
        }
        for _ in 0..9 {
            scl.set_low();
            delay.delay_micros(5);
            scl.set_high();
            if !wait_scl(&scl) {
                return false;
            }
            delay.delay_micros(5);
            if sda.is_high() {
                break;
            }
        }
        // STOP: drive SDA low with SCL low, then release SCL and finally SDA.
        scl.set_low();
        sda.set_low();
        delay.delay_micros(5);
        scl.set_high();
        let scl_released = wait_scl(&scl);
        delay.delay_micros(5);
        sda.set_high();
        delay.delay_micros(5);
        scl_released && sda.is_high()
    }
}

pub struct Bus<'a> {
    i2c: I2c<'a, Blocking>,
    fault: Option<Error>,
}

impl Bus<'_> {
    pub fn fault(&self) -> Option<Error> {
        self.fault
    }

    fn transaction<T>(
        &mut self,
        op: impl FnOnce(&mut I2c<'_, Blocking>) -> Result<T, Error>,
    ) -> Result<T, Error> {
        if let Some(error) = self.fault {
            return Err(error);
        }
        let result = op(&mut self.i2c);
        if let Err(error) = result
            && !matches!(error, Error::AcknowledgeCheckFailed(_))
        {
            self.fault = Some(error);
        }
        result
    }

    pub fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        self.transaction(|i2c| i2c.write(addr, data))
    }

    pub fn read(&mut self, addr: u8, reg: u8, data: &mut [u8]) -> Result<(), Error> {
        self.transaction(|i2c| i2c.write_read(addr, &[reg], data))
    }

    pub fn byte(&mut self, addr: u8, reg: u8) -> Result<u8, Error> {
        let mut data = [0; 1];
        self.read(addr, reg, &mut data)?;
        Ok(data[0])
    }

    pub fn word(&mut self, addr: u8, reg: u8) -> Result<u16, Error> {
        let mut data = [0; 2];
        self.read(addr, reg, &mut data)?;
        Ok(u16::from_be_bytes(data))
    }

    pub fn set_byte(&mut self, addr: u8, reg: u8, value: u8) -> Result<(), Error> {
        self.write(addr, &[reg, value])
    }

    pub fn set_word(&mut self, addr: u8, reg: u8, value: u16) -> Result<(), Error> {
        let [high, low] = value.to_be_bytes();
        self.write(addr, &[reg, high, low])
    }

    pub async fn scan(&mut self) {
        println!("I2C scan: 250 kHz, 7-bit addresses");
        for address in 0x08..=0x77 {
            if self.write(address, &[]).is_ok() {
                println!("  0x{:02x} {}", address, i2c_name(address));
            }
            if self.fault.is_some() {
                println!("I2C scan interrupted");
                return;
            }
            if address % 8 == 7 {
                Timer::after_millis(1).await;
            }
        }
        println!("I2C scan complete");
    }
}
