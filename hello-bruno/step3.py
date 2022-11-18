# See: https://github.com/BrunoLevy/learn-fpga/blob/master/FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV/README.md

from amaranth import *


# - module: Clockworks --------------------------------------------------------

class Clockworks(Elaboratable):
    def __init__(self, slow):
        self.slow = slow

        self.clk_out = Signal()

    def elaborate(self, platform):
        m = Module()

        # registers
        self.slow_clk = Signal(self.slow + 1)

        # sync
        m.d.sync += self.slow_clk.eq(self.slow_clk + 1)

        # comb
        m.d.comb += self.clk_out.eq(self.slow_clk[self.slow])

        return m

    def ports(self):
        return [self.clk_out]


# - module: SoC ---------------------------------------------------------------

class SoC(Elaboratable):
    def __init__(self):
        # make clk and rst signals explicit for didactic purposes
        self.clk_in = Signal()
        self.rst_in = Signal()

        self.leds_out = Signal(8)

        self.rxd_in = Signal()
        self.txd_out = Signal()

        instructions = [
            0b00000,
            0b00001,
            0b00010,
            0b00100,
            0b01000,
            0b10000,
            0b01000,
            0b00100,
            0b00010,
            0b00001,
            0b00000,
            0b00011,
            0b00111,
            0b01111,
            0b11111,
            0b11110,
            0b11100,
            0b11000,
            0b10000,
            0b00000,
            0b11111,
        ]
        self.mem = Array([Const(instruction) for instruction in instructions])

    def elaborate(self, platform):
        m = Module()

        # domain
        m.domains.clk_in = ClockDomain()
        m.d.comb += ClockSignal("clk_in").eq(self.clk_in)

        # registers
        mem_size = len(self.mem)
        self.pc = Signal(range(mem_size), reset=mem_size)

        # sync
        m.d.clk_in += self.pc.eq(Mux((self.rst_in) | (self.pc == mem_size), 0, self.pc + 1))
        m.d.clk_in += self.leds_out.eq(self.mem[self.pc])

        return m

    def ports(self):
        return [self.clk_in, self.rst_in, self.leds_out, self.rxd_in, self.txd_out]


# - module: Top ---------------------------------------------------------------

class Top(Elaboratable):
    def __init__(self, platform):
        if platform is not None:
            self.leds = Cat(platform.request("led", i) for i in range(8))
            self.button_pwr = platform.request("button_pwr", 0)
        else:
            self.leds = Signal(8)
            self.button_pwr = Signal()

        if platform is not None:
            self.clockworks = Clockworks(21)
        else: # run it faster for the simulator
            self.clockworks = Clockworks(3)
        self.soc = SoC()

    def elaborate(self, platform):
        m = Module()

        # submodules
        m.submodules += [
            self.clockworks,
            self.soc
        ]

        # connectors
        m.d.comb += [
            self.soc.clk_in.eq(self.clockworks.clk_out),
            self.soc.rst_in.eq(self.button_pwr),
            self.leds.eq(self.soc.leds_out)
        ]

        return m

    def ports(self):
        return self.soc.ports() + self.clockworks.ports()


# - simulate ------------------------------------------------------------------

# see: https://github.com/RobertBaruch/amaranth-tutorial/blob/main/7_simulating.md

from amaranth.sim import Simulator

dut = Top(platform=None)

sim = Simulator(dut)

def bench():
    yield

sim.add_clock(1e-6, domain="sync")
sim.add_sync_process(bench, domain="sync")

with sim.write_vcd("soc.vcd", "soc.gtkw", traces=dut.ports()):
    sim.run_until(100e-6, run_passive=True)


# - generate: verilog ---------------------------------------------------------

from amaranth.back import verilog

top = Top(platform=None)
with open("top.v", "w") as f:
    f.write(verilog.convert(top, ports=top.ports()))

soc = SoC()
with open("soc.v", "w") as f:
    f.write(verilog.convert(soc, ports=soc.ports()))

clockworks = Clockworks(21)
with open("clockworks.v", "w") as f:
    f.write(verilog.convert(clockworks, ports=clockworks.ports()))


#import sys
#sys.exit(0)


# - build: ulx3s --------------------------------------------------------------

from amaranth_boards.ulx3s import *

platform = ULX3S_85F_Platform()
top = Top(platform)

#platform.build(top, do_program=False)
platform.build(top, do_program=True)
