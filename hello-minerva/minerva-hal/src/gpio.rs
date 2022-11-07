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
macro_rules! gpio {
    ($(
        $GPIOX:ident: $PACGPIOX:ty,
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $GPIOX {
                pub index: usize,
            }

            impl $GPIOX {
                pub fn new(index: usize) -> Self {
                    Self { index }
                }
            }

            impl $crate::hal::digital::v2::OutputPin for $GPIOX {
                type Error = core::convert::Infallible;

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = !(1 << self.index);
                    riscv::interrupt::free(|_cs| {
                        let val: u32 = reg.out.read().bits() & mask;
                        unsafe {
                            reg.out.write(|w| w.bits(val));
                        }
                    });
                    Ok(())
                }
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    riscv::interrupt::free(|_cs| {
                        let val: u32 = reg.out.read().bits() | mask;
                        unsafe {
                            reg.out.write(|w| w.bits(val));
                        }
                    });
                    Ok(())
                }
            }

            impl $crate::hal::digital::v2::StatefulOutputPin for $GPIOX {
                fn is_set_low(&self) -> Result<bool, Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    let val: u32 = reg.out.read().bits() & mask;
                    Ok(val == 0)
                }
                fn is_set_high(&self) -> Result<bool, Self::Error> {
                    let reg = unsafe { &*<$PACGPIOX>::ptr() };
                    let mask: u32 = 1 << self.index;
                    let val: u32 = reg.out.read().bits() & mask;
                    Ok(val != 0)
                }
            }

            /// Opt-in to the software implementation.
            impl $crate::hal::digital::v2::toggleable::Default for $GPIOX {}
        )+
    }
}
