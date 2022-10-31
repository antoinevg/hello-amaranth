SHELL := /bin/zsh

up_counter: up_counter.py
	python up_counter.py

blinky_icestick: blinky_icestick.py
	source ../toolchain/oss-cad-suite/environment && \
	python blinky_icestick.py

test_icestick:
	source ../toolchain/oss-cad-suite/environment && \
	python -m amaranth_boards.icestick

clean:
	rm *.v *.vcd
