#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};


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