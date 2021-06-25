#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{gpio::gpioa::*, gpio::gpioc::*, gpio::*, prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let mut led_pa5 = LedPa5::new(gpioa.pa5); // use implemented LED
        let mut led_pa6 = LedPa6::new(gpioa.pa6); // use wired LED

        let gpioc = dp.GPIOC.split();
        let button1 = Button1::new(gpioc.pc13);

        loop {
            if button1.is_pressed() {
                led_pa5.turn_on();
                led_pa6.turn_off();
            } else {
                led_pa5.turn_off();
                led_pa6.turn_on();
            }
        }
    }

    loop {}
}

// Button1 driver
struct Button1 {
    pin: PC13<Input<Floating>>,
}

impl Button1 {
    fn new(pin: PC13<Input<Floating>>) -> Button1 {
        Button1 {
            pin: pin.into_floating_input(),
        }
    }
    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
    fn is_released(&self) -> bool {
        self.pin.is_high().unwrap()
    }
}

struct LedPa5 {
    pin: PA5<Output<PushPull>>,
}

impl LedPa5 {
    fn new(pin: PA5<Input<Floating>>) -> LedPa5 {
        LedPa5 {
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

struct LedPa6 {
    pin: PA6<Output<PushPull>>,
}

impl LedPa6 {
    fn new(pin: PA6<Input<Floating>>) -> LedPa6 {
        LedPa6 {
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
