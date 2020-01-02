pub mod stack;
pub mod instruction;

pub struct Cpu {
    registers: [i64; 16],
    ram: [u8; 0xFFFF],
    pc: usize,
    ip: i64,
    cir: u32,
    size: usize,
    flags: [bool; 64],
    running: bool,
}

impl Cpu{
    pub fn new(registers: [i64; 16], ram: [u8; 0xFFFF]) -> Cpu {
        Cpu {
            registers,
            ram,
            flags: [false; 64],
            pc: 0,
            ip: 0,
            size: 0,
            cir: 0,
            running: false,
        }
    }

    pub fn load_data(&mut self, data: &Vec<u8>) {
        self.size = data.len();
        if data.len() <= 0xFFFF {
            let data_tmp = data.to_vec();
            for index in 0..data_tmp.len() {
                self.ram[index] = data_tmp[index]
            }
        }
    }

    pub fn run(&mut self) {
        *&mut self.running = true;
        loop {
            if self.pc >= self.size {
                return;
            }
            self.fetch();
            let instruction = self.decode();
            self.execute(instruction);
        }
    }

    fn fetch(&mut self) {
        let mar = &self.pc;
        let mdr : &mut u32 = &mut 0u32;
        for index in 0..4 {
            *mdr += (*&mut self.ram[mar + index] as u32) << (3 - index) * 8;
        }
        *&mut self.pc += 4;
        *&mut self.cir = *mdr;
    }

    fn decode(&mut self) -> instruction::Instruction {
           instruction::Instruction::new(*&mut self.cir)
    }

    fn execute(&mut self, mut instruction: instruction::Instruction) {
        let value1: i64 = self.registers[*instruction.get_op1() as usize];
        let value2: i64;
        println!("value1: {}\n", value1);
        if *instruction.get_iv_flag() {
            value2 = *instruction.get_iv_value() as i64;
        }else{
            value2 = self.registers[*instruction.get_op2() as usize];
        }
        println!("value2: {}\n", value2);
        let result = *instruction.get_dest() as usize;
        match instruction.get_opcode() {
        instruction::opcode::Opcode::AND => self.registers[result] = (value1 != 0 && value2 != 0) as i64,
        instruction::opcode::Opcode::OR => self.registers[result] = (value1 != 0 || value2 != 0) as i64,
        instruction::opcode::Opcode::EOR => self.registers[result] = ((value1 != 0) ^ (value2 != 0)) as i64,
        instruction::opcode::Opcode::ADD => {
            let tmp1 = value1 as i128;
            let tmp2 = value2 as i128;
            let res_tmp = tmp1 + tmp2;
            if res_tmp > std::i64::MAX as i128 {
                self.get_flags()[1] = true;
                self.registers[result] = (res_tmp - (std::i64::MAX as i128 + 1)) as i64;
            }else{
                self.registers[result] = res_tmp as i64;
            }
        },
        instruction::opcode::Opcode::ADC => self.registers[result] = value1 + value2,
        instruction::opcode::Opcode::SUB => self.registers[result] = value1 - value2,
        instruction::opcode::Opcode::SBC => self.registers[result] = value1 - value2,
        instruction::opcode::Opcode::MOV => self.registers[result] = value2,
        instruction::opcode::Opcode::LSH => self.registers[result] = value1 << value2,
        instruction::opcode::Opcode::RSH => self.registers[result] = value1 >> value2,
        instruction::opcode::Opcode::CMP => match instruction.get_bcc() {
            instruction::branch_condition_code::BranchConditionCode::BEQ => self.flags[0] = value1 == value2,
            instruction::branch_condition_code::BranchConditionCode::BNE => self.flags[0] = value1 != value2,
            instruction::branch_condition_code::BranchConditionCode::BLE => self.flags[0] = value1 <= value2,
            instruction::branch_condition_code::BranchConditionCode::BGE => self.flags[0] = value1 >= value2,
            instruction::branch_condition_code::BranchConditionCode::BL => self.flags[0] = value1 < value2,
            instruction::branch_condition_code::BranchConditionCode::BG => self.flags[0] = value1 > value2,
            _ => panic!("the opcode is not existing")
        },
        _ => panic!("the opcode is not existing")
        }
        println!("result: {}\n", self.registers[result]);
    }

    pub fn get_registers(&mut self) -> &[i64; 16] {
        &self.registers
    }

    pub fn get_ram(&mut self) -> &[u8; 0xFFFF] {
        &self.ram
    }

    pub fn get_ip(&mut self) -> &i64 {
        &mut self.ip
    }

    pub fn get_pc(&mut self) -> &usize {
        &mut self.pc
    }

    pub fn get_size(&mut self) -> &usize {
        &mut self.size
    }

    pub fn get_flags(&mut self) -> &mut [bool; 64] {
        &mut self.flags
    }
}