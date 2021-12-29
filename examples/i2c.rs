#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{gpio::*, i2c::I2c, prelude::*, serial::config::Config, serial::Serial, stm32};

use core::fmt::Write; // for pretty formatting of the serial output
use vl53l0x::VL53L0x;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // init GPIO object
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    // init clock object
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();
    // init delay object
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    //set up I2C
    let scl = gpiob.pb6.into_alternate_af4_open_drain();
    let sda = gpiob.pb7.into_alternate_af4_open_drain();
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks);
    let mut tof = VL53L0x::new(i2c).unwrap();

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

    writeln!(&mut tx, "this is {} example!", "ToF").unwrap();

    loop {
        let dist = VL53L0x::read_range_single_millimeters_blocking(&mut tof).unwrap();
        writeln!(tx, "distance: {} mm\r", dist).unwrap();
        delay.delay_ms(1000_u16);
    }
}
