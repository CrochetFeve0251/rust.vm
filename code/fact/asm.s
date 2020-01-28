MOV r3, 0
MOV r4, 0
MOV r5, 0
MOV r6, r0
MOV r3, r0
MOV r0, r3
SUB r6, r6, 1
MOV r1, r6
MOV r5, r4
MOV r3, 0
MOV r4, 0
CMP r6, 0
BEQ 17
CMP r1, 0
BEQ 11
LSH r7, r1, 63
RSH r7, r7, 63
CMP r7, 1
BNE 3
ADC r3, r3, r0
ADC r4, r4, r5
LSH r0, r0, 1
LSH r5, r5, 1
RSH r1, r1, 1
B -11
MOV r8, r3
MOV r9, r4
B -22