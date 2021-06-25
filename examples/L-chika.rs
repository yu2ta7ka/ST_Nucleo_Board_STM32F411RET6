#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{gpio::gpioa::*, gpio::*, prelude::*, stm32};

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

    loop {
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
