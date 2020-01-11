use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::{X64InstName, X64Instruction, X64Operand};

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

impl X64Instruction {
    pub fn change_add_opcode(
        op_size: &OperandSize,
        src: &X64Operand,
        dst: &X64Operand,
    ) -> X64InstName {
        match op_size {
            OperandSize::QUADWORD => {
                if dst.is_register() && src.is_immediate() {
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
mod analyze_tests {
    use super::*;
    use crate::assembler::arch::x64::file::X64AssemblyFile;
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_change_add_opcode() {
        // main:
        //   add rax, 3
        let mut assembler = preprocess("main:\n  add rax, 3\n");
        assembler.analyze();
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let add_inst = &symbol.insts[0];
            assert_eq!(X64InstName::ADDRM64IMM32, add_inst.name);
        }
    }
    fn preprocess(input: &str) -> X64Assembler {
        let target = Target::new();
        let assembly_file = AssemblyFile::new_intel_file(input.to_string(), target);
        let x64_assembly_file = X64AssemblyFile::new(assembly_file);
        let mut assembler = X64Assembler::new(x64_assembly_file);

        lex_intel::lexing_intel_syntax(&mut assembler);
        assembler.parse_intel_syntax();
        assembler
    }
}
