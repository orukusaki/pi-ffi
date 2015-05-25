use super::ffi;
use std::result;

const REASON_OK: u8 = 0;
const REASON_ERROR_NACK: u8 = 1;
const REASON_ERROR_CLKT: u8 = 2;
const REASON_ERROR_DATA: u8 = 4;

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ClockDivider {
    Divider2500 = 2500u16,
    Divider626 = 626u16,
    Divider150 = 150u16,
    Divider148 = 148u16,
}

pub struct SlaveDevice {
	address: u8,
	divider: ClockDivider,
}

pub type Result<T> = result::Result<T, &'static str>;

impl SlaveDevice {
	pub fn new (address: u8, divider: ClockDivider) -> SlaveDevice {

		unsafe {
			ffi::bcm2835_i2c_begin();
			ffi::bcm2835_i2c_setClockDivider(divider as u16)
		};
		SlaveDevice {
			address: address,
			divider: divider,
		}
	}

	pub fn read(&self) -> Result<u8> {

		let mut buff = [0u8];

		unsafe {ffi::bcm2835_i2c_setSlaveAddress(self.address as u8)};

		check_error(
			unsafe {ffi::bcm2835_i2c_read(buff.as_mut_ptr(), buff.len() as u32)}
		).map(|_| buff[0])
	}

	pub fn write(&self, buff: &[u8]) -> Result<()> {

		unsafe {ffi::bcm2835_i2c_setSlaveAddress(self.address as u8)};

		check_error(unsafe {ffi::bcm2835_i2c_write(buff.as_ptr(), buff.len() as u32)})
	}

	pub fn read_rs(&self, reg: u8) -> Result<u8> {
		let mut regaddr = reg;
		let mut buff = [0u8];

		unsafe {ffi::bcm2835_i2c_setSlaveAddress(self.address as u8)};

		check_error(
			unsafe {ffi::bcm2835_i2c_read_register_rs(&mut regaddr, buff.as_mut_ptr(), buff.len() as u32)}
		).map(|_| buff[0])
	}

	pub fn read_multi(&self, start: u8, length: usize) -> Result<Vec<u8>> {
		let mut regaddr = start;
		let mut buff = Vec::<u8>::with_capacity(length);
		for _ in 0..length {
			buff.push(0);
		};

		unsafe {ffi::bcm2835_i2c_setSlaveAddress(self.address as u8)};

		check_error(
			unsafe {ffi::bcm2835_i2c_read_register_rs(&mut regaddr, buff.as_mut_ptr(), length as u32)}
		).map(|_| buff)
	}
} 

fn check_error(code: u8) -> Result<()> {
	match code {
		REASON_OK => Ok(()),
		REASON_ERROR_NACK => Err("Nack Received"),
		REASON_ERROR_CLKT => Err("Clock Timeout"),
		REASON_ERROR_DATA => Err("Data Error"),
		_ => unreachable!(),
	}
}
