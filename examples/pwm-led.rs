#![deny(unsafe_code)]
#![no_main]
#![no_std]

// refer
// https://github.com/stm32-rs/stm32f4xx-hal/blob/fcdb48096d7e2729d8cb4355befcc6f9af7355c6/examples/pwm.rs

// Halt on panic
use panic_halt as _;

use crate::hal::{prelude::*, pwm, stm32};
use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the system clock.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // Set up PWM
    let gpioa = dp.GPIOA.split();
    let channels = (
        gpioa.pa8.into_alternate_af1(),
        gpioa.pa9.into_alternate_af1(),
    );
    let pwm = pwm::tim1(dp.TIM1, channels, clocks, 20u32.khz());
    let (mut ch1, _ch2) = pwm;
    let max_duty = ch1.get_max_duty();
    ch1.set_duty(max_duty);
    ch1.enable();

    // init delay object
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    let mut ratio = 1;

    loop {
        ch1.set_duty(max_duty / ratio);
        delay.delay_ms(50_u32);
        ratio = match ratio {
            10 => 1,
            _ => {
                ratio += 1;
                ratio
            }
        };
    }
}
