#![deny(unsafe_code)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use cortex_m_semihosting::{debug, hprintln};
use nb::block;

use core::panic::PanicInfo;
use cortex_m::asm::delay;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{delay::Delay, pac, prelude::*, rcc, timer::Timer};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        let location = _info.location();
        let message = _info.message();
        //        hprintln!("panic! location: {:?}, message: {:?}", location, message).unwrap();
    }
}

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash: stm32f1xx_hal::flash::Parts = dp.FLASH.constrain();
    let mut rcc: stm32f1xx_hal::rcc::Rcc = dp.RCC.constrain();

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
    let mut gpioc: stm32f1xx_hal::gpio::gpioc::Parts = dp.GPIOC.split(&mut rcc.apb2);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(5.hz()); //will fail if less than 5 (timer.rs:268)
                                                                            //    hprintln!("clock: {:?}", clocks.pclk2().0).unwrap();
    loop {
        //        hprintln!("first block");
        block!(timer.wait()).unwrap();
        // delay(72_000_0 * 5); //have no idea why that number, but it works. Also it depends on Timer countdown
        led.set_high().unwrap();

        //        hprintln!("second block");
        // delay(72_000_0 * 5); //have no idea why that number, but it works. Also it depends on Timer countdown
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();
    }
}
