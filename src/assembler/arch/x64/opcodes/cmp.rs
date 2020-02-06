use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::{
    inst_kind::{X64InstKind, X64Operand},
    inst_name::X64InstName,
    X64Instruction,
};

impl X64Assembler {
    pub fn generate_cmprm64imm32_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. cmp rax, 0
        // dst-operand -> r/m field in ModR/M and related b-bit in REX
        // rex-prefix

        let dst_expanded_bit = Self::rex_prefix_bbit(inst.dst_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit);

        // opcode
        codes.push(0x81);

        // modr/m (MIだけど /7 なのでマスクする)
        // オフセットが設定されている -> アドレッシング方法が異なる
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        if inst.store_offset != 0 {
            codes.push(MODRM_REGISTER_DISPLACEMENT8 | rm_field | 0x38);
        } else {
            codes.push(MODRM_REGISTER_REGISTER | rm_field | 0x38);
        }

        // displacement
        // もしoffsetが設定されていれば加える
        // TODO: 今はマイナスに決め打ち
        if inst.store_offset != 0 {
            codes.push((-inst.store_offset) as u8);
        }

        // immediate-value
        for b in (inst.immediate_value as u32).to_le_bytes().to_vec().iter() {
            codes.push(*b);
        }
    }
    pub fn generate_cmprm64r64_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. cmp rax, r15
        // dst-operand -> r/m field in ModR/M and related b-bit
        // src-operand -> reg field in ModR/M and related r-bit
        // rex-prefix
        let dst_expanded_bit = Self::rex_prefix_bbit(inst.dst_expanded);
        let src_expanded_bit = Self::rex_prefix_rbit(inst.src_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit | src_expanded_bit);

        // opcode
        codes.push(0x39);

        // modr/m (MR)
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        let reg_field = Self::modrm_reg_field(inst.src_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | reg_field | rm_field);
    }
}

impl X64Instruction {
    pub fn new_cmp(src: X64Operand, dst: X64Operand) -> Self {
        Self::new(X64InstName::CMP, X64InstKind::BINARY(src, dst))
    }
}

impl X64Instruction {
    pub fn change_cmp_opcode(
        op_size: &OperandSize,
        src: &X64Operand,
        dst: &X64Operand,
    ) -> X64InstName {
        match op_size {
            OperandSize::QUADWORD => {
                if dst.is_register() && src.is_immediate()
                    || dst.is_addressing() && src.is_immediate()
                {
                    // cmp r/m64, imm32
                    return X64InstName::CMPRM64IMM32;
                }

                if dst.is_register() && src.is_register()
                    || dst.is_addressing() && src.is_register()
                {
                    // cmp r/m64, r64
                    return X64InstName::CMPRM64R64;
                }
                X64InstName::CMP
            }
            // 何も変化させない
            _ => X64InstName::CMP,
        }
    }
}
