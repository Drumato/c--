use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::{
    inst_kind::X64Operand, inst_name::X64InstName, X64Instruction,
};

impl X64Assembler {
    pub fn generate_subrm64imm32_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. sub rax, 3
        // dst-operand -> r/m field in ModR/M and related r-bit in REX
        // 本当はb-bitだけど,Op/En がMI なので r-bitに関係する
        // rex-prefix
        let dst_expanded_bit = Self::rex_prefix_rbit(inst.dst_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit);

        // opcode
        codes.push(0x81);

        // modr/m (MI だけど /5なのでマスクする )
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | rm_field | 0x28);

        // immediate-value
        for b in (inst.immediate_value as u32).to_le_bytes().to_vec().iter() {
            codes.push(*b);
        }
    }
    pub fn generate_subrm64r64_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. sub rax, r15
        // dst-operand -> r/m field in ModR/M and related r-bit in REX cuz ModR/M(MR)
        // src-operand -> reg field in ModR/M and related b-bit in REX cuz ModR/M(MR)
        // rex-prefix
        let dst_expanded_bit = Self::rex_prefix_rbit(inst.dst_expanded);
        let src_expanded_bit = Self::rex_prefix_bbit(inst.src_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit | src_expanded_bit);

        // opcode
        codes.push(0x29);

        // modr/m (MR)
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        let reg_field = Self::modrm_reg_field(inst.src_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | reg_field | rm_field);
    }
}

impl X64Instruction {
    pub fn change_sub_opcode(
        op_size: &OperandSize,
        src: &X64Operand,
        dst: &X64Operand,
    ) -> X64InstName {
        match op_size {
            OperandSize::QUADWORD => {
                if dst.is_register() && src.is_immediate() {
                    // sub r/m64, imm32
                    return X64InstName::SUBRM64IMM32;
                }

                if dst.is_register() && src.is_register() {
                    // sub r/m64, r64
                    return X64InstName::SUBRM64R64;
                }
                X64InstName::SUB
            }
            // 何も変化させない
            _ => X64InstName::SUB,
        }
    }
}

#[cfg(test)]
mod sub_opcode_tests {
    use super::*;
    use crate::assembler::arch::x64::file::X64AssemblyFile;
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_generate_subrm64r64() {
        let expected: Vec<u8> = vec![0x48, 0x29, 0xd8];
        // sub rax, rbx
        let mut assembler = preprocess("main:\n  sub rax, rbx\n");
        assembler.codegen();

        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            for (i, b) in expected.iter().enumerate() {
                assert_eq!(symbol.codes[i], *b);
            }
        }
    }
    #[test]
    fn test_generate_subrm64imm32() {
        let expected: Vec<u8> = vec![0x48, 0x81, 0xe8, 0x1e, 0x00, 0x00, 0x00];
        // sub rax, 30
        let mut assembler = preprocess("main:\n  sub rax, 30\n");
        assembler.codegen();

        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            for (i, b) in expected.iter().enumerate() {
                assert_eq!(symbol.codes[i], *b);
            }
        }
    }

    #[test]
    fn test_change_subrm64imm32() {
        // main:
        //   sub rax, 3
        let assembler = preprocess("main:\n  sub rax, 3\n");
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let add_inst = &symbol.insts[0];
            assert_eq!(X64InstName::SUBRM64IMM32, add_inst.name);
        }
    }
    #[test]
    fn test_change_subrm64r64() {
        // main:
        //   sub rax, rbx
        let assembler = preprocess("main:\n  sub rax, rbx\n");
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let add_inst = &symbol.insts[0];
            assert_eq!(X64InstName::SUBRM64R64, add_inst.name);
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
