MOV r0, 0
MOV r1, 0
MOV r2, 0
MOV r3, 0
MOV r4, 1
CMP r3, 56
BGE 6
ADD r0, r0, r4
LSH r0, r0, 4
ADD r3, r3, 4
ADD r4, r4, 1
B -6
MOV r3, 165
MOV r4, 0
CMP r4, 8
BGE 5
ADD r1, r1, r3
LSH r1, r1, 8
ADD r4, r4, 1
B -5
ADD r1, r1, r3
MOV r3, 10
LSH r3, r3, 56
MOV r2, r3
MOV r3, 14
LSH r3, r3, 52
ADD r2, r3
MOV r3, 15
LSH r3, r3, 48
ADD r2, r3
MOV r3, 4
LSH r3, r3, 44
ADD r2, r3
MOV r3, 5
LSH r3, r3, 40
ADD r2, r3
MOV r3, 13
LSH r3, r3, 36
ADD r2, r3
MOV r3, 7
LSH r3, r3, 32
ADD r2, r3
MOV r3, 4
LSH r3, r3, 28
ADD r2, r3
MOV r3, 10
LSH r3, r3, 24
ADD r2, r3
MOV r3, 15
LSH r3, r3, 20
ADD r2, r3
MOV r3, 15
LSH r3, r3, 16
ADD r2, r3
MOV r3, 5
LSH r3, r3, 12
ADD r2, r3
MOV r3, 8
LSH r3, r3, 8
ADD r2, r3
MOV r3, 4
LSH r3, r3, 4
ADD r2, r3
MOV r3, 15
ADD r2, r3
MOV r4, 0