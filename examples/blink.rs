//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer,
    delay::Delay,
    rcc,
};
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use embedded_hal::digital::v2::OutputPin;
use cortex_m::asm::delay;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        let location = _info.location();
        let message = _info.message();
        hprintln!("panic! location: {:?}, message: {:?}", location, message).unwrap();
    }
}

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(72.mhz())
        .freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    hprintln!("clock: {:?}", clocks.sysclk().0).unwrap();
    loop {
        delay(7200000 * 3);
        led.set_high().unwrap();

        delay(7200000 * 5);
        led.set_low().unwrap();
    }
}