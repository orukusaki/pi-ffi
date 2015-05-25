use super::{ffi, GpioPin};

/// 
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum FunctionSelect {
    Inpt = 0b000,
    Outp = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010,
}

pub struct Pin {
	pin: u8
}	

/// Initialiate a pin based on its header locator
pub fn from_location<T: GpioPin>(pin_location: T, function: FunctionSelect) -> Pin {	
	Pin::new(pin_location.gpio_number(), function)
}

/// Represents a GPIO pin
impl Pin {

	/// Initiate a pin from its gpio number
	pub fn new(pin_location: u8, function: FunctionSelect) -> Pin {	

		let pin = Pin {
			pin: pin_location
		};
		pin.set_func(function);
		pin
	}

	/// Select the function of the pin
	pub fn set_func(&self, function: FunctionSelect) {
		unsafe {ffi::bcm2835_gpio_fsel(self.pin as u8, function as u8)}
	}

	/// Set the output value to 1
	pub fn set(&self) {
		unsafe {ffi::bcm2835_gpio_set(self.pin as u8)}
	}

	/// Set the output value to 0
	pub fn clear(&self) {
		unsafe {ffi::bcm2835_gpio_clr(self.pin as u8)}
	}

	/// Read the Input value
	pub fn get(&self) -> u8 {
		unsafe {ffi::bcm2835_gpio_lev(self.pin as u8) as u8}
	}
}