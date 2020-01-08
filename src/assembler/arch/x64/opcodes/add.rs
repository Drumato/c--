use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::X64Instruction;
use crate::assembler::arch::x64::X64Assembler;

impl X64Assembler {
    pub fn generate_addrm64imm32_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. add rax, 3
        // dst-operand -> r/m field in ModR/M and related r-bit in REX
        // 本当はb-bitだけど,Op/En がMI なので r-bitに関係する
        // rex-prefix
        let dst_expanded_bit = Self::rex_prefix_rbit(inst.dst_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit);

        // opcode
        codes.push(0x81);

        // modr/m (MI)
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | rm_field);

        // immediate-value
        for b in (inst.immediate_value as u32).to_le_bytes().to_vec().iter() {
            codes.push(*b);
        }
    }
    pub fn generate_addrm64r64_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. add rax, r15
        // dst-operand -> r/m field in ModR/M and related r-bit in REX cuz ModR/M(MR)
        // src-operand -> reg field in ModR/M and related b-bit in REX cuz ModR/M(MR)
        // rex-prefix
        let dst_expanded_bit = Self::rex_prefix_rbit(inst.dst_expanded);
        let src_expanded_bit = Self::rex_prefix_bbit(inst.src_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit | src_expanded_bit);

        // opcode
        codes.push(0x01);

        // modr/m (MR)
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        let reg_field = Self::modrm_reg_field(inst.src_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | reg_field | rm_field);
    }
}
