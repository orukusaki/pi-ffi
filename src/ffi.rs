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