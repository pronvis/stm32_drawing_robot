#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use stm32f1xx_hal::{
    prelude::*,
    pac,
    timer::Timer,
    delay::Delay,
    rcc,
};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use cortex_m::asm::delay;
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
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

    let second = 72_000_000;
    loop {
        delay(second * 3);
        led.set_high().unwrap();

        delay(second * 5);
        led.set_low().unwrap();
    }
}