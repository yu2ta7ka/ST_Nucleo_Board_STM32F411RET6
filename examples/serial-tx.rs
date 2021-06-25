#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{
    gpio::gpioa::*, gpio::*, prelude::*, serial::config::Config, serial::Serial, stm32,
};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // init LED object
    let gpioa = dp.GPIOA.split();
    let mut led = Led::new(gpioa.pa5);

    // init clock object
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();
    // init delay object
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
    // define RX/TX pins
    let tx_pin = gpioa.pa2.into_alternate_af7();
    let rx_pin = gpioa.pa3.into_alternate_af7();
    // configure serial
    let serial = Serial::usart2(
        dp.USART2,
        (tx_pin, rx_pin),
        Config::default().baudrate(9600.bps()),
        clocks,
    )
    .unwrap();
    let (mut tx, mut _rx) = serial.split();

    let mut value: u8 = 0;
    loop {
        // print some value every 500 ms, value will overflow after 255
        writeln!(tx, "value: {:02}\r", value).unwrap();
        value += 1;
        delay.delay_ms(500_u32);
        led.toggle();
    }
}

// LED driver
struct Led {
    pin: PA5<Output<PushPull>>,
}

impl Led {
    fn new(pin: PA5<Input<Floating>>) -> Led {
        Led {
            pin: pin.into_push_pull_output(),
        }
    }

    fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }

    fn toggle(&mut self) {
        self.pin.toggle().unwrap();
    }
}
