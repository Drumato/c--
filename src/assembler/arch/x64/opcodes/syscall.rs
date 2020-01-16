use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::{
    inst_kind::X64InstKind, inst_name::X64InstName, X64Instruction,
};

impl X64Instruction {
    pub fn new_syscall() -> Self {
        Self::new(X64InstName::SYSCALL, X64InstKind::NOOPERAND)
    }
}

impl X64Assembler {
    pub fn generate_syscall_inst(codes: &mut Vec<u8>) {
        // syscall-opcode
        codes.push(0x0f);
        codes.push(0x05);
    }
}
