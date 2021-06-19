#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, serial::config::Config, serial::Serial, stm32};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();

    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();

    // define RX/TX pins
    let tx_pin = gpioa.pa2.into_alternate_af7();
    let rx_pin = gpioa.pa3.into_alternate_af7();

    // configure serial
    let mut serial = Serial::usart2(
        dp.USART2,
        (tx_pin, rx_pin),
        Config::default().baudrate(9600.bps()),
        clocks,
    )
    .unwrap();

    loop {
        // Use nb crate (https://crates.io/crates/nb)
        // Minimal and reusable non-blocking I/O layer
        // Waiting for a word
        if let Ok(c) = nb::block!(serial.read()){
            // Send the received data
            nb::block!(serial.write(c)).unwrap();
        }
    }
}
