pub enum Opcode {
    AND = 0x0,
    OR = 0x1,
    EOR = 0x2,
    ADD = 0x3,
    ADC = 0x4,
    CMP = 0x5,
    SUB = 0x6,
    SBC = 0x7,
    MOV = 0x8,
    LSH = 0x9,
    RSH= 0xa,
}

impl Opcode {
    pub fn find(code: u8) -> Opcode {
        match code {
            0x0 => Opcode::AND,
            0x1 => Opcode::OR,
            0x2 => Opcode::EOR,
            0x3 => Opcode::ADD,
            0x4 => Opcode::ADC,
            0x5 => Opcode::CMP,
            0x6 => Opcode::SUB,
            0x7 => Opcode::SBC,
            0x8 => Opcode::MOV,
            0x9 => Opcode::LSH,
            0xa => Opcode::RSH,
            _ => panic!("the opcode is not existing")
        }
    }
}