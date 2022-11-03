# - module: Breathe -----------------------------------------------------------

from amaranth import Elaboratable, Module, Signal
from amaranth.build import Platform

class Breathe(Elaboratable):
    def __init__(self):
        pass

    def elaborate(self, platform: Platform) -> Module:
        m = Module()

        led0 = platform.request("led", 0)
        led1 = platform.request("led", 1)
        led2 = platform.request("led", 2)
        led3 = platform.request("led", 3)
        led4 = platform.request("led", 4)

        half_freq = int(platform.default_clk_frequency // 2)
        timer = Signal(range(half_freq), reset=half_freq - 1)
        with m.Switch(timer):
            with m.Case(half_freq):
                m.d.sync += led0.eq(~led0)
                m.d.sync += timer.eq(0)
            with m.Case(int(half_freq // 2)):
                m.d.sync += led1.eq(~led1)
                m.d.sync += timer.eq(timer + 1)
            with m.Case(int(half_freq // 4)):
                m.d.sync += led2.eq(~led2)
                m.d.sync += timer.eq(timer + 1)
            with m.Case(int(half_freq // 8)):
                m.d.sync += led3.eq(~led3)
                m.d.sync += timer.eq(timer + 1)
            with m.Case(int(half_freq // 16)):
                m.d.sync += led4.eq(~led4)
                m.d.sync += timer.eq(timer + 1)
            with m.Default():
                m.d.sync += timer.eq(timer + 1)

        return m

# - build: icestick -----------------------------------------------------------

from amaranth_boards.icestick import *

ICEStickPlatform().build(Breathe(), do_program=True)
