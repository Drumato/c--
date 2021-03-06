use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::{
    inst_kind::{X64InstKind, X64Operand},
    inst_name::X64InstName,
    X64Instruction,
};

impl X64Instruction {
    pub fn new_imul(src: X64Operand, dst: X64Operand) -> Self {
        Self::new(X64InstName::IMUL, X64InstKind::BINARY(src, dst))
    }
}

impl X64Assembler {
    pub fn generate_imulr64rm64imm32_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. imul rax, 3
        // dst-operand -> reg field in ModR/M and related r-bit in REX
        // src1-operand -> r/m field in ModR/M and related b-bit in REX
        // immediate-operand -> ModR/M,Rexともに影響なし

        // rex-prefix
        // IMULは rax <- rax * 3 となるので,srcとdstに同じレジスタを用いる
        let dst_expanded_bit = Self::rex_prefix_rbit(inst.dst_expanded);
        let src_expanded_bit = Self::rex_prefix_bbit(inst.dst_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit | src_expanded_bit);

        // opcode
        codes.push(0x69);

        // ModR/M(RMI)
        // IMULは rax <- rax * 3 となるので,srcとdstに同じレジスタを用いる
        let reg_field = Self::modrm_reg_field(inst.dst_regnumber);
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | reg_field | rm_field);

        // immediate-value
        for b in (inst.immediate_value as u32).to_le_bytes().to_vec().iter() {
            codes.push(*b);
        }
    }
    pub fn generate_imulr64rm64_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. imul rax, r15
        // dst-operand -> reg field in ModR/M and related r-bit in REX
        // src1-operand -> r/m field in ModR/M and related b-bit in REX

        // rex-prefix
        let dst_expanded_bit = Self::rex_prefix_rbit(inst.dst_expanded);
        let src_expanded_bit = Self::rex_prefix_bbit(inst.src_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit | src_expanded_bit);

        // opcode
        codes.push(0x0f);
        codes.push(0xaf);

        // ModR/M(RM)
        let reg_field = Self::modrm_reg_field(inst.dst_regnumber);
        let rm_field = Self::modrm_rm_field(inst.src_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | reg_field | rm_field);
    }
}

impl X64Instruction {
    pub fn change_imul_opcode(
        op_size: &OperandSize,
        src: &X64Operand,
        dst: &X64Operand,
    ) -> X64InstName {
        match op_size {
            OperandSize::QUADWORD => {
                if dst.is_register() && src.is_immediate() {
                    // imul r64, r/m64, imm32
                    return X64InstName::IMULR64RM64IMM32;
                }

                if dst.is_register() && src.is_register() {
                    // imul r64, r/m64
                    return X64InstName::IMULR64RM64;
                }
                X64InstName::IMUL
            }
            // 何も変化させない
            _ => X64InstName::IMUL,
        }
    }
}

#[cfg(test)]
mod imul_opcode_tests {
    use super::*;
    use crate::assembler::arch::x64::file::X64AssemblyFile;
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_change_imulr64rm64imm32() {
        // main:
        //   imul rax, 3
        let assembler = preprocess("main:\n  imul rax, 3\n");
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let add_inst = &symbol.insts[0];
            assert_eq!(X64InstName::IMULR64RM64IMM32, add_inst.name);
        }
    }
    #[test]
    fn test_change_imulr64rm64() {
        // main:
        //   imul rax, rbx
        let assembler = preprocess("main:\n  imul rax, rbx\n");
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let add_inst = &symbol.insts[0];
            assert_eq!(X64InstName::IMULR64RM64, add_inst.name);
        }
    }

    fn preprocess(input: &str) -> X64Assembler {
        let target = Target::new();
        let assembly_file = AssemblyFile::new_intel_file(input.to_string(), target);
        let x64_assembly_file = X64AssemblyFile::new(assembly_file);
        let mut assembler = X64Assembler::new(x64_assembly_file);

        lex_intel::lexing_intel_syntax(&mut assembler);
        assembler.parse_intel_syntax();
        assembler.analyze();
        assembler
    }
}
