use super::ffi;
use super::Polarity;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum BitOrder {
    BitOrderLsbfirst = 0u8,
    BitOrderMsbfirst = 1u8,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Mode {
    Mode0 = 0u8,
    Mode1 = 1u8,
    Mode2 = 2u8,
    Mode3 = 3u8,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ChipSelect {
    Cs0 = 0u8,
    Cs1 = 1u8,
    Cs2 = 2u8,
    CsNone = 3u8,
}

#[repr(u16)]
#[derive(Copy, Clone)]
pub enum ClockDivider {
    Divider65536 = 0u16,
    Divider32768 = 32768u16,
    Divider16384 = 16384u16,
    Divider8192 = 8192u16,
    Divider4096 = 4096u16,
    Divider2048 = 2048u16,
    Divider1024 = 1024u16,
    Divider512 = 512u16,
    Divider256 = 256u16,
    Divider128 = 128u16,
    Divider64 = 64u16,
    Divider32 = 32u16,
    Divider16 = 16u16,
    Divider8 = 8u16,
    Divider4 = 4u16,
    Divider2 = 2u16,
    Divider1 = 1u16,
}

pub fn begin() {
	unsafe {ffi::bcm2835_spi_begin()}
}

pub fn set_bit_order(order: BitOrder) {
	unsafe {ffi::bcm2835_spi_setBitOrder(order as u8)}
}

pub fn set_data_mode(mode: Mode) {
	unsafe {ffi::bcm2835_spi_setDataMode(mode as u8)}
}

pub fn set_clock_divider(divider: ClockDivider) {
	unsafe {ffi::bcm2835_spi_setClockDivider(divider as u16)}
}

pub fn chip_select(mode: ChipSelect) {
	unsafe {ffi::bcm2835_spi_chipSelect(mode as u8)}
}

pub fn chip_select_polarity(mode: ChipSelect, active: Polarity) {
	unsafe {ffi::bcm2835_spi_setChipSelectPolarity(mode as u8, active as u8)}
}

pub fn write_bytes(src: &mut [u8]) {
	unsafe {ffi::bcm2835_spi_writenb(src.as_mut_ptr(), src.len() as u32)}
}

pub fn transfer(src: &mut [u8]) {
	unsafe {ffi::bcm2835_spi_transfernb(src.as_mut_ptr(), src.as_mut_ptr(), src.len() as u32)}
}

pub fn end() {
	unsafe {ffi::bcm2835_spi_end()}
}
