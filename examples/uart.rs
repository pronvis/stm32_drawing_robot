//! Serial interface loopback test
//!
//! You have to short the TX and RX pins to make this program work

#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(panic_info_message)]

//extern crate panic_halt;

use cortex_m::asm;

use nb::block;
use cortex_m_semihosting::{debug, hprintln};

use stm32f1xx_hal::{
    prelude::*,
    device,
    serial::Serial,
    timer::Timer,
};
use cortex_m_rt::entry;
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
    let p = device::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    // USART1
    // let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::usart3(
        p.USART3,
        (tx, rx),
        &mut afio.mapr,
        stm32f1xx_hal::serial::Config::default()
            .baudrate(9600.bps())
            .stopbits(stm32f1xx_hal::serial::StopBits::STOP2)
            .parity_odd(),
        clocks,
        &mut rcc.apb1,
    );

    let (mut tx, mut rx) = serial.split();

    let sent = b'X';

    hprintln!("hi there! 1");

    block!(tx.write(sent)).ok();

    hprintln!("hi there! 2");



    // we are blocked here - waiting for data form serial port!
    let received = block!(rx.read()).unwrap();

    hprintln!("hi there! 3");

    assert_eq!(received, sent);

    hprintln!("hi there! 4");

    asm::bkpt();


    let cp = cortex_m::Peripherals::take().unwrap();
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());


    loop {
        block!(tx.write(sent)).ok();

        hprintln!("hi there!");
        block!(timer.wait()).unwrap();
    }
}