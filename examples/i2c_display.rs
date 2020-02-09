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
use embedded_graphics::{image::Image, prelude::*, pixelcolor::BinaryColor};
use embedded_graphics::fonts::{Font6x8, Text, Font6x12};
use embedded_graphics::style::TextStyleBuilder;
use stm32f1xx_hal::i2c::{BlockingI2c, DutyCycle, Mode};
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::stm32;

use ssd1306::prelude::*;
use ssd1306::Builder;

use stm32f1xx_hal::timer::Timer;
use stm32f1xx_hal::gpio::gpiob::{PB8, PB9};
use stm32f1xx_hal::gpio::{Alternate, OpenDrain};
use stm32f1::stm32f103::I2C1;

type OledDisplay = GraphicsMode<I2cInterface<BlockingI2c<I2C1, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>>>;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc: stm32f1xx_hal::rcc::Rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(72.mhz())
        .freeze(&mut flash.acr);

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
        .size(DisplaySize::Display128x64)
        .with_rotation(DisplayRotation::Rotate0)
        .with_i2c_addr(0x3c)
        .connect_i2c(i2c).into();
    disp.init().unwrap();

    let orig_image: Image<BinaryColor> = Image::new(include_bytes!("./rust.raw"), 64, 64);

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(200.hz());
    let mut x_shift = 0;

    loop {
        block!(timer.wait()).unwrap();
        disp.clear();

        let shift_point = Point::new(x_shift, 0);
        let shifted_image = orig_image.translate(shift_point);
        shifted_image.draw(&mut disp);
        draw_text(&mut disp);
        disp.flush().unwrap();

        x_shift += 1;
        if x_shift >= 128 {x_shift = 0;}
    }
}

fn draw_text(disp: &mut OledDisplay) {
    let mut buf = [0u8; 64];

    let text_style_1 = TextStyleBuilder::new(Font6x12)
        .text_color(BinaryColor::On)
        .build();

    let text_1: &str = write_to::show(
        &mut buf,
        format_args!("Hello world! {:?}", text_style_1.font),
    ).unwrap();

    Text::new(text_1, Point::zero())
        .into_styled(text_style_1)
        .draw(disp);


    let text_style_2 = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .build();

    let text_2: &str = write_to::show(
        &mut buf,
        format_args!("Hello Rust! {:?}", text_style_2.font),
    ).unwrap();

    Text::new(text_2, Point::new(0, 16))
        .into_styled(text_style_2)
        .draw(disp);

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




// from https://stackoverflow.com/questions/50200268/how-can-i-use-the-format-macro-in-a-no-std-environment
pub mod write_to {
    use core::cmp::min;
    use core::fmt;

    pub struct WriteTo<'a> {
        buffer: &'a mut [u8],
        // on write error (i.e. not enough space in buffer) this grows beyond
        // `buffer.len()`.
        used: usize,
    }

    impl<'a> WriteTo<'a> {
        pub fn new(buffer: &'a mut [u8]) -> Self {
            WriteTo { buffer, used: 0 }
        }

        pub fn as_str(self) -> Option<&'a str> {
            if self.used <= self.buffer.len() {
                // only successful concats of str - must be a valid str.
                use core::str::from_utf8_unchecked;
                Some(unsafe { from_utf8_unchecked(&self.buffer[..self.used]) })
            } else {
                None
            }
        }
    }

    impl<'a> fmt::Write for WriteTo<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            if self.used > self.buffer.len() {
                return Err(fmt::Error);
            }
            let remaining_buf = &mut self.buffer[self.used..];
            let raw_s = s.as_bytes();
            let write_num = min(raw_s.len(), remaining_buf.len());
            remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
            self.used += raw_s.len();
            if write_num < raw_s.len() {
                Err(fmt::Error)
            } else {
                Ok(())
            }
        }
    }

    pub fn show<'a>(buffer: &'a mut [u8], args: fmt::Arguments) -> Result<&'a str, fmt::Error> {
        let mut w = WriteTo::new(buffer);
        fmt::write(&mut w, args)?;
        w.as_str().ok_or(fmt::Error)
    }
}
