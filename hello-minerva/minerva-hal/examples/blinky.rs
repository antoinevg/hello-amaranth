#![no_std]
#![no_main]

use panic_halt as _;

use minerva_hal as hal;
use minerva_pac as pac;

use hal::prelude::*;
use embedded_hal::digital::v2::ToggleableOutputPin;

use riscv_rt::entry;

hal::timer! {
    Timer: minerva_pac::TIMER0,
}

hal::gpio! {
    LEDS: pac::LEDS,
}

const SYSTEM_CLOCK_FREQUENCY: u32 = 50_000_000;

#[entry]
fn main() -> ! {
    let peripherals = unsafe { pac::Peripherals::steal() };

    let mut led = LEDS { index: 0 };
    let mut timer = Timer::new(peripherals.TIMER0, SYSTEM_CLOCK_FREQUENCY);

    loop {
        led.toggle().unwrap();
        timer.delay_ms(1000_u32);
    }
}
