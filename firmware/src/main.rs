#![no_std]
#![no_main]
#![deny(unsafe_op_in_unsafe_fn)]

use defmt::debug;
use embedded_hal::blocking::delay::DelayMs;

// Panic handler
use panic_halt as _;

// Set up global defmt logger
use defmt_rtt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
	debug!("Booted");

	let p = embassy_nrf::init(Default::default());
	let mut delay = embassy_time::Delay;

	debug!("Initialized peripherals");
	delay.delay_ms(100 as u32);
	debug!("After delay");

	loop {}
}
