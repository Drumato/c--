use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::{
    inst_kind::X64InstKind, inst_name::X64InstName, X64Instruction,
};

impl X64Instruction {
    pub fn new_ret() -> Self {
        Self::new(X64InstName::RET, X64InstKind::NOOPERAND)
    }
}

impl X64Assembler {
    pub fn generate_ret_inst(codes: &mut Vec<u8>, _inst: &X64Instruction) {
        codes.push(0xc3);
    }
}
