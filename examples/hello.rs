//! Prints "Hello, world!" on the host console using semihosting


#![no_main]
#![no_std]

//extern crate panic_halt;
//extern crate stm32f103xx;
//extern crate stm32f1xx;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};


//use stm32f103xx::GPIOC;
//use stm32_hal::gpio::Port;

use panic_halt as _;
use nb::block;
use stm32f1xx_hal;


#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();
    hprintln!("Hello, second!").unwrap();
    hprintln!("Hello, third!").unwrap();

//    let cp = cortex_m::Peripherals::take().unwrap();
//    let p = stm32f103xx::Peripherals::take().unwrap();
//    p.PWR
//    p.GPIOA.idr.write(|w| w.pin0);
//
//    let gpioc = p.GPIOA.crl.write(|x| x.enable().set_bit().mode().set_bit());
//
//
//    // Get GPIOC somehow...
//    let gpioc = GPIOC.borrow(cs);
//    // Set pins 13, 14 and 15 on GPIOC to 1, 0 and 1.
//    gpioc.write_pin_range(13, 3, 0b101);

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
//    debug::exit(debug::EXIT_SUCCESS);

    let mut x = 10;
    loop {
        x += 1;
        if x % 100 == 0 {
            hprintln!("current x = {}", x).unwrap();
        }
    }

//    let mut x: i32  =0 ;
//
//    loop {
//
//        hprintln!("Hello from loop: {}", x).unwrap();
//        x += 1;
//
//    }
}
