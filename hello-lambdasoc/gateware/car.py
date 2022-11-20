from amaranth         import *
from amaranth.build   import *
from amaranth.lib.cdc import ResetSynchronizer

from amaranth_boards.ulx3s import ULX3S_85F_Platform

from lambdasoc.cores.pll.lattice_ecp5 import PLL_LatticeECP5
from lambdasoc.sim.platform           import CXXRTLPlatform


class ECP5DomainGenerator(Elaboratable):
    def __init__(self, *, sync_clk_freq, with_sdram, with_ethernet):
        if not isinstance(sync_clk_freq, (int, float)) or sync_clk_freq <= 0:
            raise ValueError("Sync domain clock frequency must be a positive integer or float, "
                             "not {!r}"
                             .format(sync_clk_freq))
        self.sync_clk_freq = sync_clk_freq
        self.with_sdram    = bool(with_sdram)
        self.with_ethernet = bool(with_ethernet)

    def elaborate(self, platform):
        m = Module()

        m.domains += [
            ClockDomain("_ref", reset_less=platform.default_rst is None, local=True),
            ClockDomain("sync"),
        ]

        m.d.comb += ClockSignal("_ref").eq(platform.request(platform.default_clk, 0).i)
        if platform.default_rst is not None:
            m.d.comb += ResetSignal("_ref").eq(platform.request(platform.default_rst, 0).i)

        # The LiteDRAM core provides its own PLL, which drives the litedram_user clock domain.
        # We reuse this clock domain as the sync domain, in order to avoid CDC between LiteDRAM
        # and the SoC interconnect.
        if self.with_sdram:
            m.domains += ClockDomain("litedram_input")
            m.d.comb += ClockSignal("litedram_input").eq(ClockSignal("_ref"))
            if platform.default_rst is not None:
                m.d.comb += ResetSignal("litedram_input").eq(ResetSignal("_ref"))

            m.domains += ClockDomain("litedram_user")
            m.d.comb += [
                ClockSignal("sync").eq(ClockSignal("litedram_user")),
                ResetSignal("sync").eq(ResetSignal("litedram_user")),
            ]

        # In simulation mode, the sync clock domain is directly driven by the platform clock.
        elif isinstance(platform, CXXRTLPlatform):
            assert self.sync_clk_freq == platform.default_clk_frequency
            m.d.comb += ClockSignal("sync").eq(ClockSignal("_ref"))
            if platform.default_rst is not None:
                m.d.comb += ResetSignal("sync").eq(ResetSignal("_ref"))

        # Otherwise, we use a PLL to drive the sync clock domain.
        else:
            if isinstance(platform, ULX3S_85F_Platform):
                sync_pll_params = PLL_LatticeECP5.Parameters(
                    i_domain     = "_ref",
                    i_freq       = platform.default_clk_frequency,
                    i_reset_less = platform.default_rst is None,
                    o_domain     = "sync",
                    o_freq       = self.sync_clk_freq,
                )
                m.submodules.sync_pll = sync_pll = PLL_LatticeECP5(sync_pll_params)
            else:
                assert False

            if platform.default_rst is not None:
                sync_pll_arst = ~sync_pll.locked | ResetSignal("_ref")
            else:
                sync_pll_arst = ~sync_pll.locked

            m.submodules += ResetSynchronizer(sync_pll_arst, domain="sync")

        return m
