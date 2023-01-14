SHELL := /bin/zsh

UART := /dev/cu.usbserial-D00137

ENVIRONMENT := source ../toolchain/oss-cad-suite/environment
PYTHON := python

AS := riscv64-unknown-elf-as
LD := riscv64-unknown-elf-ld
OBJDUMP := riscv64-unknown-elf-objdump

blinky: blinky.py
	$(ENVIRONMENT) && $(PYTHON) $<

top: top.py
	$(ENVIRONMENT) && $(PYTHON) $<

step2: step2.py
	$(ENVIRONMENT) && $(PYTHON) $<

step3: step3.py
	$(ENVIRONMENT) && $(PYTHON) $<

step3_pmod: step3_pmod.py
	$(ENVIRONMENT) && $(PYTHON) $<

step4: step4.py
	$(ENVIRONMENT) && $(PYTHON) $<

step5: step5.py
	$(ENVIRONMENT) && $(PYTHON) $<

step6: step6.py
	$(ENVIRONMENT) && $(PYTHON) $<

step7: step7.py
	$(ENVIRONMENT) && $(PYTHON) $<

step8: step8.py
	$(ENVIRONMENT) && $(PYTHON) $<

picocom:
	picocom --imap lfcrlf -b 115200 $(UART)

counter: counter.py
	$(ENVIRONMENT) && $(PYTHON) $<

test:
	$(ENVIRONMENT) && $(PYTHON) -m amaranth_boards.ulx3s 85F

gas:
	$(ENVIRONMENT) && $(AS) -march=rv32i -mno-arch-attr step8.s -o step8.o
	$(ENVIRONMENT) && $(LD) -m elf32lriscv -Ttext=0x0000 step8.o -o step8.elf
	$(ENVIRONMENT) && $(OBJDUMP) -D step8.elf

clean:
	-rm -rf build/
	-rm -f *.v
	-rm	-f *.vcd
	-rm	-f *.gtkw
	-rm	-f *.elf *.o
