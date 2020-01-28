use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::{
    inst_kind::{X64InstKind, X64Operand},
    inst_name::X64InstName,
    X64Instruction,
};

impl X64Instruction {
    pub fn new_neg(negative_op: X64Operand) -> Self {
        Self::new(X64InstName::NEG, X64InstKind::UNARY(negative_op))
    }
    pub fn change_neg_opcode(op_size: &OperandSize, _op: &X64Operand) -> X64InstName {
        match op_size {
            // neg r/m64
            _ => X64InstName::NEGRM64,
        }
    }
}

impl X64Assembler {
    pub fn generate_negrm64_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // REX.W + 0xf7 /3
        // dst-operand -> r/m field in ModR/M and related b-bit in REX

        let dst_expanded_bit = Self::rex_prefix_bbit(inst.dst_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit);

        // neg-opcode
        codes.push(0xf7);

        // modr/m (Mだけど /3 なのでマスクする)
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | rm_field | 0x18);
    }
}
