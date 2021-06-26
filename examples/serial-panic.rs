#![no_main]
#![no_std]

use stm32f4xx_hal::stm32::USART2;
//use panic_halt as _;
use core::panic::PanicInfo;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{gpio::*, prelude::*, serial::config::Config, serial::Serial, stm32};

use core::fmt::Write; // for pretty formatting of the serial output

//static mut USART: Option<stm32f4xx_hal::serial::Serial<stm32f4xx_hal::stm32::USART2, (stm32f4xx_hal::gpio::gpioa::PA2<stm32f4xx_hal::gpio::Alternate<stm32f4xx_hal::gpio::AF7>>, stm32f4xx_hal::gpio::gpioa::PA3<stm32f4xx_hal::gpio::Alternate<stm32f4xx_hal::gpio::AF7>>)>>=None;
static mut TX: Option<stm32f4xx_hal::serial::Tx<USART2>> = None;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    // init GPIO object
    let gpioa = dp.GPIOA.split();
    // init clock object
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();
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
    let (tx, _rx) = serial.split();

    unsafe {
        TX = Some(tx);
        writeln!(TX.as_mut().unwrap(), "hello {}", "world").unwrap();
        writeln!(TX.as_mut().unwrap(), "panic test: none.unwrap()",).unwrap();
    }

    let none: Option<usize> = None;
    none.unwrap();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        writeln!(TX.as_mut().unwrap(), "panic : {}", info).ok();

        loop {}
    }
}
