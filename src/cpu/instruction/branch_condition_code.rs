#[derive(PartialEq, Copy, Clone)]
pub enum BranchConditionCode {
    B = 0x8,
    BEQ = 0x9,
    BNE = 0xa,
    BLE = 0xb,
    BGE = 0xc,
    BL = 0xd,
    BG = 0xe,
}

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
            _ => panic!("the bcc is not existing")
        }
    }
}