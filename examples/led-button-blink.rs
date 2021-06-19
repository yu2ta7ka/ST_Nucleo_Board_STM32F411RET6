#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*,stm32, gpio::*,gpio::gpioa::* ,gpio::gpioc::*};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        //let mut led = gpioa.pa5.into_push_pull_output();
        let mut led = Led::new(gpioa.pa5);

        let gpioc = dp.GPIOC.split();
        //let button1 = gpioc.pc13.into_floating_input();
        let button1 = Button1::new(gpioc.pc13);

        loop {
            //if button1.is_low().unwrap() {
            if button1.is_released() {
                led.turn_on();
            } else {
                led.turn_off();
            }
        }
    }

    loop {}
}

// Button1 driver
struct Button1{
    pin: PC13<Input<Floating>>,
}

impl Button1 {
    fn new(pin: PC13<Input<Floating>> ) -> Button1{
        Button1 {
            pin: pin.into_floating_input(),
        }
    }
    fn is_pressed(&self) -> bool{
        self.pin.is_low().unwrap()
    }
    fn is_released(&self) -> bool{
        self.pin.is_high().unwrap()
    }
}

struct Led{
    pin:PA5<Output<PushPull>>, 
}

impl Led{
    fn new(pin: PA5<Input<Floating>>) -> Led{
        Led {
            pin: pin.into_push_pull_output(),
        }
    }

    fn turn_on(&mut self){
        self.pin.set_high().unwrap();
    }

    fn turn_off(&mut self){
        self.pin.set_low().unwrap();
    }

    fn toggle(&mut self){
        self.pin.toggle().unwrap();
    }
}