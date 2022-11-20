from amaranth         import *
from amaranth.build   import *

from amaranth_soc          import wishbone
from amaranth_soc.periph   import ConstantMap
from amaranth_stdio.serial import AsyncSerial
from amaranth_boards.ulx3s import ULX3S_85F_Platform

from lambdasoc.cores       import litedram, liteeth
from lambdasoc.cpu.minerva import MinervaCPU

from lambdasoc.periph.intc   import GenericInterruptController
from lambdasoc.periph.serial import AsyncSerialPeripheral
from lambdasoc.periph.sram   import SRAMPeripheral
from lambdasoc.periph.timer  import TimerPeripheral
from lambdasoc.periph.sdram  import SDRAMPeripheral
from lambdasoc.periph.eth    import EthernetMACPeripheral

from lambdasoc.soc.cpu import CPUSoC, BIOSBuilder

from lambdasoc.sim.blackboxes.serial import AsyncSerial_Blackbox

from car import ECP5DomainGenerator


__all__ = ["MinervaSoC"]


class MinervaSoC(CPUSoC, Elaboratable):
    def __init__(self,
            sync_clk_freq,
            cpu_core,
            bootrom_addr,
            bootrom_size,
            scratchpad_addr,
            scratchpad_size,
            uart_core,
            uart_addr,
            uart_irqno,
            timer_addr,
            timer_width,
            timer_irqno):
        if not isinstance(sync_clk_freq, (int, float)) or sync_clk_freq <= 0:
            raise ValueError("Sync domain clock frequency must be a positive integer or float, "
                             "not {!r}"
                             .format(sync_clk_freq))
        self.sync_clk_freq = int(sync_clk_freq)

        if not isinstance(cpu_core, MinervaCPU):
            raise TypeError("CPU core must be an instance of MinervaCPU, not {!r}"
                            .format(cpu_core))
        self.cpu = cpu_core

        self._arbiter = wishbone.Arbiter(addr_width=30, data_width=32, granularity=8,
                                         features={"cti", "bte", "err"})
        self._decoder = wishbone.Decoder(addr_width=30, data_width=32, granularity=8,
                                         features={"cti", "bte", "err"})

        self._arbiter.add(self.cpu.ibus)
        self._arbiter.add(self.cpu.dbus)

        self.intc = GenericInterruptController(width=len(self.cpu.ip))

        self.bootrom = SRAMPeripheral(size=bootrom_size, writable=False)
        self._decoder.add(self.bootrom.bus, addr=bootrom_addr)

        self.scratchpad = SRAMPeripheral(size=scratchpad_size)
        self._decoder.add(self.scratchpad.bus, addr=scratchpad_addr)

        self.uart = AsyncSerialPeripheral(core=uart_core)
        self._decoder.add(self.uart.bus, addr=uart_addr)
        self.intc.add_irq(self.uart.irq, index=uart_irqno)

        self.timer = TimerPeripheral(width=timer_width)
        self._decoder.add(self.timer.bus, addr=timer_addr)
        self.intc.add_irq(self.timer.irq, index=timer_irqno)

        self._sdram  = None
        self._sram   = None
        self._ethmac = None

    @property
    def memory_map(self):
        return self._decoder.bus.memory_map

    @property
    def constants(self):
        return super().constants.union(
            SDRAM  = self.sdram .constant_map if self.sdram  is not None else None,
            ETHMAC = self.ethmac.constant_map if self.ethmac is not None else None,
            SOC    = ConstantMap(
                WITH_SDRAM        = self.sdram  is not None,
                WITH_ETHMAC       = self.ethmac is not None,
                MEMTEST_ADDR_SIZE = 8192,
                MEMTEST_DATA_SIZE = 8192,
            ),
        )

    @property
    def mainram(self):
        assert not (self._sdram and self.sram)
        return self._sdram or self._sram

    @property
    def sdram(self):
        return self._sdram

    @property
    def sram(self):
        return self._sram

    def add_sdram(self, core, *, addr, cache_size):
        if self.mainram is not None:
            raise AttributeError("Main RAM has already been set to {!r}".format(self.mainram))
        if core.config.user_clk_freq != self.sync_clk_freq:
            raise ValueError("LiteDRAM user domain clock frequency ({} MHz) must match sync "
                             "domain clock frequency ({} MHz)"
                             .format(core.config.user_clk_freq / 1e6, self.sync_clk_freq / 1e6))
        self._sdram = SDRAMPeripheral(core=core, cache_size=cache_size)
        self._decoder.add(self._sdram.bus, addr=addr)

    def add_internal_sram(self, *, addr, size):
        if self.mainram is not None:
            raise AttributeError("Main RAM has already been set to {!r}".format(self.mainram))
        self._sram = SRAMPeripheral(size=size)
        self._decoder.add(self._sram.bus, addr=addr)

    @property
    def ethmac(self):
        return self._ethmac

    def add_ethmac(self, core, *, addr, irqno, local_ip, remote_ip):
        if self._ethmac is not None:
            raise AttributeError("Ethernet MAC has already been set to {!r}"
                                 .format(self._ethmac))
        self._ethmac = EthernetMACPeripheral(core=core, local_ip=local_ip, remote_ip=remote_ip)
        self._decoder.add(self._ethmac.bus, addr=addr)
        self.intc.add_irq(self._ethmac.irq, index=irqno)

    def elaborate(self, platform):
        m = Module()

        m.submodules.crg = ECP5DomainGenerator(
            sync_clk_freq = self.sync_clk_freq,
            with_sdram    = self.sdram is not None,
            with_ethernet = self.ethmac is not None,
        )

        m.submodules.cpu        = self.cpu
        m.submodules.arbiter    = self._arbiter
        m.submodules.decoder    = self._decoder
        m.submodules.uart       = self.uart
        m.submodules.timer      = self.timer
        m.submodules.intc       = self.intc
        m.submodules.bootrom    = self.bootrom
        m.submodules.scratchpad = self.scratchpad

        if self.sdram is not None:
            m.submodules.sdram = self.sdram
        if self.sram is not None:
            m.submodules.sram = self.sram
        if self.ethmac is not None:
            m.submodules.ethmac = self.ethmac

        m.d.comb += [
            self._arbiter.bus.connect(self._decoder.bus),
            self.cpu.ip.eq(self.intc.ip),
        ]

        return m

    def build(self, build_dir, name=None, do_build=True, do_init=False):
        super().build(build_dir=build_dir, name=name, do_build=do_build, do_init=do_init)
