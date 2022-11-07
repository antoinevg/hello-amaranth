# - module: Pmod --------------------------------------------------------------

from amaranth import Elaboratable, Module, Signal
from amaranth.build import Attrs, Pins, Platform, Resource, Subsignal

class Pmod(Elaboratable):
    def __init__(self):
        pass

    def elaborate(self, platform: Platform) -> Module:
        m = Module()

        platform.add_resources([
            Resource("pmodtest", 0,
                Subsignal("l1", Pins("1",  conn=("pmod", 0), dir="o")),
                Subsignal("l2", Pins("2",  conn=("pmod", 0), dir="o")),
                Subsignal("l3", Pins("3",  conn=("pmod", 0), dir="o")),
                Subsignal("l4", Pins("4",  conn=("pmod", 0), dir="o")),
                Subsignal("r1", Pins("7",  conn=("pmod", 0), dir="o")),
                Subsignal("r2", Pins("8",  conn=("pmod", 0), dir="o")),
                Subsignal("r3", Pins("9",  conn=("pmod", 0), dir="o")),
                Subsignal("r4", Pins("10", conn=("pmod", 0), dir="o")),
                Attrs(IO_STANDARD="SB_LVCMOS")
            )
        ])

        led0 = platform.request("led", 0)
        pmod = platform.request("pmodtest", 0)
        led5 = pmod.l1
        led6 = pmod.r2

        half_freq = int(platform.default_clk_frequency // 2)
        timer = Signal(range(half_freq), reset=half_freq - 1)

        with m.Switch(timer):
            with m.Case(half_freq):
                m.d.sync += led0.eq(~led0)
                m.d.sync += timer.eq(0)
            with m.Case(int(half_freq // 2)):
                m.d.sync += led5.eq(~led5)
                m.d.sync += timer.eq(timer + 1)
            with m.Case(int(half_freq // 4)):
                m.d.sync += led6.eq(~led6)
                m.d.sync += timer.eq(timer + 1)
            with m.Default():
                m.d.sync += timer.eq(timer + 1)

        return m

# - build: icestick -----------------------------------------------------------

from amaranth_boards.icestick import *

ICEStickPlatform().build(Pmod(), do_program=True)
