use super::ffi;

pub fn read() -> u64 {
	unsafe {ffi::bcm2835_st_read() as u64}
}

pub fn delay(micros: u64) {
	unsafe {ffi::bcm2835_st_delay(ffi::bcm2835_st_read(), micros as u64)}
}