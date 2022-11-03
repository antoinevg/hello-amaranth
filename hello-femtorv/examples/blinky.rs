#![allow(dead_code)]

#![no_std]
#![no_main]

use panic_halt as _;
use riscv_rt::entry;

const IO_BASE: usize = 0x00400000;
const IO_LEDS: usize = IO_BASE + 0x4;
const IO_UART_DAT: usize = IO_BASE + 0x8;
const IO_UART_CNTL: usize = IO_BASE + 0x16;

mod asm {
    /// Delay for the given amount of cycles
    #[inline]
    pub unsafe fn delay(cycles: u32) {
        match () {
            () => {
                let real_cyc = 1 + cycles / 2;
                core::arch::asm!(
                    "1:",
                    "addi {0}, {0}, -1",
                    "bne {0}, zero, 1b",
                    inout(reg) real_cyc => _,
                    options(nomem, nostack),
                )
            }
        }
    }
}

/// Output string to UART
fn print(string: &str) {
    for c in string.chars() {
        let mem = IO_UART_DAT as *mut u8;
        unsafe { *mem = c as u8 }
     }
}

#[entry]
fn main() -> ! {
    loop {
        let delay = 1_000_000;
        let io_leds = IO_LEDS as *mut _;

        unsafe {
            for led in [1, 2, 4, 8] {
                print("it's on\n");
                *io_leds = led;
                asm::delay(delay);

                print("now it's off\n");
                *io_leds = 0;
                asm::delay(delay);
            }
        }
    }
}
