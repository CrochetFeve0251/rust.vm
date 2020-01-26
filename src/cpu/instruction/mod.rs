use crate::cpu::instruction::branch_condition_code::BranchConditionCode;
use crate::cpu::instruction::opcode::Opcode;
use ::std::println;

pub mod branch_condition_code;
pub mod opcode;

#[derive(Copy, Clone)]
pub enum Instruction{
    BranchInstruction{
        bcc: BranchConditionCode,
        is_positive: bool,
        offset: i128,
    },
    OperationInstruction{
        opcode: Opcode,
        iv_flag: bool,
        ope1: u8,
        ope2: u8,
        dest: u8,
        iv_value: u8,
    },
    ErrorInstruction
}
///instruction containing all the needed information to be executed
impl Instruction {
    pub fn new(code: u32) -> Instruction {
        let bytes = code.to_be_bytes();
        let bcc = BranchConditionCode::find(Instruction::get_high_bytes(bytes[0]));
        if bcc == BranchConditionCode::NO_BRANCH {
            let iv_flag= Instruction::get_low_bytes(bytes[0]) != 0;
            let opcode = Opcode::find(Instruction::get_high_bytes(bytes[1]));
            let ope1 = Instruction::get_low_bytes(bytes[1]);
            let ope2 = Instruction::get_high_bytes(bytes[2]);
            let dest = Instruction::get_low_bytes(bytes[2]);
            let iv_value = bytes[3];
            Instruction::OperationInstruction {
                iv_flag,
                opcode,
                ope1,
                ope2,
                dest,
                iv_value
            }
        }else{
            let is_positive = Instruction::get_is_positive_bit(bytes[0]);
            let offset = Instruction::get_offset(bytes);
            Instruction::BranchInstruction {
                bcc,
                is_positive,
                offset
            }
        }

    }
    ///get the high bits of a byte
    fn get_high_bytes(value: u8) -> u8 {
        let tmp = value >> 4;
        tmp
    }
    ///get the low bits of a byte
    fn get_low_bytes(value: u8) -> u8 {
        let tmp = value << 4;
        tmp >> 4
    }
    ///get the positive bit to know if the offset of the branch is negative or positive
    fn get_is_positive_bit(value: u8) -> bool {
        let tmp = value << 4;
        (tmp >> 7) != 0
    }
    ///get the offset of the branch
    fn get_offset(bytes: [u8; 4]) -> i128 {
        let mut sum= 0;
        for index in 1..4 {
            sum += (bytes[4 - index] as i128) << (index - 1) * 8;
        }
        let tmp = bytes[1] >> 1;
        sum + (tmp << 1) as i128
    }
}
