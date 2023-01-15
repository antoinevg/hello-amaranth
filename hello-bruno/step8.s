    ADD    x1, x0, x0
loop:
    ADDI   x1, x1, 1
    JAL    x0, loop
    EBREAK
