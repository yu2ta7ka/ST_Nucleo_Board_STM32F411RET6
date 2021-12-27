#![no_main]
#![no_std]

extern crate panic_halt; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::rectangle::Rectangle;
use embedded_graphics::primitives::Circle;
use embedded_graphics::primitives::Line;
use embedded_graphics::primitives::Triangle;
use embedded_graphics::style::PrimitiveStyle;
use embedded_graphics::style::PrimitiveStyleBuilder;
use hal::{delay::Delay, gpio::*, prelude::*, spi::*, stm32};
use st7735_lcd;
use st7735_lcd::Orientation;
use stm32f4xx_hal as hal;

// refer to
//- https://github.com/melodyaheath/stm32f411re-embedded-rust-ST7735-lcd
#[entry]
fn main() -> ! {
    if let (Some(board_peripherals), Some(processor_peripherals)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // The below two lines just setup the system clock and the peripheral clocks.
        // This shouldn't matter for the LCD.
        let reset_and_clock_control = board_peripherals.RCC;
        let clocks = reset_and_clock_control
            .constrain()
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze();
        //??? What are sysclk and pclk1?

        // Get the general purpose registers ready to use.
        let gpioa = board_peripherals.GPIOA.split();
        let gpiob = board_peripherals.GPIOB.split();
        let gpioc = board_peripherals.GPIOC.split();

        // for SPI communication
        // PA5 connects to SCL/SCK on the LCD
        let sck = gpioa.pa5.into_alternate_af5();
        // PA6 does not get connected to the LCD
        let miso = gpioa.pa6.into_alternate_af5();
        // PA7 connects to SDA/MOSI on the LCD
        let mosi = gpioa.pa7.into_alternate_af5();
        // GND connects to CS. Therefore, no code is reuired.

        // PC0 connects to RST/RES on the LCD
        let rst = gpioc.pc0.into_push_pull_output();
        // PB0 connects to RS/DC on the LCD
        let dc = gpiob.pb0.into_push_pull_output();

        /* Notice this board is communicating over SPI_1. If it was some other SPI,
        the pins would be different depending on the alternate functions. The alternate
        function group could also end up being some number other than 5. */
        let spi = Spi::spi1(
            board_peripherals.SPI1,
            (sck, miso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            16000000.hz(),
            clocks,
        );

        /* Remember the change the width and height to match your LCD screen.
        The RGB parameter specifies whether the LCD screen uses RGB or BGR for
        color. Your LCD might vary so if you find your blues are reds or vice
        versa change this parameter. */
        //let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, false, false, 128, 128);
        let mut disp = st7735_lcd::ST7735::new(spi, dc, rst, true, false, 160, 128);

        // This gives the display a source to use to control pin timings.
        let mut delay = Delay::new(processor_peripherals.SYST, clocks);

        // Initialize the display.
        disp.init(&mut delay).unwrap();
        // Set the orientation of the display
        disp.set_orientation(&Orientation::Landscape).unwrap();

        /* Create a style that specifies a color of RED. This will always use
        Rgb565 regardless if your board uses RGB or BGR. */
        //let style = PrimitiveStyleBuilder::new().fill_color(Rgb565::RED).build();
        let style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLUE)
            .build();

        /* Create a rectangle to fill the background. Make sure the second point
        has a width and height that matches your ST7735. */
        let red_backdrop =
            Rectangle::new(Point::new(0, 0), Point::new(160, 128)).into_styled(style);
        red_backdrop.draw(&mut disp).unwrap();

        // draw green line.
        let start = Point::new(50, 20);
        let end = Point::new(110, 20);
        let style = PrimitiveStyle::with_stroke(Rgb565::GREEN, 1);
        Line::new(start, end)
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        // draw red circle
        Circle::new(Point::new(50, 50), 20)
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5))
            .draw(&mut disp)
            .unwrap();

        // draw Triangle filled with blue
        Triangle::new(Point::new(10, 20), Point::new(10, 45), Point::new(30, 15))
            .into_styled(PrimitiveStyle::with_fill(Rgb565::YELLOW))
            .draw(&mut disp)
            .unwrap();

        // draw rectangle
        let style = PrimitiveStyleBuilder::new()
            .stroke_width(5)
            .stroke_color(Rgb565::CYAN)
            .fill_color(Rgb565::YELLOW)
            .build();
        Rectangle::new(Point::new(70, 80), Point::new(100, 110))
            .into_styled(style)
            .draw(&mut disp)
            .unwrap();

        loop {
            continue;
        }
    }

    loop {}
}
