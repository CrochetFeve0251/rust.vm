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
    ///create a new Cpu
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
    ///load the data of the program into the virtual ram
    pub fn load_data(&mut self, data: &Vec<u8>) {
        self.size = data.len();
        if data.len() <= 0xFFFF {
            let data_tmp = data.to_vec();
            for index in 0..data_tmp.len() {
                self.ram[index] = data_tmp[index]
            }
        }
    }
    ///run the cpu
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
    ///Fetch the instruction from the ram and change the pc
    fn fetch(&mut self) {
        let mar = &self.pc;
        let mdr : &mut u32 = &mut 0u32;
        for index in 0..4 {
            *mdr += (*&mut self.ram[mar + index] as u32) << (3 - index) * 8;
        }
        *&mut self.cir = *mdr;
    }
    ///Decode the instruction
    fn decode(&mut self) -> instruction::Instruction {
          let instruction = instruction::Instruction::new(*&mut self.cir);
        *&mut self.pc = match instruction {
            instruction::Instruction::BranchInstruction { bcc, is_positive, offset } => {
                match bcc {
                    instruction::branch_condition_code::BranchConditionCode::B => *&mut self.pc + 4,
                    instruction::branch_condition_code::BranchConditionCode::BEQ => if self.flags[3] {
                        if is_positive {
                            *&mut self.pc - offset as usize
                        }else{
                            *&mut self.pc + offset as usize
                        }
                    }else{
                        *&mut self.pc + 4
                    },
                    instruction::branch_condition_code::BranchConditionCode::BG => if self.flags[8] {
                        if is_positive {
                            *&mut self.pc - offset as usize
                        }else{
                            *&mut self.pc + offset as usize
                        }
                    }else{
                        *&mut self.pc + 4
                    },
                    instruction::branch_condition_code::BranchConditionCode::BGE => if self.flags[6] {
                        if is_positive {
                            *&mut self.pc - offset as usize
                        }else{
                            *&mut self.pc + offset as usize
                        }
                    }else{
                        *&mut self.pc + 4
                    },
                    instruction::branch_condition_code::BranchConditionCode::BL => if self.flags[7] {
                        if is_positive {
                            *&mut self.pc - offset as usize
                        }else{
                            *&mut self.pc + offset as usize
                        }
                    }else{
                        *&mut self.pc + 4
                    },
                    instruction::branch_condition_code::BranchConditionCode::BLE => if self.flags[5] {
                        if is_positive {
                            *&mut self.pc - offset as usize
                        }else{
                            *&mut self.pc + offset as usize
                        }
                    }else{
                        *&mut self.pc + 4
                    },
                    instruction::branch_condition_code::BranchConditionCode::BNE => if self.flags[4] {
                        if is_positive {
                            *&mut self.pc - offset as usize
                        }else{
                            *&mut self.pc + offset as usize
                        }
                    }else{
                        *&mut self.pc + 4
                    },
                }
            },
            instruction::Instruction::OperationInstruction { opcode: _, iv_flag: _, ope1: _, ope2: _, dest: _, iv_value: _} => {
                *&mut self.pc + 4
            },
            instruction::Instruction::ErrorInstruction => {
                *&mut self.pc + 4
            }
        };
        self.reset_flags();
        instruction
    }

    fn reset_flags(&mut self){
        for index in 0..self.flags.len() {
            self.flags[index] = false;
        }
    }

    ///Execute the current instruction
    fn execute(&mut self, instruction: instruction::Instruction) {
        match instruction {
            instruction::Instruction::OperationInstruction {opcode, iv_flag, ope1, ope2, dest, iv_value} => {
                let value1: i64 = self.registers[ope1 as usize];
                let value2: i64;
                println!("value1: {}\n", value1);
                if iv_flag {
                    value2 = iv_value as i64;
                }else{
                    value2 = self.registers[ope2 as usize];
                }
                println!("value2: {}\n", value2);
                let result = dest as usize;
                match opcode {
                    instruction::opcode::Opcode::AND => self.registers[result] = (value1 != 0 && value2 != 0) as i64,
                    instruction::opcode::Opcode::OR => self.registers[result] = (value1 != 0 || value2 != 0) as i64,
                    instruction::opcode::Opcode::EOR => self.registers[result] = ((value1 != 0) ^ (value2 != 0)) as i64,
                    instruction::opcode::Opcode::ADD => {
                        let tmp1 = value1 as i128;
                        let tmp2 = value2 as i128;
                        let res_tmp = tmp1 + tmp2;
                        self.registers[result] = if res_tmp > std::i64::MAX as i128 {
                            self.get_flags()[1] = true;
                            (res_tmp - (std::i64::MAX as i128 + 1)) as i64
                        } else {
                            res_tmp as i64
                        }
                    },
                    instruction::opcode::Opcode::ADC => {
                        let tmp1 = value1 as i128;
                        let tmp2 = value2 as i128;
                        let res_tmp = tmp1 + tmp2 + self.get_flags()[1] as i128;
                        self.get_flags()[1] = false;
                        self.registers[result] = if res_tmp > std::i64::MAX as i128 {
                            self.get_flags()[1] = true;
                            (res_tmp - (std::i64::MAX as i128 + 1)) as i64
                        } else {
                            res_tmp as i64
                        }
                    },
                    instruction::opcode::Opcode::SUB => {
                        let tmp1 = value1 as i128;
                        let tmp2 = value2 as i128;
                        let res_tmp = tmp1 - tmp2;
                        self.registers[result] = if res_tmp < std::i64::MIN as i128 {
                            self.get_flags()[2] = true;
                            (res_tmp + (std::i64::MIN as i128 + 1)) as i64
                        } else {
                            res_tmp as i64
                        }
                    },
                    instruction::opcode::Opcode::SBC => {
                        let tmp1 = value1 as i128;
                        let tmp2 = value2 as i128;
                        let res_tmp = tmp1 - tmp2 + self.get_flags()[2] as i128 - 1;
                        self.registers[result] = if res_tmp < std::i64::MIN as i128 {
                            self.get_flags()[2] = true;
                            (res_tmp + (std::i64::MIN as i128 + 1)) as i64
                        } else {
                            res_tmp as i64
                        }
                    },
                    instruction::opcode::Opcode::MOV => self.registers[result] = value2,
                    instruction::opcode::Opcode::LSH => self.registers[result] = value1 << value2,
                    instruction::opcode::Opcode::RSH => self.registers[result] = value1 >> value2,
                    instruction::opcode::Opcode::CMP => {
                        self.flags[3] = value1 == value2;
                        self.flags[4] = value1 != value2;
                        self.flags[5] = value1 <= value2;
                        self.flags[6] = value1 >= value2;
                        self.flags[7] = value1 < value2;
                        self.flags[8] = value1 > value2;
                    }
                }
            },
            instruction::Instruction::BranchInstruction { bcc: _, is_positive: _, offset: _ } => {}
            instruction::Instruction::ErrorInstruction => {},
        }
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