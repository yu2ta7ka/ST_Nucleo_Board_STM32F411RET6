#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_rt::entry;

use crate::hal::{
    gpio::*,
    prelude::*,
    serial::config::Config,
    serial::Serial,
    stm32,
    stm32::interrupt,
    timer::{Event, Timer},
};
use core::fmt::Write;
use stm32f4xx_hal as hal; // for pretty formatting of the serial output

static TIMER_TIM2: Mutex<RefCell<Option<Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));
static mut TIMER_COUNTER: u32 = 0;

#[interrupt]
fn TIM2() {
    free(|cs| {
        if let Some(ref mut tim2) = TIMER_TIM2.borrow(cs).borrow_mut().deref_mut() {
            // Clears interrupt associated with event.
            tim2.clear_interrupt(Event::TimeOut);
        }
        unsafe {
            TIMER_COUNTER += 1;
        }
    });
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    // Set up the system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();

    // Set up the interrupt timer
    // Generates an interrupt at 1 milli second intervals.
    let mut timer = Timer::tim2(dp.TIM2, 1000.hz(), clocks);
    timer.listen(Event::TimeOut);

    free(|cs| {
        TIMER_TIM2.borrow(cs).replace(Some(timer));
    });

    // Enable interrupt
    stm32::NVIC::unpend(stm32::Interrupt::TIM2);
    unsafe {
        stm32::NVIC::unmask(stm32::Interrupt::TIM2);
    }

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

    let mut _x = 0;
    loop {
        // dummy process
        for _ in 0..10000 {
            _x += 1;
        }
        unsafe {
            writeln!(tx, "timer count: {}ms \r", TIMER_COUNTER).unwrap();
        }
    }
}
