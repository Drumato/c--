use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::inst::{
    inst_kind::{X64InstKind, X64Operand},
    inst_name::X64InstName,
    X64Instruction,
};

impl X64Instruction {
    pub fn new_jz(jump_op: X64Operand) -> Self {
        Self::new(X64InstName::JZ, X64InstKind::UNARY(jump_op))
    }
    pub fn change_jz_opcode(op_size: &OperandSize, _op: &X64Operand) -> X64InstName {
        match op_size {
            // jz rel32
            _ => X64InstName::JZREL32,
        }
    }
}
