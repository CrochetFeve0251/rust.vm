#[derive(PartialEq, Copy, Clone)]
pub enum BranchConditionCode {
    NO_BRANCH = 0x0,
    B = 0x8,
    BEQ = 0x9,
    BNE = 0xa,
    BLE = 0xb,
    BGE = 0xc,
    BL = 0xd,
    BG = 0xe,
}
///The branch condition of the instruction
impl BranchConditionCode {
    pub fn find(code: u8) -> BranchConditionCode {
        match code {
            0x8 => BranchConditionCode::B,
            0x9 => BranchConditionCode::BEQ,
            0xa => BranchConditionCode::BNE,
            0xb => BranchConditionCode::BLE,
            0xc => BranchConditionCode::BGE,
            0xd => BranchConditionCode::BL,
            0xe => BranchConditionCode::BG,
            _ => BranchConditionCode::NO_BRANCH
        }
    }
}