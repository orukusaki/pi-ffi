extern crate pi_ffi;
use pi_ffi::*;
use pi_ffi::pwm::ClockDivider;
use std::sync::mpsc::Receiver;

/// Calculating frequency calculations:
/// Base clock is 19.2 Mhz
/// f = 19.2E6 / divider / Range
///
/// Ultrasonic DC Motor:
/// 19.2E6 / 32 / 30 = 20,000 Hz
///
/// Servo (wide range):
/// 19.2E6 / 64 / 10000 = 30Hz
fn main() {

	
	if !cfg!(target_arch="arm") {
		set_debug(true);
	}

	init().ok().expect("Failed to initiate GPIO");

	let range = 30;
	let mut pwm = pwm::init(true, true, ClockDivider::Divider32, range);

    let metronome = timer_periodic(100);

    let mut turn = 1;

	loop {

		for data in 0..range {
			let _ = metronome.recv();
			println!("{:?}", data);
			pwm.set_data(data);
		}

		turn = -turn;
	}
}

fn timer_periodic(ms: u32) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep_ms(ms);
            if tx.send(()).is_err() {
                break;
            }
        }
    });
    rx
}