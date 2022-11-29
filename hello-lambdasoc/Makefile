SHELL := /bin/zsh

UART := /dev/cu.usbserial-D00137

# ulx3s 85f v3.1.7
BOARD := "radiona_ulx3s"
DEVICE := "LFE5U-85F"
REVISION := "2.0"
SDRAM := "IS42S16160"

LITEX := ../toolchain/litex
LITEX_BOARDS := $(LITEX)/litex-boards/litex_boards
YOSYS := ../toolchain/oss-cad-suite/

ENVIRONMENT := source $(YOSYS)/environment

help:
	$(ENVIRONMENT) && python $(LITEX_BOARDS)/targets/$(BOARD).py --help

blinky:
	$(ENVIRONMENT) && python gateware/blinky.py

top:
	$(ENVIRONMENT) && AR=riscv64-unknown-elf-ar python gateware/top.py

program:
	$(ENVIRONMENT) && openFPGALoader --board ulx3s build/top.bit

flash_TODO:
	 echo -e "\nserialboot" | script -q /dev/null \
				"flterm --speed $(BAUDRATE) --kernel $< --kernel-addr $(shell ./$(SOC) --get-fw-address) $(SERIALPORT)"

flash:
	 lxterm --kernel ./blinky.bin --kernel-adr 0x00008000 --speed 115200 /dev/cu.usbserial-D00137


picocom:
	picocom --imap lfcrlf -b 115200 $(UART)

svd:
	rm -rf firmware/minerva-pac/src
	mkdir firmware/minerva-pac/src
	svd2rust -i firmware/minerva-pac/svd/minerva.svd -o build --target riscv
	mv build/lib.rs firmware/minerva-pac/src
	mv build/build.rs firmware/minerva-pac/
	mv build/device.x firmware/minerva-pac/
	form -i firmware/minerva-pac/src/lib.rs -o firmware/minerva-pac/src/
	cd firmware/minerva-pac && cargo fmt -- --emit files

clean:
	-rm -rf build/
