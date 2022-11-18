SHELL := /bin/zsh

UART := /dev/cu.usbserial-D00137

ENVIRONMENT := source ../toolchain/oss-cad-suite/environment
PYTHON := python

blinky: blinky.py
	$(ENVIRONMENT) && $(PYTHON) $<

top: top.py
	$(ENVIRONMENT) && $(PYTHON) $<

step3: step3.py
	$(ENVIRONMENT) && $(PYTHON) $<

step2: step2.py
	$(ENVIRONMENT) && $(PYTHON) $<

picocom:
	picocom --imap lfcrlf -b 115200 $(UART)

counter: counter.py
	$(ENVIRONMENT) && $(PYTHON) $<

test:
	$(ENVIRONMENT) && $(PYTHON) -m amaranth_boards.ulx3s 85F

clean:
	-rm -rf build/
	-rm -f *.v
	-rm	-f *.vcd
	-rm	-f *.gtkw