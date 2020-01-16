use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::{
    inst_kind::X64InstKind, inst_name::X64InstName, X64Instruction,
};

impl X64Assembler {
    pub fn generate_cqo_inst(codes: &mut Vec<u8>, _inst: &X64Instruction) {
        // REX.W + 0x99
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT);
        codes.push(0x99);
    }
}

impl X64Instruction {
    pub fn new_cqo() -> Self {
        Self::new(X64InstName::CQO, X64InstKind::NOOPERAND)
    }
}
