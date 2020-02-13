#![no_main]
#![no_std]
#![feature(panic_info_message)]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use core::panic::PanicInfo;
use stm32f1xx_hal;

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
    hprintln!("Hello, second!").unwrap();
    hprintln!("Hello, third!").unwrap();

    let mut x = 10;
    loop {
        x += 1;
        if x % 100 == 0 {
            hprintln!("current x = {}", x).unwrap();
        }
    }

}