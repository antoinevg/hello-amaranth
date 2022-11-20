from amaranth import *
from amaranth.build import *
from amaranth_stdio.serial import AsyncSerial

from lambdasoc.cpu.minerva import MinervaCPU

from minerva_soc import MinervaSoC

# - module: Top ---------------------------------------------------------------

class Top(Elaboratable):
    def __init__(self, platform):
        sync_clk_freq = 50_000_000
        internal_sram_size = 8192
        uart_baudrate   = 115200 # default: 9600

        print("sync_clk_freq: ", sync_clk_freq)

        uart_core = AsyncSerial(
            data_bits = 8,
            divisor   = int(sync_clk_freq // uart_baudrate),
            pins      = platform.request("uart", 0),
        )

        self.soc = soc = MinervaSoC(
            sync_clk_freq = sync_clk_freq,

            cpu_core = MinervaCPU(
                reset_address = 0x00000000,
                with_icache   = True,
                icache_nlines = 16,
                icache_nwords = 4,
                icache_nways  = 1,
                icache_base   = 0x40000000,
                icache_limit  = 0x40000000 + internal_sram_size,
                with_dcache   = True,
                dcache_nlines = 16,
                dcache_nwords = 4,
                dcache_nways  = 1,
                dcache_base   = 0x40000000,
                dcache_limit  = 0x40000000 + internal_sram_size,
                with_muldiv   = True,
            ),

            bootrom_addr    = 0x00000000,
            bootrom_size    = 0x8000,
            scratchpad_addr = 0x00008000,
            scratchpad_size = 0x1000,

            uart_addr       = 0x80000000,
            uart_core       = uart_core,
            uart_irqno      = 1,
            timer_addr      = 0x80001000,
            timer_width     = 32,
            timer_irqno     = 0,
        )

        soc.add_internal_sram(addr=0x40000000, size=internal_sram_size)

    def elaborate(self, platform):
        m = Module()

        m.submodules.soc = self.soc

        return m

    def build(self, name=None, build_dir="build"):
        print("Building BIOS")
        self.soc.build(name=name, build_dir=build_dir)
        print("Built BIOS")


# - main ----------------------------------------------------------------------

from amaranth_boards.ulx3s import ULX3S_85F_Platform

if __name__ == "__main__":
    platform = ULX3S_85F_Platform()
    top = Top(platform)

    # build bios ?
    import os
    build_dir = os.path.join("build", 'soc')

    # TODO fix build
    #thirdparty = os.path.join(build_dir, "lambdasoc.soc.cpu/bios/3rdparty/litex")
    #if not os.path.exists(thirdparty):
    #    print("Fixing build, creating output directory: ", thirdparty)
    #    os.makedirs(thirdparty)

    print("Generating svd for SoC: firmware/minerva-pac/svd/minerva.svd")
    import svd
    svd.build(top.soc, name="minerva", build_dir="firmware/minerva-pac/svd/")

    import sys
    sys.exit(0)

    print("Building bios")
    top.soc.build(name="soc", build_dir=build_dir, do_init=True)

    print("Building soc")
    platform.build(top, do_program=False)

    print("Build completed. Use 'make program' to load bitsream to device.")
