use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::{
    inst_kind::{X64InstKind, X64Operand},
    inst_name::X64InstName,
    X64Instruction,
};

impl X64Instruction {
    pub fn new_pop(pop_op: X64Operand) -> Self {
        Self::new(X64InstName::POP, X64InstKind::UNARY(pop_op))
    }
}

impl X64Instruction {
    pub fn change_pop_opcode(op_size: &OperandSize, _op: &X64Operand) -> X64InstName {
        match op_size {
            // pop r64
            _ => X64InstName::POPR64,
        }
    }
}

impl X64Assembler {
    pub fn generate_popr64_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // pop r64 -> pop opcode と 引数のレジスタ番号
        let op_reg_number = Self::modrm_rm_field(inst.dst_regnumber);
        codes.push(0x58 | op_reg_number);
    }
}
