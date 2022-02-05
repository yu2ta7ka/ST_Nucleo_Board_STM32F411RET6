#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use core::cell::{Cell, RefCell};
use core::ops::DerefMut;

use cortex_m;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;

use crate::hal::{
    gpio::gpioa::*,
    gpio::*,
    prelude::*,
    stm32,
    stm32::interrupt,
    timer::{Event, Timer},
};
use stm32f4xx_hal as hal;

// Refer to https://docs.rust-embedded.org/book/concurrency/
static LED_STATE: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static TIMER_TIM2: Mutex<RefCell<Option<Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));

#[interrupt]
fn TIM2() {
    free(|cs| {
        if let Some(ref mut tim2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            // Clears interrupt associated with event.
            tim2.clear_interrupt(Event::TimeOut);
        }
        let led_state = LED_STATE.borrow(cs);
        led_state.replace(!led_state.get());
    });
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    // Set up the LED
    let gpioa = dp.GPIOA.split();
    let mut led = Led::new(gpioa.pa5);

    // Set up the system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();

    // Set up the interrupt timer
    // Generates an interrupt at 1-second intervals.
    let mut timer = Timer::tim2(dp.TIM2, 1.hz(), clocks);
    timer.listen(Event::TimeOut);

    free(|cs| {
        TIMER_TIM2.borrow(cs).replace(Some(timer));
    });

    // Enable interrupt
    stm32::NVIC::unpend(stm32::Interrupt::TIM2);
    unsafe {
        stm32::NVIC::unmask(stm32::Interrupt::TIM2);
    }

    loop {
        if free(|cs| LED_STATE.borrow(cs).get()) {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}

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
