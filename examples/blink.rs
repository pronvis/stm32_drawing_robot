//#![deny(unsafe_code)]
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
        hprintln!("panic! location: {:?}, message: {:?}", location, message).unwrap();
    }
}

#[entry]
unsafe fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    //    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash: stm32f1xx_hal::flash::Parts = dp.FLASH.constrain();
    let mut rcc: stm32f1xx_hal::rcc::Rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    // with those values one second equals 'delay_cycles: u32 = 72_000_000'
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(72.mhz())
        .freeze(&mut flash.acr);

    if !clocks.usbclk_valid() {
        panic!("Clock parameter values are wrong!");
    }

    // Acquire the GPIOC peripheral
    let mut gpioc: stm32f1xx_hal::gpio::gpioc::Parts = dp.GPIOC.split(&mut rcc.apb2);
    let mut gpiod: stm32f1xx_hal::gpio::gpiod::Parts = dp.GPIOD.split(&mut rcc.apb2);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpiod.pd1.into_push_pull_output(&mut gpiod.crl);
    // let mut led = gpioc.pc1.into_push_pull_output(&mut gpioc.crl);

    //
    //    let mut MODER_C: *mut u32 = unsafe { 0x40011000 as *mut u32 };
    //    unsafe {
    //        *MODER_C &= !(0b11<<26);  // clear bits 27 and 26 to zero
    //        *MODER_C |= (0b01<<26);   // "or" in the new value
    //    }
    //

    // 72_000_000 - 1s
    let delay_cycles: u32 = 72_000_000;
    loop {
        led.toggle().unwrap();
        delay(delay_cycles);

        let is_high = led.is_set_high().unwrap();
        hprintln!("is pin high = {}", is_high).unwrap();

        // led.set_high().unwrap();
        // delay(delay_cycles);

        // led.set_low().unwrap();
        // delay(delay_cycles);
        // {
        //     hprintln!("current x = {}", x).unwrap();
        //     x += 1;
        // }
    }

    //    let ODR_B: *u32  = *0x40020414;
    //
    //    unsafe {
    //        *ODR_B |= (1 << 13);
    //        *ODR_B &= !(1<<13);
    //    }
    //    unsafe {
    //        assert_eq!(core::ptr::read_volatile(y), 12);
    //    }
}
