use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::{
    inst_kind::{X64InstKind, X64Operand},
    inst_name::X64InstName,
    X64Instruction,
};

impl X64Instruction {
    pub fn new_add(src: X64Operand, dst: X64Operand) -> Self {
        Self::new(X64InstName::ADD, X64InstKind::BINARY(src, dst))
    }
}

impl X64Assembler {
    pub fn generate_addrm64imm32_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. add r10, 3
        // dst-operand -> r/m field in ModR/M and related b-bit in REX
        // rex-prefix
        let dst_expanded_bit = Self::rex_prefix_bbit(inst.dst_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit);

        // opcode
        codes.push(0x81);

        // modr/m (MI)
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);

        // オフセットが設定されている -> アドレッシング方法が異なる
        if inst.store_offset != 0 {
            codes.push(MODRM_REGISTER_DISPLACEMENT8 | rm_field);
        } else {
            codes.push(MODRM_REGISTER_REGISTER | rm_field);
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
    pub fn generate_addrm64r64_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // e.g. add rax, r15
        // dst-operand -> r/m field in ModR/M and related b-bit
        // src-operand -> reg field in ModR/M and related r-bit
        // rex-prefix
        let dst_expanded_bit = Self::rex_prefix_bbit(inst.dst_expanded);
        let src_expanded_bit = Self::rex_prefix_rbit(inst.src_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit | src_expanded_bit);

        // opcode
        codes.push(0x01);

        // modr/m (MR)
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        let reg_field = Self::modrm_reg_field(inst.src_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | reg_field | rm_field);
    }
}

impl X64Instruction {
    pub fn change_add_opcode(
        op_size: &OperandSize,
        src: &X64Operand,
        dst: &X64Operand,
    ) -> X64InstName {
        match op_size {
            OperandSize::QUADWORD => {
                if dst.is_register() && src.is_immediate()
                    || dst.is_addressing() && src.is_immediate()
                {
                    // add r/m64, imm32
                    return X64InstName::ADDRM64IMM32;
                }

                if dst.is_register() && src.is_register() {
                    // add r/m64, r64
                    return X64InstName::ADDRM64R64;
                }
                X64InstName::ADD
            }
            // 何も変化させない
            _ => X64InstName::ADD,
        }
    }
}

#[cfg(test)]
mod add_opcode_tests {
    use super::*;
    use crate::assembler::arch::x64::file::X64AssemblyFile;
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_change_addrm64imm32() {
        // main:
        //   add rax, 3
        let assembler = preprocess("main:\n  add rax, 3\n");
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let add_inst = &symbol.insts[0];
            assert_eq!(X64InstName::ADDRM64IMM32, add_inst.name);
        }
    }
    #[test]
    fn test_change_addrm64r64() {
        // main:
        //   add rax, rbx
        let assembler = preprocess("main:\n  add rax, rbx\n");
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let add_inst = &symbol.insts[0];
            assert_eq!(X64InstName::ADDRM64R64, add_inst.name);
        }
    }

    #[test]
    fn test_generate_addrm64imm32() {
        let expected: Vec<u8> = vec![0x49, 0x81, 0xc2, 0x1e, 0x00, 0x00, 0x00];
        // add r10, 30
        let mut assembler = preprocess("main:\n  add r10, 30\n");
        assembler.codegen();

        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            for (i, b) in expected.iter().enumerate() {
                assert_eq!(symbol.codes[i], *b);
            }
        }
    }

    #[test]
    fn test_generate_addrm64r64() {
        let expected: Vec<u8> = vec![0x48, 0x01, 0xd8];
        // add rax, rbx
        let mut assembler = preprocess("main:\n  add rax, rbx\n");
        assembler.codegen();

        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            for (i, b) in expected.iter().enumerate() {
                assert_eq!(symbol.codes[i], *b);
            }
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
