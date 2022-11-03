UART := /dev/cu.usbserial-224401
ICEPROG := ../toolchain/oss-cad-suite/bin/iceprog

blinky:
	cargo build --release --example blinky
	cargo objcopy --release --example blinky -- -Obinary blinky.bin
	$(ICEPROG) -o 128k blinky.bin

terminal:
	picocom -b 115200 $(UART) --imap lfcrlf,crcrlf --omap delbs,crlf --send-cmd "ascii-xfr -s -l 30 -n"

fix-build:
	rm -rf Cargo.lock target; touch build.rs
