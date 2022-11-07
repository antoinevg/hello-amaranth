#![no_std]
#![no_main]

use core::fmt::Write;
use minerva_hal::prelude::*;
use minerva_pac::{Peripherals};
use riscv_rt::entry;
use panic_halt as _;

minerva_hal::uart! {
    Uart: minerva_pac::UART,
}

minerva_hal::timer! {
    Timer: minerva_pac::TIMER0,
}

const SYSTEM_CLOCK_FREQUENCY: u32 = 50_000_000;

#[entry]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };
    let mut uart = Uart::new(peripherals.UART);
    writeln!(uart, "Peripherals initialized").unwrap();

    let mut timer = Timer::new(peripherals.TIMER0, SYSTEM_CLOCK_FREQUENCY);
    let mut uptime = 0;
    loop {
        timer.delay_ms(1000_u32);
        uptime += 1;
        writeln!(uart, "Uptime: {} seconds", uptime).unwrap();
    }
}
