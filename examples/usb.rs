//! Demonstrates our ability to log over USB, and read
//! USB serial messages from a USB host.
//!
//! Success criteria: you see log messages when connecting
//! to the Teensy 4 using PuTTY of another serial console.
//! Baud rate is 9600.

#![no_std]
#![no_main]

extern crate panic_halt;

use bsp::rt;
use teensy4_bsp as bsp;

#[rt::entry]
fn main() -> ! {
    let mut p = bsp::Peripherals::take().unwrap();
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    // Initialize the USB stack with the default logging settings
    let mut usb_reader = bsp::usb::init(
        &systick,
        bsp::usb::LoggingConfig {
            filters: &[("usb", None)],
            ..Default::default()
        },
    )
    .unwrap();
    systick.delay(2000);
    p.ccm
        .pll1
        .set_arm_clock(bsp::hal::ccm::PLL1::ARM_HZ, &mut p.ccm.handle, &mut p.dcdc);
    let mut led: bsp::LED = bsp::configure_led(pins.p13);
    let mut buffer = [0; 256];
    loop {
        let bytes_read = usb_reader.read(&mut buffer);
        if bytes_read > 0 {
            let bytes = &buffer[..bytes_read];
            match core::str::from_utf8(bytes) {
                Ok(msg) => log::info!("Received message: {} ({:?})", msg, bytes),
                Err(e) => log::warn!(
                    "Read {} bytes, but could not interpret message {:?}: {:?}",
                    bytes_read,
                    bytes,
                    e
                ),
            }
        }

        log::error!("Something terrible happened!");
        log::warn!("Something happened, but we fixed it");
        log::info!("It's 31'C outside");
        log::debug!("Sleeping for 1 second...");
        log::trace!("{} + {} = {}", 3, 2, 3 + 2);
        led.toggle();
        systick.delay(5000);
    }
}
