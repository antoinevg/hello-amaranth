SHELL := /bin/zsh

ENVIRONMENT := source ../toolchain/oss-cad-suite/environment
PYTHON := python

blinky_icestick: blinky_icestick.py
	$(ENVIRONMENT) && $(PYTHON) $<

breathe_icestick: breathe_icestick.py
	$(ENVIRONMENT) && $(PYTHON) $<

pwm: pwm.py
	$(PYTHON) $< generate -t v > $@.v

up_counter: up_counter.py
	$(PYTHON) $<

test_icestick:
	$(ENVIRONMENT) && $(PYTHON) -m amaranth_boards.icestick

clean:
	-rm -rf build/
	-rm *.v
	-rm	*.vcd
