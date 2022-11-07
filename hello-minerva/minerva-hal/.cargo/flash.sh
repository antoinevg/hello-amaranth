#!/usr/bin/env zsh

set -ex

NAME=$(basename $1)

# Create bin file
cargo objcopy --release --example $NAME -- -Obinary $1.bin

echo "Firmware file: $1.bin"

# Program vexriscv
litex_term --kernel $1.bin /dev/cu.usbserial-D00137
