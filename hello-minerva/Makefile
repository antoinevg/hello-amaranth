SHELL := /bin/zsh

UART := /dev/cu.usbserial-D00137

# ulx3s 85f v3.1.7
BOARD := "radiona_ulx3s"
DEVICE := "LFE5U-85F"
REVISION := "2.0"
SDRAM := "IS42S16160"

PYTHON := python
LITEX := ../toolchain/litex
LITEX_BOARDS := $(LITEX)/litex-boards/litex_boards
YOSYS := ../toolchain/oss-cad-suite/

ENVIRONMENT := source $(YOSYS)/environment

help:
	$(ENVIRONMENT) && python $(LITEX_BOARDS)/targets/$(BOARD).py --help

synth:
	$(ENVIRONMENT) && python $(LITEX_BOARDS)/targets/$(BOARD).py \
	  --cpu-type minerva \
	  --cpu-variant standard \
	  --device $(DEVICE) \
	  --revision $(REVISION) \
	  --sdram-module $(SDRAM) \
	  --csr-svd minerva-pac/svd/minerva.svd \
	  --output-dir build/radiona_ulx3s/ \
	  --with-oled \
	  --build

svd:
	rm -rf minerva-pac/src
	mkdir minerva-pac/src
	svd2rust -i minerva-pac/svd/minerva.svd -o build --target riscv
	mv build/lib.rs minerva-pac/src
	mv build/build.rs minerva-pac/
	mv build/device.x minerva-pac/
	form -i minerva-pac/src/lib.rs -o minerva-pac/src/
	cd minerva-pac && cargo fmt -- --emit files

load:
	$(ENVIRONMENT) && openFPGALoader --board ulx3s build/radiona_ulx3s/gateware/radiona_ulx3s.bit

picocom:
	picocom --imap lfcrlf -b 115200 $(UART)

clean:
	-rm -rf build/
