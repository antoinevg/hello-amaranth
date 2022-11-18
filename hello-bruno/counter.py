# - module: Counter -----------------------------------------------------------

from amaranth import *
from amaranth.build import Pins, Resource, Subsignal

class Counter(Elaboratable):
    def __init__(self):
        self.counter = Signal(8)

        self.leds = Cat(platform.request("led", i) for i in range(8))
        self.pmod0 = platform.request("pmod", 0)
        self.pmod1 = platform.request("pmod", 1)

    def elaborate(self, platform):
        m = Module()
        m.submodules.car = platform.clock_domain_generator()

        m.d.sync += self.counter.eq(self.counter + 1)
        m.d.sync += [
            self.leds.eq(self.counter),
            self.pmod0.eq(self.counter),
            self.pmod1.eq(self.counter),
            #self.pmod1[0:7].eq(self.counter[0:7]),
            #self.pmod1[7].eq(self.counter[0]),
        ]

        return m


# - build: ulx3s --------------------------------------------------------------

from amaranth_boards.ulx3s import *
from pll import ULX3SDomainGenerator

class FastestPlatform(ULX3S_85F_Platform):
    name                   = "ULX3S (85F)"
    clock_domain_generator = ULX3SDomainGenerator

#platform = ULX3S_85F_Platform()
platform = FastestPlatform()

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

platform.build(Counter(), do_program=True)
