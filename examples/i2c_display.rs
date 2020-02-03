//! Image was created with ImageMagick:
//!
//! ```bash
//! convert rust.png -depth 1 gray:rust.raw
//! ```


#![no_std]
#![no_main]
#![feature(panic_info_message)]

use nb::block;

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use embedded_graphics::{image::Image, prelude::*};
use stm32f1xx_hal::i2c::{BlockingI2c, DutyCycle, Mode};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::stm32;

use ssd1306::prelude::*;
use ssd1306::Builder;

use stm32f1xx_hal::timer::Timer;
use stm32f1xx_hal::gpio::gpiob::{PB8, PB9};
use stm32f1xx_hal::gpio::{Alternate, OpenDrain};
use stm32f1::stm32f103::I2C1;

use embedded_graphics::image::Image1BPP;
use embedded_graphics::pixelcolor::PixelColorU8;

type OledDisplay = GraphicsMode<I2cInterface<BlockingI2c<I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>>>;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc: stm32f1xx_hal::rcc::Rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio: stm32f1xx_hal::afio::Parts = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob: stm32f1xx_hal::gpio::gpiob::Parts = dp.GPIOB.split(&mut rcc.apb2);
    let scl: PB8<Alternate<OpenDrain>> = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda: PB9<Alternate<OpenDrain>> = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);


    let i2c: BlockingI2c<I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)> = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        100,
        1000,
        1000,
    );

    let mut disp: OledDisplay = Builder::new()
        .with_size(DisplaySize::Display128x64)
        .with_rotation(DisplayRotation::Rotate0)
        .with_i2c_addr(0x3c)
        .connect_i2c(i2c).into();
    disp.init().unwrap();

    let orig_image: Image1BPP<PixelColorU8> = Image::new(include_bytes!("./rust.raw"), 64, 64);

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(20.hz());
    let mut x_shift = 0;

    loop {
        block!(timer.wait()).unwrap();
        disp.clear();

        let shifted_image = orig_image.translate(Coord::new(x_shift, 0));
        disp.draw(shifted_image.into_iter());
        disp.flush().unwrap();

        x_shift += 1;
        if x_shift >= 128 {x_shift = 0;}
    }
}

//use cortex_m_semihosting::{debug, hprintln};
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        let _location = _info.location();
        let _message = _info.message();
//        hprintln!("panic! location: {:?}, message: {:?}", _location, _message).unwrap();
    }
}