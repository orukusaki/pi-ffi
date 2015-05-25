//! # Pulse Width Modulation (PWM) control.
//! This module allows control of the pwm pin on the P1 header (PWM0)
//! ## Calculating frequency calculations:
//!
//! The base clock is 19.2 Mhz
//! Select a divider and range to suit your application, based on the formula:
//! `frequency = 19.2E6 / divider / Range`
//! 
//! The range value decides how many steps you have between min and max. If you need
//! finer control, decrease the divider and increase the range to arrive back at the correct
//! frequency.
//!
//! Ultrasonic DC Motor:
//! `19.2E6 / 32 / 30 = 20,000 Hz`
//!
//! Servo (wide range):
//! `19.2E6 / 64 / 10000 = 30Hz`

use super::{gpio, ffi, RPiGPIOPin};
use gpio::FunctionSelect;
use num::Float;
use num::FromPrimitive;
use num::ToPrimitive;

pub struct Pwm {
	markspace: bool,
	enabled: bool,
	divider: ClockDivider,
	range: u32,
}

/// Initiate the pwm pin (Pin 18 / Gpio 18)
pub fn init(markspace: bool, enabled: bool, divider: ClockDivider, range: u32) -> Pwm {
	let mut pwm = Pwm {
		markspace: false,
		enabled: false,
		divider: ClockDivider::Divider1,
		range: 0,
	};

	let _ = gpio::from_location(RPiGPIOPin::P1Pin12, FunctionSelect::Alt5);

	pwm.set_mode(markspace, enabled);
	pwm.set_clock_divider(divider);
	pwm.set_range(range);
	pwm
}

impl Pwm {
	/// Set the clock divider, see module level docs for help picking the right one
	fn set_clock_divider(&mut self, divider: ClockDivider) {
		unsafe {ffi::bcm2835_pwm_set_clock(divider as u32)}
		self.divider = divider;
	}

	/// Set the pwm mode
	fn set_mode(&mut self, markspace: bool, enabled: bool) {
		unsafe {ffi::bcm2835_pwm_set_mode(0, markspace as u8, enabled as u8)}
		self.markspace = markspace;
		self.enabled = enabled;
	}

	/// Set the clock range, see module level docs for help picking the right one
	fn set_range(&mut self, range: u32) {
		unsafe {ffi::bcm2835_pwm_set_range(0, range as u32)}
		self.range = range;
	}

	/// Set the pwm data.  This should be a number between `0` and `range-1`
	pub fn set_data(&mut self, data: u32) {
		unsafe {ffi::bcm2835_pwm_set_data(0, data as u32)}
	}

	/// Set the pwm data as a fraction of `range` from a floating point number between 0 and 1
	pub fn set_fract<T>(&mut self, fract: T) 
	where T: Float + FromPrimitive + ToPrimitive {
		// only interested in numbers between 0 and 1
		let mul: T = FromPrimitive::from_u32(self.range).unwrap();
		let data = (
			mul * fract.fract()
			).to_u32()
		.unwrap();

		self.set_data(data);
	}
}

/// Clock divider (base clock is 19.2Mhz).
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum ClockDivider {
    Divider32768 = 32768,
    Divider16384 = 16384,
    Divider8192 = 8192,
    Divider4096 = 4096,
    Divider2048 = 2048,
    Divider1024 = 1024,
    Divider512 = 512,
    Divider256 = 256,
    Divider128 = 128,
    Divider64 = 64,
    Divider32 = 32,
    Divider16 = 16,
    Divider8 = 8,
    Divider4 = 4,
    Divider2 = 2,
    Divider1 = 1,
}