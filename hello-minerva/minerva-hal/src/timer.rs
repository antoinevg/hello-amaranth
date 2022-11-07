// Copyright (c) 2022 Pepijn de Vos
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#[macro_export]
macro_rules! timer {
    ($(
        $TIMERX:ident: $PACTIMERX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $TIMERX {
                registers: $PACTIMERX,
                pub sys_clk: u32,
            }

            impl $TIMERX {
                pub fn new(registers: $PACTIMERX, sys_clk: u32) -> Self {
                    Self { registers, sys_clk }
                }

                pub fn free(self) -> $PACTIMERX {
                    self.registers
                }
            }

            impl<UXX: core::convert::Into<u32>> $crate::hal::blocking::delay::DelayMs<UXX> for $TIMERX {
                fn delay_ms(&mut self, ms: UXX) -> () {
                    let value: u32 = self.sys_clk / 1_000 * ms.into();
                    unsafe {
                        self.registers.en.write(|w| w.bits(0));
                        self.registers.reload.write(|w| w.bits(0));
                        self.registers.load.write(|w| w.bits(value));
                        self.registers.en.write(|w| w.bits(1));
                        self.registers.update_value.write(|w| w.bits(1));
                        while self.registers.value.read().bits() > 0 {
                            self.registers.update_value.write(|w| w.bits(1));
                        }
                    }
                }
            }
        )+
    }
}
