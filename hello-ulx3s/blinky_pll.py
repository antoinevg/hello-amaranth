from amaranth import ClockDomain, ClockSignal, Elaboratable, Instance, Module, ResetSignal, Signal
from amaranth.build import Attrs, Pins, Platform, Resource, Subsignal


# - module: ULX3SDomainGenerator ----------------------------------------------

class ULX3SDomainGenerator(Elaboratable):
    """ Clock domain generator that creates the domain clocks for the ULX3S. """

    def elaborate(self, platform):
        m = Module()

        # Grab our default input clock.
        input_clock = platform.request(platform.default_clk, dir="i")

        # Create our domains; but don't do anything else for them, for now.
        m.domains.sync   = ClockDomain()
        m.domains.usb    = ClockDomain()
        m.domains.usb_io = ClockDomain()
        m.domains.fast   = ClockDomain()
        m.domains.foo   = ClockDomain()

        feedback = Signal()
        locked   = Signal()

        m.submodules.pll = Instance("EHXPLLL",
            i_RST=1,
            i_STDBY=0,
            i_CLKI=input_clock.i,
            i_PHASESEL0=0,
            i_PHASESEL1=0,
            i_PHASEDIR=1,
            i_PHASESTEP=1,
            i_PHASELOADREG=1,
            i_PLLWAKESYNC=0,
            i_ENCLKOP=0,
            i_CLKFB=feedback,

            o_LOCK=locked,
            o_CLKOP=feedback,
            o_CLKOS=ClockSignal("sync"),
            o_CLKOS2=ClockSignal("usb"),

            p_PLLRST_ENA="DISABLED",
            p_INTFB_WAKE="DISABLED",
            p_STDBY_ENABLE="DISABLED",
            p_DPHASE_SOURCE="DISABLED",
            p_OUTDIVIDER_MUXA="DIVA",
            p_OUTDIVIDER_MUXB="DIVB",
            p_OUTDIVIDER_MUXC="DIVC",
            p_OUTDIVIDER_MUXD="DIVD",
            p_CLKI_DIV=5,
            p_CLKOP_ENABLE="ENABLED",
            p_CLKOS2_ENABLE="ENABLED",
            p_CLKOP_DIV=48,
            p_CLKOP_CPHASE=9,
            p_CLKOP_FPHASE=0,
            p_CLKOS_ENABLE="ENABLED",
            p_CLKOS_DIV=10,
            p_CLKOS_CPHASE=0,
            p_CLKOS_FPHASE=0,
            p_CLKOS2_DIV=40,
            p_CLKOS2_CPHASE=0,
            p_CLKOS2_FPHASE=0,
            p_FEEDBK_PATH="CLKOP",
            p_CLKFB_DIV=2,

            a_FREQUENCY_PIN_CLKI="25",
            a_FREQUENCY_PIN_CLKOP="48",
            a_FREQUENCY_PIN_CLKOS="48",
            a_FREQUENCY_PIN_CLKOS2="12",
            a_ICP_CURRENT="12",
            a_LPF_RESISTOR="8",
            a_MFG_ENABLE_FILTEROPAMP="1",
            a_MFG_GMCREF_SEL="2",
        )

        # We'll use our 48MHz clock for everything _except_ the usb domain...
        m.d.comb += [
            ClockSignal("usb_io")  .eq(ClockSignal("sync")),
            ClockSignal("fast")    .eq(ClockSignal("sync")),

            ResetSignal("sync")    .eq(~locked),
            ResetSignal("fast")    .eq(~locked),
            ResetSignal("usb")     .eq(~locked),
            ResetSignal("usb_io")  .eq(~locked),
        ]

        return m


# - module: BlinkyPll ---------------------------------------------------------

class BlinkyPll(Elaboratable):
    def __init__(self):
        pass

    def elaborate(self, platform: Platform) -> Module:

        # - platform
        led0 = platform.request("led", 0)
        led1 = platform.request("led", 1)
        led2 = platform.request("led", 2)
        led3 = platform.request("led", 3)
        led4 = platform.request("led", 4)
        led5 = platform.request("led", 5)
        led6 = platform.request("led", 6)
        led7 = platform.request("led", 7)

        # - gateware
        m = Module()
        m.submodules.car = platform.clock_domain_generator()

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
            with m.Case(int(half_freq // 32)):
                m.d.sync += led5.eq(~led5)
                m.d.sync += timer.eq(timer + 1)
            with m.Case(int(half_freq // 64)):
                m.d.sync += led6.eq(~led6)
                m.d.sync += timer.eq(timer + 1)
            with m.Case(int(half_freq // 128)):
                m.d.sync += led7.eq(~led7)
                m.d.sync += timer.eq(timer + 1)
            with m.Default():
                m.d.sync += timer.eq(timer + 1)

        return m

# - build: ulx3s --------------------------------------------------------------

import amaranth_boards.ulx3s as _ulx3s

class BlinkyPlatform(_ulx3s.ULX3S_85F_Platform):
    name                   = "ULX3S (85F)"
    clock_domain_generator = ULX3SDomainGenerator

if __name__ == "__main__":
    #_ulx3s.ULX3S_85F_Platform().build(BlinkyPll(), do_program=True)
    BlinkyPlatform().build(BlinkyPll(), do_program=True)
