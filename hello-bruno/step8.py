from amaranth import *

from enum import Enum, unique
from typing import List

from asm import assemble


# - ISA -----------------------------------------------------------------------

@unique
class Opcode(Enum):
    LOAD   = 0b00_000_11 #   3 I  rd <- mem[rs1+Iimm]         (isLoad)
    OP_IMM = 0b00_100_11 #  19 I  rd <- rs1 OP Iimm           (isALUimm)
    AUIPC  = 0b00_101_11 #  23 U  rd <- PC + Uimm             (isAUIPC)
    STORE  = 0b01_000_11 #  35 S  mem[rs1+Simm] <- rs2        (isStore)
    OP     = 0b01_100_11 #  51 R  rd <- rs1 OP rs2            (isALUreg)
    LUI    = 0b01_101_11 #  55 U  rd <- Uimm                  (isLUI)
    BRANCH = 0b11_000_11 #  99 B  if(rs1 OP rs2) PC<-PC+Bimm  (isBranch)
    JALR   = 0b11_001_11 # 103 I  rd <- PC+4; PC<-rs1+Iimm    (isJALR)
    JAL    = 0b11_011_11 # 111 J  rd <- PC+4; PC<-PC+Jimm     (isJAL)
    SYSTEM = 0b11_100_11 # 115    special                     (isSYSTEM)

class Funct3(Enum):
    ADD = BEQ  = B  = 0b000  # add | subtract
    SLL = BNE  = H  = 0b001  # left shift
    SLT        = W  = 0b010  # signed comparison (<)
    SLTU            = 0b011  # unsigned comapriston (<)
    XOR = BLT  = BU = 0b100  # xor
    SR  = BGE  = HU = 0b101  # logical right shift | arithmetic right shift
    OR  = BLTU      = 0b110  # or
    AND = BGEU      = 0b111  # and

class Funct7(Enum):
    SRL = ADD = 0b0000000
    SRA = SUB = 0b0100000

# - module: Clockworks --------------------------------------------------------

class Clockworks(Elaboratable):
    def __init__(self, clock_multiplier=21):
        self.clock_multiplier = clock_multiplier

        self.clk_out = Signal()

    def elaborate(self, platform):
        m = Module()

        # registers
        self.slow_clk = Signal(self.clock_multiplier + 1)

        # sync
        m.d.sync += self.slow_clk.eq(self.slow_clk + 1)

        # comb
        m.d.comb += self.clk_out.eq(self.slow_clk[self.clock_multiplier])

        return m

    def ports(self):
        return [self.clk_out]



# - module: SoC ---------------------------------------------------------------

@unique
class State(Enum):
    FETCH_INSTR = 0
    FETCH_REGS  = 1
    EXECUTE     = 2


class SoC(Elaboratable):
    def __init__(self):
        # ports
        self.clk_in = Signal()
        self.rst_in = Signal()
        self.leds_out = Signal(8)
        self.rxd_in = Signal()
        self.txd_out = Signal()

        # instruction bus
        #    add x0, x0, x0
        #                                          rs2   rs1 add    rd  ALUREG
        self._instr = Signal(32, reset=0b0000000_00000_00000_000_00000_0110011)

        # memory
        # instructions = [
        #     0x000000b3, #     ADD     ra,zero,zero
        #                 # loop:
        #     0x00108093, #     ADDI    ra,ra,1
        #     0xffdff06f, #     j       4 <loop>
        #     0x00100073, #     EBREAK
        # ]

        asm = """
            ADD    x1, x0, x0
        loop:
            ADDI   x1, x1, 1
            JAL    x0, loop
        # EBREAK
        """
        instructions = assemble(asm)
        instructions.append(0x00100073) # EBREAK

        self._MEM = Array([Const(instruction) for instruction in instructions])

        # program counter
        self._PC = Signal(32, reset=0)

        # Register Bank
        self._RegisterBank = Array([Signal(32) for _ in range(32)])
        self._rs1 = Signal(32)
        self._rs2 = Signal(32)
        self._writeBackData = Signal(32, reset=0)
        self._writeBackEn = Signal(reset=0)

    def elaborate(self, platform):
        m = Module()
        m.domains.clk_in = ClockDomain()
        m.d.comb += ClockSignal("clk_in").eq(self.clk_in)

        # - instruction decoder --

        instr  = Signal(32)

        opcode = Signal(Opcode)
        Uimm   = Signal(32)
        Iimm   = Signal(32)
        Simm   = Signal(32)
        Bimm   = Signal(32)
        Jimm   = Signal(32)
        rs1Id  = Signal(5)
        rs2Id  = Signal(5)
        rdId   = Signal(5)
        funct3 = Signal(3)
        funct7 = Signal(7)

        isALUreg = Signal(reset=0)
        isALUimm = Signal(reset=0)
        isBRANCH = Signal(reset=0)
        isJALR   = Signal(reset=0)
        isJAL    = Signal(reset=0)
        isAUIPC  = Signal(reset=0)
        isLUI    = Signal(reset=0)
        isLOAD   = Signal(reset=0)
        isSTORE  = Signal(reset=0)
        isSYSTEM = Signal(reset=0)

        m.d.comb += [
            # opcode
            opcode.eq(instr[:7]),

            # the 5 immediate formats
            Uimm.eq(Cat(Repl(C(0), 12),
                        instr[12:])),
            Iimm.eq(Cat(instr[20:],
                        Repl(instr[31], 20))),
            Simm.eq(Cat(instr[7:12],
                        instr[25:],
                        Repl(instr[31], 20))),
            Bimm.eq(Cat(C(0),
                        instr[8:12],
                        instr[25:31],
                        instr[7],
                        instr[31],
                        Repl(instr[31], 19))),
            Jimm.eq(Cat(C(0),
                        instr[21:31],
                        instr[20],
                        instr[12:20],
                        instr[31],
                        Repl(instr[31], 11))),

            # source and destination registers
            rs1Id.eq(instr[15:20]),
            rs2Id.eq(instr[20:25]),
            rdId.eq(instr[7:12]),

            # function codes
            funct3.eq(instr[12:15]),
            funct7.eq(instr[25:]),
        ]
        with m.Switch(opcode):
            with m.Case(Opcode.OP):
                m.d.comb += isALUreg.eq(1)
            with m.Case(Opcode.OP_IMM):
                m.d.comb += isALUimm.eq(1)
            with m.Case(Opcode.BRANCH):
                m.d.comb += isBRANCH.eq(1)
            with m.Case(Opcode.JALR):
                m.d.comb += isJALR.eq(1)
            with m.Case(Opcode.JAL):
                m.d.comb += isJAL.eq(1)
            with m.Case(Opcode.AUIPC):
                m.d.comb += isAUIPC.eq(1)
            with m.Case(Opcode.LUI):
                m.d.comb += isLUI.eq(1)
            with m.Case(Opcode.LOAD):
                m.d.comb += isLOAD.eq(1)
            with m.Case(Opcode.STORE):
                m.d.comb += isSTORE.eq(1)
            with m.Case(Opcode.SYSTEM):
               m.d.comb += isSYSTEM.eq(1)
            with m.Default():
                m.d.comb += [
                    isALUreg.eq(0),
                    isALUimm.eq(0),
                    isBRANCH.eq(0),
                    isJALR.eq(0),
                    isJAL.eq(0),
                    isAUIPC.eq(0),
                    isLUI.eq(0),
                    isLOAD.eq(0),
                    isSTORE.eq(0),
                    isSYSTEM.eq(0),
                ]


        # - comb : the ALU ------------

        aluIn1 = Signal(32)
        aluIn2 = Signal(32)
        aluOut = Signal(32)
        shamt = Signal(5)

        m.d.comb += aluIn1.eq(self._rs1)

        with m.If(isALUreg == C(1)):
            m.d.comb += aluIn2.eq(self._rs2)
            m.d.comb += shamt.eq(self._rs2[0:5])
        with m.Else():
            m.d.comb += aluIn2.eq(Iimm)
            m.d.comb += shamt.eq(instr[20:25]) # shift amount

        # ADD/SUB/ADDI:
        #   funct7[5] is 1 for SUB and 0 for ADD. We need also to test instr[5] (aka isALUreg/isALUimm)
        #   to make the difference with ADDI
        # SRLI/SRAI/SRL/SRA:
        #   funct7[5] is 1 for arithmetic shift (SRA/SRAI) and 0 for logical
        #   shift (SRL/SRLI). We also need to test instr[5] (aka isALUreg/isALUimm) ...
        alt_func = Signal()
        m.d.comb += alt_func.eq(funct7[5])

        with m.Switch(funct3):
            with m.Case(Funct3.ADD):
                # Funct7 is only valid if instruction type is OP
                m.d.comb += aluOut.eq(Mux(alt_func & isALUreg,
                                          aluIn1 - aluIn2,
                                          aluIn1 + aluIn2))
            with m.Case(Funct3.SLL):
                m.d.comb += aluOut.eq(aluIn1 << shamt)
            with m.Case(Funct3.SLT):
                m.d.comb += aluOut.eq(Mux(aluIn1.as_signed() < aluIn2.as_signed(),
                                          0b1,
                                          0b0))
            with m.Case(Funct3.SLTU):
                m.d.comb += aluOut.eq(Mux(aluIn1 < aluIn2,
                                          0b1,
                                          0b0))
            with m.Case(Funct3.XOR):
                m.d.comb += aluOut.eq(aluIn1 ^ aluIn2)
            with m.Case(Funct3.SR):
                # Funct7 is only valid if instruction type is OP or OP-IMM
                m.d.comb += aluOut.eq(Mux(alt_func & (isALUreg | isALUimm),
                                          aluIn1.as_signed() >> shamt,
                                          aluIn1 >> shamt))
            with m.Case(Funct3.OR):
                m.d.comb += aluOut.eq(aluIn1 | aluIn2)
            with m.Case(Funct3.AND):
                m.d.comb += aluOut.eq(aluIn1 & aluIn2)

        # - sync : the state machine --

        state = Signal(State, reset=State.FETCH_INSTR)

        # set register write back enable
        m.d.comb += [
            self._writeBackData.eq(Mux(
                (isJAL | isJALR),
                self._PC + 4,
                aluOut
            )),
            self._writeBackEn.eq(Mux(
                (state == State.EXECUTE) & (isALUreg | isALUimm | isJAL | isJALR),
                0b1,
                0b0
            ))
        ]

        # set next PC
        nextPC = Signal(32)
        with m.If(isJAL == C(1)):
            m.d.comb += nextPC.eq(self._PC + Jimm)
        with m.Elif(isJALR == C(1)):
            m.d.comb += nextPC.eq(self._PC + Iimm)
        with m.Else():
            m.d.comb += nextPC.eq(self._PC + 4)

        with m.If(self.rst_in == C(1)):
            m.d.clk_in += self._PC.eq(0)
            m.d.clk_in += state.eq(State.FETCH_INSTR)
            m.d.clk_in += instr.eq(0b0000000_00000_00000_000_00000_0110011) # NOP

        with m.Elif(isSYSTEM != C(1)):
            with m.If((self._writeBackEn) & (rdId != 0)):
                m.d.clk_in += self._RegisterBank[rdId].eq(self._writeBackData)

            with m.Switch(state):
                with m.Case(State.FETCH_INSTR):
                    address = self._PC[2:32]
                    m.d.clk_in += instr.eq(self._MEM[address])
                    m.d.clk_in += state.eq(State.FETCH_REGS)
                with m.Case(State.FETCH_REGS):
                    m.d.clk_in += self._rs1.eq(self._RegisterBank[rs1Id])
                    m.d.clk_in += self._rs2.eq(self._RegisterBank[rs2Id])
                    m.d.clk_in += state.eq(State.EXECUTE)
                with m.Case(State.EXECUTE):
                    with m.If(isSYSTEM == C(0)):
                        m.d.clk_in += self._PC.eq(nextPC)
                    m.d.clk_in += state.eq(State.FETCH_INSTR)

        # - diagnostics --

        m.d.comb += [
            self.leds_out[0:7].eq(self._RegisterBank[1]),
            self.leds_out[7].eq(isSYSTEM),
        ]

        return m

    def ports(self) -> List[Signal]:
        return [self.clk_in, self.rst_in, self.leds_out, self.rxd_in, self.txd_out]



# - module: Top ---------------------------------------------------------------

class Top(Elaboratable):
    def __init__(self, clock_multiplier=21):
        # peripherals
        self.leds = Signal(8)
        self.pmod = Signal(8)
        self.button_pwr = Signal()

        # submodules
        self.clockworks = Clockworks(clock_multiplier)
        self.soc = SoC()

    def elaborate(self, platform):
        m = Module()

        # peripherals
        if platform is not None:
            self.leds = Cat(platform.request("led", i) for i in range(8))
            self.pmod = platform.request("pmod", 1)
            self.button_pwr = platform.request("button_pwr", 0)

        # submodules
        m.submodules += [
            self.clockworks,
            self.soc
        ]

        # connections
        m.d.comb += [
            self.soc.clk_in.eq(self.clockworks.clk_out),
            self.soc.rst_in.eq(self.button_pwr),

            # diagnostics
            self.leds.eq(self.soc.leds_out),
            self.pmod.eq(self.soc._PC),
        ]

        return m

    def ports(self):
        return self.soc.ports() + self.clockworks.ports()



# - simulate ------------------------------------------------------------------

# see: https://github.com/RobertBaruch/amaranth-tutorial/blob/main/7_simulating.md

if False:
    from amaranth.sim import Simulator

    dut = Top(clock_multiplier=1)

    sim = Simulator(dut)

    def bench():
        yield

    sim.add_clock(1e-6, domain="sync")
    sim.add_sync_process(bench, domain="sync")

    with sim.write_vcd("step7.vcd", "step7.gtkw", traces=dut.ports()):
        sim.run_until(150e-6, run_passive=True) # 20s

    import sys
    sys.exit(0)

# - generate: verilog ---------------------------------------------------------

if False:
    from amaranth.back import verilog

    top = Top()
    with open("top.v", "w") as f:
        f.write(verilog.convert(top, ports=top.ports()))

    soc = SoC()
    with open("soc.v", "w") as f:
        f.write(verilog.convert(soc, ports=soc.ports()))

    clockworks = Clockworks()
    with open("clockworks.v", "w") as f:
        f.write(verilog.convert(clockworks, ports=clockworks.ports()))



# - build: ulx3s --------------------------------------------------------------

if True:
    from amaranth.build import Pins, Resource, Subsignal
    from amaranth_boards.ulx3s import *

    # platform
    platform = ULX3S_85F_Platform()
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

    # build
    top = Top()
    #platform.build(top, do_program=False)
    platform.build(top, do_program=True)
