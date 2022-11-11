# - module: Pmod --------------------------------------------------------------

from amaranth import Elaboratable, Module, Signal
from amaranth.build import Attrs, Pins, Platform, Resource, Subsignal

class Pmod(Elaboratable):
    def __init__(self):
        pass

    def elaborate(self, platform: Platform) -> Module:
        m = Module()

        platform.add_resources([
            Resource("pmod", 0,
                Subsignal("gp3", Pins("3+", conn=("gpio", 0), dir="o")),
                Subsignal("gn3", Pins("3-", conn=("gpio", 0), dir="o")),
                Subsignal("gp4", Pins("4+", conn=("gpio", 0), dir="o")),
                Subsignal("gn4", Pins("4-", conn=("gpio", 0), dir="o")),
                Subsignal("gp5", Pins("5+", conn=("gpio", 0), dir="o")),
                Subsignal("gn5", Pins("5-", conn=("gpio", 0), dir="o")),
                Subsignal("gp6", Pins("6+", conn=("gpio", 0), dir="o")),
                Subsignal("gn6", Pins("6-", conn=("gpio", 0), dir="o")),
            ),
            Resource("pmod", 1,
                Subsignal("gp10", Pins("10+", conn=("gpio", 0), dir="o")),
                Subsignal("gn10", Pins("10-", conn=("gpio", 0), dir="o")),
                Subsignal("gp11", Pins("11+", conn=("gpio", 0), dir="o")),
                Subsignal("gn11", Pins("11-", conn=("gpio", 0), dir="o")),
                Subsignal("gp12", Pins("12+", conn=("gpio", 0), dir="o")),
                Subsignal("gn12", Pins("12-", conn=("gpio", 0), dir="o")),
                Subsignal("gp13", Pins("13+", conn=("gpio", 0), dir="o")),
                Subsignal("gn13", Pins("13-", conn=("gpio", 0), dir="o")),
            )
        ])

        led0 = platform.request("led", 0)
        pmod = platform.request("pmod", 1)
        led5 = pmod.gp12
        led6 = pmod.gp13

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

from amaranth_boards.ulx3s import *

if __name__ == "__main__":
    ULX3S_85F_Platform().build(Pmod(), do_program=True)
