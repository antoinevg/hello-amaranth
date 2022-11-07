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
macro_rules! spi {
    ($(
        $SPIX:ident: ($PACSPIX:ty, $WORD:ty),
    )+) => {
        $(
            #[derive(Debug)]
            pub struct $SPIX {
                registers: $PACSPIX,
            }

            impl $SPIX {
                pub fn new(registers: $PACSPIX) -> Self {
                    Self { registers }
                }

                pub fn free(self) -> $PACSPIX {
                    self.registers
                }
            }

            impl $crate::hal::spi::FullDuplex<$WORD> for $SPIX {
                type Error = core::convert::Infallible;

                fn read(&mut self) -> $crate::nb::Result<$WORD, Self::Error> {
                    if self.registers.status.read().done().bit() {
                        Ok(self.registers.miso.read().bits() as $WORD)
                    } else {
                        Err($crate::nb::Error::WouldBlock)
                    }
                }

                fn send(&mut self, word: u8) -> $crate::nb::Result<(), Self::Error> {
                    if self.registers.status.read().done().bit() {
                        unsafe {
                            self.registers.mosi.write(|w| w.bits(word.into()));
                            self.registers.control.write(|w| {
                                w.length().bits(8).start().bit(true)
                            });
                        }
                        Ok(())
                    } else {
                        Err($crate::nb::Error::WouldBlock)
                    }
                }
            }

            impl $crate::hal::blocking::spi::write::Default<u8> for $SPIX {}
            impl $crate::hal::blocking::spi::transfer::Default<u8> for $SPIX {}

            impl From<$PACSPIX> for $SPIX {
                fn from(registers: $PACSPIX) -> $SPIX {
                    $SPIX::new(registers)
                }
            }
        )+
    }
}
