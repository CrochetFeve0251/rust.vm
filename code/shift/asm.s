MOV r2, 0
CMP r1, 0
BEQ 5
LSH r0, r0, 1
LSH r2, r2, 1
SUB r1, r1, 1
B -5