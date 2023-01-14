# https://github.com/kcelebi/riscv-assembler
# pip install riscv-assembler
# if needed: pip install bitstring
from riscv_assembler.convert import AssemblyConverter, flatten, nibbleForm
#from riscv_assembler.utils import *


# - assembler -----------------------------------------------------------------

import io

def assemble(input: str) -> [str]:
    input = input.split('\n')
    cnv = AssemblyConverter()

    # clean input
    code = []
    for line in input:
        line = line.strip()
        line = line.lower()
        if len(line) == 0:
            continue
        clean = flatten([elem.replace("\n","").split(",") for elem in line.split(" ")])

        # dirty hack to support RV32
        #if clean[0] == "srli":
        #    clean[0] = "srliw"
        #line = " ".join(clean)
        #print("line: ", line)

        if not cnv._AssemblyConverter__valid_line(clean, True):
            print("Invalid line:", line)
            continue
        code.append(line)

    # get instructions
    instructions = []
    for i, line in enumerate(code):
        response = cnv._AssemblyConverter__interpret(line, i)
        if -1 in response:
            print("Failed to interpret line {}: '{}'".format(i, line))
            continue

        instructions.extend(response)

    # Format: convert to nibble form
    if False:
        for i, instruction in enumerate(instructions):
            instructions[i] = nibbleForm(instruction)

    # Format: convert to hex
    if False:
        for i, instruction in enumerate(instructions):
            instructions[i] = cnv.hex(instruction)

    # Format: convert to 32-bit bin
    if True:
        for i, instruction in enumerate(instructions):
            instructions[i] = int(instruction, 2)

    # Format: convert to 8-bit bin
    if False:
        ret = []
        for i, instruction in enumerate(instructions):
            byte_array = [instruction[n : n + 8] for n in range(0, len(instruction), 8)]
            byte_list = [int(b, 2) for b in byte_array]
            ret += byte_list
        instructions = ret

    return instructions




# - test ----------------------------------------------------------------------

if __name__ == "__main__":
     cnv = AssemblyConverter()

     # assemble file: step7.s
     output_a = cnv.convert_ret("step7.s")
     #print(output_a)

     asm_1 = """
         ADD   x0 x0 x0
         ADD   x1 x0 x0
         ADDI  x1 x1 1
         ADDI  x1 x1 1
         ADDI  x1 x1 1
         ADDI  x1 x1 1
         ADD   x2 x1 x0
         ADD   x3 x1 x2
         SRLIW x3 x3 3
         SLLIW x3 x3 31
         SRAI  x3 x3 5
         SRLIW x1 x3 26
         EBREAK x0 x0 0
         #TODO  ^^ ^^ ^
     """.strip()

     asm = """
         ADD    x1, x0, x0
     loop:
         ADDI   x1, x1, 1
         JAL    x0, loop
         EBREAK x0, x0, 0
         #TODO  ^^ ^^ ^
     """.strip()

     instructions = assemble(asm)
     print("assembled: ", instructions)

     # dump it
     asm = asm.strip().split("\n")
     asm = [line.strip() for line in asm]
     for i, instruction in enumerate(instructions):
         hs = "0x{0:08x}".format(instruction)
         bs = "0b{0:032b}".format(instruction)
         print(asm[i])
         print(hs)
         #print(bs)
