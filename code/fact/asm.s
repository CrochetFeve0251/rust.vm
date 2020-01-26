MOV r2, 0
MOV r3, 0
CMP r0, 0
BNQ 2
CMP r1, 0
BEQ 14
MOV r4, r0
MOV r5, r1
CMP r4, 0
BNQ 2
CMP r5, 0
BEQ 4
ADC r2,r2, r4
ADC r3, r3, r5
SUB r4, r4, 1
SUB r5, r5, 0
B -8
SUB r0, r0, 1
SUB r1, r1, 0
B -17