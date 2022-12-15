add  x0 x0 x0
add  x1 x0 x0
addi x1 x1 1
addi x1 x1 1
addi x1 x1 1
addi x1 x1 1
add  x2 x1 x0
add  x3 x1 x2
srliw x3 x3 3
slliw x3 x3 31
srai x3 x3 5
srliw x1 x3 26
# TODO ebreak
