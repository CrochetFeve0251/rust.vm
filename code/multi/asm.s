MOV r3, 0
MOV r4, 0
CMP r0, 0
BEQ 5
ADC r3, r3, r0
ADC r4, r4, 0
SUB r1, r1, 1
B -5