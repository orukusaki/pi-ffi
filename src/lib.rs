#![allow(dead_code)]

extern crate num;

pub mod gpio;
pub mod pwm;
pub mod st;
pub mod spi;
pub mod i2c;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Polarity {
	High = 1,
	Low = 0,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PUDControl {
    Off = 0b00,
    Down = 0b01,
    Up = 0b10,
}

const PADS_GPIO_0_27: u32 = 0x002c;
const PADS_GPIO_28_45: u32 = 0x0030;
const PADS_GPIO_46_53: u32 = 0x0034;
const PAD_PASSWRD: u32 = (0x5A << 24);
const PAD_SLEW_RATE_UNLIMITED: u32 = 0x10;
const PAD_HYSTERESIS_ENABLED: u32 = 0x08;
const PAD_DRIVE_2MA: u32 = 0x00;
const PAD_DRIVE_4MA: u32 = 0x01;
const PAD_DRIVE_6MA: u32 = 0x02;
const PAD_DRIVE_8MA: u32 = 0x03;
const PAD_DRIVE_10MA: u32 = 0x04;
const PAD_DRIVE_12MA: u32 = 0x05;
const PAD_DRIVE_14MA: u32 = 0x06;
const PAD_DRIVE_16MA: u32 = 0x07;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PadGroup {
    Gpio0To27 = 0,
    Gpio28To45 = 1,
    Gpio46To53 = 2,
}

pub trait GpioPin {
	fn gpio_number(self) -> u8;
}

/// Used to represent a pin on the Raspberry Pi V1 P1 header
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum RPiGPIOPin {
    P1Pin03 = 0,
    P1Pin05 = 1,
    P1Pin07 = 4,
    P1Pin08 = 14,
    P1Pin10 = 15,
    P1Pin11 = 17,
    P1Pin12 = 18,
    P1Pin13 = 21,
    P1Pin15 = 22,
    P1Pin16 = 23,
    P1Pin18 = 24,
    P1Pin19 = 10,
    P1Pin21 = 9,
    P1Pin22 = 25,
    P1Pin23 = 11,
    P1Pin24 = 8,
    P1Pin26 = 7,
}

impl GpioPin for RPiGPIOPin {
	fn gpio_number(self) -> u8 {self as u8}
}

// A pin on the Revision 2 P1 header
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum RPiV2GpioPin {
    P1Pin03 = 2,
    P1Pin05 = 3,
    P1Pin07 = 4,
    P1Pin08 = 14,
    P1Pin10 = 15,
    P1Pin11 = 17,
    P1Pin12 = 18,
    P1Pin13 = 27,
    P1Pin15 = 22,
    P1Pin16 = 23,
    P1Pin18 = 24,
    P1Pin19 = 10,
    P1Pin21 = 9,
    P1Pin22 = 25,
    P1Pin23 = 11,
    P1Pin24 = 8,
    P1Pin26 = 7,
    P5Pin03 = 28,
    P5Pin04 = 29,
    P5Pin05 = 30,
    P5Pin06 = 31,
}

impl GpioPin for RPiV2GpioPin {
	fn gpio_number(self) -> u8 {self as u8}
}

mod ffi {

	#[link(name = "bcm2835")]
	extern "C" {
	    pub fn bcm2835_init() -> i32;
	    pub fn bcm2835_close() -> i32;
	    pub fn bcm2835_set_debug(debug: u8);

	    pub fn bcm2835_gpio_fsel(pin: u8, mode: u8);
	    pub fn bcm2835_gpio_set(pin: u8);
	    pub fn bcm2835_gpio_clr(pin: u8);

	    pub fn bcm2835_gpio_set_multi(mask: u32);
	    pub fn bcm2835_gpio_clr_multi(mask: u32);

	    pub fn bcm2835_gpio_lev(pin: u8) -> u8;
	    pub fn bcm2835_gpio_eds(pin: u8) -> u8;
	    pub fn bcm2835_gpio_set_eds(pin: u8);
	    pub fn bcm2835_gpio_ren(pin: u8);
	    pub fn bcm2835_gpio_clr_ren(pin: u8);
	    pub fn bcm2835_gpio_fen(pin: u8);
	    pub fn bcm2835_gpio_clr_fen(pin: u8);
	    pub fn bcm2835_gpio_hen(pin: u8);
	    pub fn bcm2835_gpio_clr_hen(pin: u8);
	    pub fn bcm2835_gpio_len(pin: u8);
	    pub fn bcm2835_gpio_clr_len(pin: u8);
	    pub fn bcm2835_gpio_aren(pin: u8);
	    pub fn bcm2835_gpio_clr_aren(pin: u8);
	    pub fn bcm2835_gpio_afen(pin: u8);
	    pub fn bcm2835_gpio_clr_afen(pin: u8);
	    pub fn bcm2835_gpio_pud(pud: u8);
	    pub fn bcm2835_gpio_pudclk(pin: u8, on: u8);

	    pub fn bcm2835_gpio_write(pin: u8, on: u8);
	    pub fn bcm2835_gpio_write_multi(mask: u32, on: u8);
	    pub fn bcm2835_gpio_write_mask(value: u32, mask: u32);
	    pub fn bcm2835_gpio_set_pud(pin: u8, pud: u8);
	    
	    pub fn bcm2835_gpio_pad(group: u8) -> u32;
	    pub fn bcm2835_gpio_set_pad(group: u8, control: u32);
	    
	    pub fn bcm2835_delay(millis: u32);
	    pub fn bcm2835_delayMicroseconds(micros: u64);

	    pub fn bcm2835_spi_begin();
	    pub fn bcm2835_spi_end();
	    pub fn bcm2835_spi_setBitOrder(order: u8);
	    pub fn bcm2835_spi_setClockDivider(divider: u16);
	    pub fn bcm2835_spi_setDataMode(mode: u8);
	    pub fn bcm2835_spi_chipSelect(cs: u8);
	    pub fn bcm2835_spi_setChipSelectPolarity(cs: u8, active: u8);
	    pub fn bcm2835_spi_transfer(value: u8) -> u8;
	    pub fn bcm2835_spi_transfernb(tbuf: *mut u8, rbuf: *mut u8, len: u32);
	    pub fn bcm2835_spi_transfern(buf: *mut u8, len: u32);
	    pub fn bcm2835_spi_writenb(buf: *mut u8, len: u32);

	    pub fn bcm2835_i2c_begin();
	    pub fn bcm2835_i2c_end();
	    pub fn bcm2835_i2c_setSlaveAddress(addr: u8);
	    pub fn bcm2835_i2c_setClockDivider(divider: u16);
	    pub fn bcm2835_i2c_set_baudrate(baudrate: u32);
	    pub fn bcm2835_i2c_write(buf: *const u8, len: u32) -> u8;
	    pub fn bcm2835_i2c_read(buf: *mut u8, len: u32) -> u8;
	    pub fn bcm2835_i2c_read_register_rs(regaddr: *mut u8, buf: *mut u8, len: u32) -> u8;
	    pub fn bcm2835_i2c_write_read_rs(cmds: *mut u8, cmds_len: u32, buf: *mut u8, buf_len: u32) -> u8;

	    pub fn bcm2835_st_read() -> u64;
	    pub fn bcm2835_st_delay(offset_micros: u64, micros: u64);

	    pub fn bcm2835_pwm_set_clock(divisor: u32);
	    pub fn bcm2835_pwm_set_mode(channel: u8, markspace: u8, enabled: u8);
	    pub fn bcm2835_pwm_set_range(channel: u8, range: u32);
	    pub fn bcm2835_pwm_set_data(channel: u8, data: u32);
	}
}

/// Initialise the bcm2835, should be called at the start of your program
pub fn init() -> Result<(), &'static str> {
	match unsafe {ffi::bcm2835_init()} {
		1 => Ok(()),
		_ => Err("Init failed")
	}
}

/// Release bcm2835 resources
pub fn close() -> bool {
	unsafe {ffi::bcm2835_close() == 0}
}

/// When set to true, no data will be sent to the pins; it'll be written to `stdout`
/// instead
pub fn set_debug(value: bool) {
	unsafe {ffi::bcm2835_set_debug(value as u8)}
}

