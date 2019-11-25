use crate::cpu::instruction::branch_condition_code::BranchConditionCode;
use crate::cpu::instruction::opcode::Opcode;

pub mod branch_condition_code;
pub mod opcode;


pub struct Instruction{
    bcc: BranchConditionCode,
    opcode: Opcode,
    iv_flag: bool,
    ope1: i8,
    ope2: i8,
    dest: i64,
    iv_value: i64,
}