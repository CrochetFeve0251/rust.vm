MOV r3, 0
MOV r4, 0
MOV r5, 0
MOV r6, 0
CMP r1, 0
BEQ 11
LSH r0, r0, 1
LSH r5, r5, 1
RSH r1, r1, 1
LSH r7, r1, 63
RSH r7, r7, 63
CMP r7, 1
BNE 3
ADC r3, r3, r0
ADC r4, r4, r5
B -11