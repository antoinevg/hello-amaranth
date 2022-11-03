## Dependencies

See [`../README.md`](../README.md) for basic repo setup

### FemtoRV

    cd ../toolchain
    git clone https://github.com/BrunoLevy/learn-fpga.git learn-fpga.git

More info: [https://github.com/BrunoLevy/learn-fpga/blob/master/FemtoRV/TUTORIALS/IceStick.md](https://github.com/BrunoLevy/learn-fpga/blob/master/FemtoRV/TUTORIALS/IceStick.md)


### Rust

    rustup target add riscv32i-unknown-none-elf
    cargo install cargo-binutils
    rustup component add llvm-tools-preview

<!--
For nightly:

    rustup target add riscv32i-unknown-none-elf --toolchain nightly
    cargo +nightly install cargo-binutils
    rustup component add llvm-tools-preview --toolchain nightly
-->

## Load FemtoRV bit stream

    source ../toolchain/oss-cad-suite/environment
    cd ../toolchain/learn-fpga.git/FemtoRV
    make ICESTICK

## Run blinky

    cargo build --release --example blinky
    cargo objcopy --release --example blinky -- -Obinary blinky.bin
    ../toolchain/oss-cad-suite/bin/iceprog -o 128k blinky.bin

### Terminal

    picocom -b 115200 /dev/cu.usbserial-224401 --imap lfcrlf,crcrlf --omap delbs,crlf --send-cmd "ascii-xfr -s -l 30 -n"
