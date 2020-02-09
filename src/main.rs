#![deny(unsafe_code)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use cortex_m_semihosting::{debug, hprintln};
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        let location = _info.location();
        let message = _info.message();
        hprintln!("panic! location: {:?}, message: {:?}", location, message).unwrap();
    }
}

//todo: trying to use https://github.com/invis87/svg_drawing , but compile fails because of:
//--> /Users/pronvis/.cargo/registry/src/github.com-1ecc6299db9ec823/lazy_static-1.4.0/src/inline_lazy.rs:9:1
//|
//9 | extern crate std;
//| ^^^^^^^^^^^^^^^^^ can't find crate

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

    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(200.hz());
    let lines_to_draw = draw_something();


    let text_style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .build();
    loop {
        block!(timer.wait()).unwrap();
        disp.clear();

        for line in lines_to_draw {
            match line {
                LineTo::Fly(_) => {},
                LineTo::Erase(_) => {},
                LineTo::Draw(draw) => {
                    Text::new(draw.x.to_str(), Point::zero())
                        .into_styled(text_style)
                        .draw(&mut disp);
                    disp.flush().unwrap();
                },
            }
        }

    }
}

use drawing_robot::svg::svg_curve;
use drawing_robot::svg::svg_curve::LineTo;

fn draw_something() -> impl Iterator<Item = LineTo> {
    let svg_string = "M198.901,545.277c-4.035-3.746-7.869-7.492-11.702-11.632c-4.641-5.126-7.264-11.435-9.08-17.941
						c-1.614-5.521-3.026-11.041-3.026-16.956c0-1.577,0-3.352-0.202-4.929c-2.421-11.436-1.009-23.068-1.21-34.503
						c1.614-6.506,1.413-13.407,2.018-20.11c2.018-7.492,1.211-15.378,3.43-22.87c3.43-23.462,8.676-43.375,15.334-65.063
						l1.21-3.943c0.202-0.591-0.807-2.366-1.413-2.563c-4.035-0.986-7.869-2.563-11.904-2.76c-4.237,0-8.474-0.395-12.509,0
						c-5.448,0.591-10.896,1.972-15.939,4.337c-2.623,1.183-4.439,3.155-4.641,6.112c0,1.38-2.421,1.774-3.43,0.986
						c-1.816-1.577-3.43-3.352-4.641-5.521c-0.404-0.789-0.605-1.971-0.404-2.76c0.605-2.366,1.614-4.338,3.229-6.31
						c4.842-5.323,10.896-8.28,17.755-10.055c6.658-1.774,13.317-2.958,20.378-2.366c1.816,0.197,3.43,0.395,5.246,0.789
						l12.106,2.366c0.605,0.197,2.22-0.592,2.421-1.38c1.816-4.14,3.43-8.083,5.044-12.224c3.229-7.886,7.062-15.378,11.097-22.871
						c5.246-10.449,11.097-20.307,17.755-29.771c3.43-4.731,7.264-8.872,10.896-13.407c2.22-2.76,5.246-4.535,8.272-6.112
						c2.825-1.577,6.053-1.38,9.08-0.395c3.632,1.183,6.658,3.746,8.071,7.295c1.614,4.141,2.623,8.478,1.412,13.013
						c-0.403,1.183-0.605,2.563-0.807,3.943c-1.21,12.815-4.641,25.04-8.07,37.46c-3.229,11.435-6.658,22.673-10.896,33.911
						c-0.403,1.183-0.605,2.563-1.009,3.746c-0.202,0.789,0.605,2.366,1.412,2.563c0.807,0.395,1.816,0.986,2.825,0.986
						c5.852,0.592,11.703,0.592,17.554,1.774c1.009,0.197,2.018,0.197,3.027,0c13.518-0.986,26.835-2.366,39.344-7.886
						c4.439-1.972,8.676-4.14,13.115-6.309c4.641-2.169,8.676-5.521,12.509-8.872c32.081-27.011,2.22-75.315-23.203-51.656
						c-0.605,1.38-2.421,0.986-3.632,0c-0.605-0.394-1.009-0.789-1.412-1.38c-1.009-1.38-2.018-2.76-2.825-4.141
						c21.589-27.602,78.689,7.886,55.083,63.485c-3.43,6.703-7.465,12.815-13.72,17.547c-4.237,2.958-8.07,6.309-12.509,9.069
						c-8.878,5.323-18.563,8.675-28.651,11.436c-0.202,0.197-0.605,0.394-1.009,0.394c-7.465,0.395-14.931,2.169-22.598,2.169
						c-1.614,0-3.43-0.197-5.044-0.395c-1.413-0.197-2.825-0.789-4.237-0.789c-10.29-0.197-20.378-2.958-30.467-5.323l-1.816-0.591
						c-0.807-0.197-2.421,0.591-2.825,1.38c-1.009,2.366-2.421,4.732-3.43,7.295c-7.264,17.153-16.141,33.517-25.019,50.078
						c-1.614,2.958-3.43,5.718-5.246,8.675c-1.21,1.972-2.018,4.14-2.018,6.703c-0.202,3.549-0.403,7.295-1.412,11.041
						c-0.404,1.577-0.404,3.352-0.404,4.929c-0.807,17.941-0.605,35.883,0,53.824c0,2.563,0.202,5.323,0.807,8.084
						c0.807,3.154,1.009,6.506,1.413,9.858c0.202,0.591,0.807,1.183,1.21,1.577c0.403,0.592,2.421,0,2.825-0.591
						c0.807-0.789,1.413-1.38,2.018-2.366l5.852-8.281c1.412-2.366,2.825-4.732,4.438-6.901c0.605-0.591,1.413-0.985,2.018-1.577
						c0.403-0.986,0.403-1.774,0.807-2.76c1.614-2.958,3.026-6.112,4.842-8.872c5.044-7.689,9.08-15.97,13.115-24.053
						c1.614-3.155,3.026-6.112,4.641-9.069c1.009-1.774,3.632-2.169,5.448-0.986c1.009,0.591,1.614,1.183,2.421,1.972
						c0.807,0.789,1.412,1.774,1.009,2.76c-3.229,7.492-6.255,14.984-9.685,22.279c-6.255,13.801-11.904,27.997-19.571,41.206
						c-0.807,1.577-1.21,2.958-2.623,3.943c-1.614,5.323-4.641,9.661-8.676,13.604c-2.825,2.563-6.053,3.154-9.685,1.972
						C205.358,549.22,201.928,547.643,198.901,545.277z M214.236,531.279c-0.202,0-0.403,0.197-0.403,0.197
						c-0.202,0-0.202,0.394-0.202,0.591L214.236,531.279z M215.446,400.563c3.43-4.337,5.246-9.463,7.869-14.195
						c3.833-7.492,7.062-15.181,10.492-22.871c0.807-1.774,0.404-2.76-1.009-3.352l-7.465-3.549
						c-1.614-0.592-3.833,0.394-4.237,1.972l-4.842,23.659c-0.807,4.338-1.614,8.478-2.018,11.041
						c-0.202,3.549-0.403,5.126-0.403,6.704c0,0.197,0.403,0.591,0.605,0.591C214.841,400.76,215.244,400.76,215.446,400.563z
						 M214.639,530.885c1.009-0.197,1.816-0.789,1.816-1.774C215.446,529.11,214.841,529.899,214.639,530.885L214.639,530.885z
						 M220.692,536.997v-3.746c-2.421,0.591-3.632,2.366-5.044,4.14c";

    let path_parser = svgtypes::PathParser::from(svg_string);
    let path_segments = path_parser.filter_map(Result::ok).into_iter();

    svg_curve::points_from_path_segments(path_segments)
}