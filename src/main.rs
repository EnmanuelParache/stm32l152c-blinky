#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate stm32l1xx_hal as hal;

use core::panic::PanicInfo;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::ToggleableOutputPin;
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.freeze(Config::default());
    let mut delay = cp.SYST.delay(rcc.clocks);

    let gpiob = dp.GPIOB.split();
    let mut blue_led = gpiob.pb6.into_push_pull_output();
    let mut green_led = gpiob.pb7.into_push_pull_output();

    green_led.set_high().unwrap();
    blue_led.set_low().unwrap();

    loop {
        
        delay.delay(500.ms());

        green_led.toggle().unwrap();
        blue_led.toggle().unwrap();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}