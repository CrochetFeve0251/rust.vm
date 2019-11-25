use crate::cpu::instruction::branch_condition_code::BranchConditionCode;
use crate::cpu::instruction::opcode::Opcode;

pub mod branch_condition_code;
pub mod opcode;


pub struct Instruction{
    bcc: BranchConditionCode,
    opcode: Opcode,
    iv_flag: bool,
    ope1: u8,
    ope2: u8,
    dest: u8,
    iv_value: u8,
}

impl Instruction {
    pub fn new(code: u32) -> Instruction {
        let bytes = code.to_be_bytes();
        let bcc = BranchConditionCode::find(Instruction::get_high_bytes(bytes[0]));
        let iv_flag= Instruction::get_low_bytes(bytes[0]) as bool;
        let opcode =  Opcode::find(Instruction::get_high_bytes(bytes[1]));
        let ope1 = Instruction::get_low_bytes(bytes[1]);
        let ope2 = Instruction::get_high_bytes(bytes[2]);
        let dest = Instruction::get_low_bytes(bytes[2]);
        let iv_value = bytes[3];
        Instruction {
            bcc,
            iv_flag,
            opcode,
            ope1,
            ope2,
            dest,
            iv_value
        }
    }

    fn get_low_bytes(value: u8) -> u8 {
        let tmp = value >> 4;
        tmp << 4
    }

    fn get_high_bytes(value: u8) -> u8 {
        let tmp = value << 4;
        tmp
    }

    pub fn get_bcc(&mut self) -> &mut BranchConditionCode {
        &mut self.bcc
    }

    pub fn get_opcode(&mut self) -> &mut Opcode {
        &mut self.opcode
    }

    pub fn get_iv_flag(&mut self) -> &mut bool {
        &mut self.iv_flag
    }

    pub fn get_op1(&mut self) -> &u8 {
        &mut self.ope1
    }

    pub fn get_op2(&mut self) -> &u8 {
        &mut self.ope2
    }

    pub fn get_dest(&mut self) -> &u8 {
        &mut self.dest
    }

    pub fn get_iv_value(&mut self) -> &u8 {
        &mut self.iv_value
    }
}
