SHELL := /bin/zsh

UART := /dev/cu.usbserial-D00137

ENVIRONMENT := source ../toolchain/oss-cad-suite/environment
PYTHON := python

blinky: blinky.py
	$(ENVIRONMENT) && $(PYTHON) $<

picocom:
	picocom --imap lfcrlf -b 115200 $(UART)

test:
	$(ENVIRONMENT) && $(PYTHON) -m amaranth_boards.ulx3s 85F

clean:
	-rm -rf build/
	-rm *.v
	-rm	*.vcd
