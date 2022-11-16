# - module: Blinky ------------------------------------------------------------

from amaranth import *

class Blinky(Elaboratable):
    def elaborate(self, platform):
        m = Module()

        led = platform.request("led")

        half_freq = int(platform.default_clk_frequency // 2)
        timer = Signal(range(half_freq + 1))

        with m.If(timer == half_freq):
            m.d.sync += led.eq(~led)
            m.d.sync += timer.eq(0)

        with m.Else():
            m.d.sync += timer.eq(timer + 1)

        return m


# - build: ulx3s --------------------------------------------------------------

from amaranth_boards.ulx3s import *

ULX3S_85F_Platform().build(Blinky(), do_program=True)
