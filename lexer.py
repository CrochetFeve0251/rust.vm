opcode = {
    "AND" : "0x0",
    "ORR"  : "0x1",
    "XOR" : "0x2",
    "ADD" : "0x3",
    "ADC" : "0x4",
    "CMP" : "0x5",
    "SUB" : "0x6",
    "SBC" : "0x7",
    "MOV" : "0x8",
    "LSH" : "0x9",
    "RSH" : "0xa"
}

bcc = {
    "B" : "0x8",
    "BEQ" : "0x9",
    "BNE" : "0xa",
    "BLE" : "0xb",
    "BGE" : "0xc",
    "BL" : "0xd",
    "BG" : "0xe"
}

def lexer(data):
    return [data.split(" ")[0]] + " ".join(data.split(" ")[1:]).split(", ")


def compile(tab):
    bits = ""

    # Branch instruction
    if len(tab) == 2 and tab[0] in bcc:
        bits += bin(int(bcc[tab[0]], 16))[2:]  # Branch Condition Code
        bits += "0" if int(tab[1]) > 0 else "1"  # Positive or Negative
        bits += ('{0: >27}'.format(bin(int(tab[1]))[2:]).replace(" ", "0"))  # Value

    # Data processing instruction
    elif tab[0] in opcode:
            bits += "0000000" + ("0" if tab[-1][0] == 'r' else "1")  # Branch Condition + Immediate Value Flag
            bits += ('{0: >4}'.format(bin(int(opcode[tab[0]], 16))[2:]).replace(" ", "0"))  # Oeration Code
            bits += ('{0: >4}'.format(bin(int(tab[-2][1:]))[2:]).replace(" ", "0"))  # First Operand
            bits += "0000" if tab[-1][0] != 'r' else '{0: >4}'.format(bin(int(tab[-1][1:]))[:2]).replace(" ", "0")  # Second Operand
            if tab[0] == "CMP":
                bits += "0000"  # No destination for CMP
            else:
                bits += ('{0: >4}'.format(bin(int(tab[1][1:]))[2:]).replace(" ", "0"))  # Destination
            bits += "00000000" if tab[-1][0] == 'r' else ('{0: >8}'.format(bin(int(tab[-1]))[2:]).replace(" ", "0"))  # Immediate Value
    else:
        raise("Error")

    return bits

def read_instructions(file="instructions"):
    instructions = []
    with open(file, 'r') as file_instructions:
        for line in file_instructions:
            instructions.append(line.replace("\n", ""))
    return instructions


def write_instruction(instructions, file="compiled"):
    with open(file, "w") as compiled_file:
        for instruction in instructions:
            compiled_file.write(instruction)


if __name__ == "__main__":
    instructions = read_instructions()
    result = []
    for instruction in instructions:
        result.append(compile(lexer(instruction)))
    write_instruction(result)
