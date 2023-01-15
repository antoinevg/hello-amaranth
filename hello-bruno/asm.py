# -  metastableB / RISCV-RV32I-Assembler  -------------------------------------

# Dependencies
#
# pip install ply==3.9
#
# see: https://github.com/metastableB/RISCV-RV32I-Assembler

import sys
sys.path.append("rvi")


# - assembler -----------------------------------------------------------------

from rvi.lib.line_parser import *

def assemble(source):
    lines = source.split("\n")

    # clean up input
    source = []
    for linenumber, line in enumerate(lines):
        line = line.replace(' x', ' $').lower()
        if len(line.strip()) == 0:
            continue
        source.append(line + "\n")

    debug = False
    symbols_table = parse_pass_one(source, {"echo_symbols": debug})
    output = parse_pass_two(source, [], symbols_table, {"hex": True, "echo": debug, "tokenize": debug})

    return output

# - test ----------------------------------------------------------------------

if __name__ == "__main__":
    asm = """
        ADD    x1, x0, x0
    loop:
        ADDI   x1, x1, 1
        JAL    x0, loop
    #   EBREAK
    """

    output = assemble(asm)
    for address, instruction in enumerate(output):
        hs = "0x{0:08x}".format(instruction)
        print("{}: {}".format(address, hs))
