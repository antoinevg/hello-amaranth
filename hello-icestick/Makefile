SHELL := /bin/zsh

ENVIRONMENT := source ../toolchain/oss-cad-suite/environment
PYTHON := python

blinky: blinky.py
	$(ENVIRONMENT) && $(PYTHON) $<

breathe: breathe.py
	$(ENVIRONMENT) && $(PYTHON) $<

pmod: pmod.py
	$(ENVIRONMENT) && $(PYTHON) $< generate -t v > $@.v

up_counter: up_counter.py
	$(PYTHON) $<

test:
	$(ENVIRONMENT) && $(PYTHON) -m amaranth_boards.icestick

clean:
	-rm -rf build/
	-rm *.v
	-rm	*.vcd
