#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{
    adc::{config::AdcConfig, Adc},
    gpio::*,
    prelude::*,
    serial::config::Config,
    serial::Serial,
    stm32,
};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // init GPIO object
    let gpioa = dp.GPIOA.split();
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

    let text = b"hello world\n";
    for c in text.iter() {
        nb::block!(tx.write(*c)).unwrap();
    }

    // Enable ADC
    let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());
    // Configure ADC pin
    let mut pa5 = gpioa.pa5.into_analog();

    let mut count = 0;
    loop {
        let value = match adc.read(&mut pa5) {
            Ok(x) => x,
            Err(_) => 0,
        };
        writeln!(tx, "count:{} value: {}\r", count, value).unwrap();
        count += 1;
        delay.delay_ms(2000_u32);
    }
}
