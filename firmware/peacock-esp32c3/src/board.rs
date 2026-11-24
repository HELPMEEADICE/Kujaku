//! Main V2.1 control-sheet nets and A2 wiring.
use esp_hal::peripherals::{GPIO0, GPIO1, GPIO3, GPIO4, GPIO5, GPIO6, GPIO7, GPIO10};

pub type ChgDet = GPIO0<'static>;
pub type PwmOe = GPIO1<'static>;
pub type FlashButton = GPIO3<'static>;
pub type Sda = GPIO4<'static>;
pub type Scl = GPIO5<'static>;
pub type MicBclk = GPIO6<'static>;
pub type MicWs = GPIO7<'static>;
pub type MicData = GPIO10<'static>;

// Strap pull-ups, BOOT SW1, and native USB D-/D+. These are left to the chip.
pub const BOOT_STRAPS: [u8; 3] = [2, 8, 9];
pub const USB_SERIAL_JTAG: [u8; 2] = [18, 19];

pub const HUSB238: u8 = 0x08;
pub const TCA9534: u8 = 0x20;
pub const PY32_RIGHT: u8 = 0x40;
pub const PY32_LEFT: u8 = 0x41;
pub const TOTAL_MONITOR: u8 = 0x44;
pub const PCA9685: u8 = 0x60;
pub const BQ25798: u8 = 0x6b;
pub const PCA_ALL_CALL: u8 = 0x70;
pub const I2C_HZ: u32 = 250_000;

pub const CHANNEL_COUNT: usize = 12;
pub const PWM_MAX: u16 = 4095;
/// Logical CH1..CH12 -> physical LEDn, from the V2.1 netlist.
pub const PWM_CHANNELS: [usize; CHANNEL_COUNT] = [11, 10, 9, 8, 0, 1, 2, 3, 4, 5, 6, 7];

// Calibrated ADC at GPIO0, after the 1:2 divider.
pub const CHG_PRESENT_MV: u16 = 1_800;
pub const CHG_ABSENT_MV: u16 = 1_200;
// Effective pack availability and undervoltage hysteresis on VBAT_SW.
pub const BUS_ON_MV: u16 = 9_300;
pub const BUS_OFF_MV: u16 = 8_800;

pub const TOTAL_SOFT_MA: u32 = 18_000;
pub const TOTAL_SUSTAINED_MA: u32 = 15_000;
pub const TOTAL_CUTOFF_MA: u32 = 20_000;
pub const CHANNEL_SOFT_MA: u32 = 1_500;
pub const CHANNEL_CUTOFF_MA: u32 = 2_000;
pub const MAX_SIMULTANEOUS_CHANNELS: usize = (TOTAL_SOFT_MA / CHANNEL_CUTOFF_MA) as usize;
pub const CHARGE_DERATE_HALF_C: i16 = 130;
pub const CHARGE_RESUME_HALF_C: i16 = 120;
pub const SHOW_THERMAL_CUTOFF_HALF_C: i16 = 170;

pub fn i2c_name(address: u8) -> &'static str {
    match address {
        HUSB238 => "HUSB238",
        TCA9534 => "TCA9534",
        PY32_RIGHT => "PY32 right",
        PY32_LEFT => "PY32 left",
        TOTAL_MONITOR => "U7 total monitor",
        PCA9685 => "PCA9685",
        BQ25798 => "BQ25798",
        PCA_ALL_CALL => "PCA9685 All Call",
        _ => "other",
    }
}
