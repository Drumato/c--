use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::codegen::*;
use crate::assembler::arch::x64::inst::{
    inst_kind::{X64InstKind, X64Operand},
    inst_name::X64InstName,
    X64Instruction,
};

impl X64Instruction {
    pub fn new_idiv(idiv_op: X64Operand) -> Self {
        Self::new(X64InstName::IDIV, X64InstKind::UNARY(idiv_op))
    }
}

impl X64Assembler {
    pub fn generate_idivrm64_inst(codes: &mut Vec<u8>, inst: &X64Instruction) {
        // REX.W + 0xf7 /7
        // dst-operand -> r/m field in ModR/M and related b-bit in REX

        let dst_expanded_bit = Self::rex_prefix_bbit(inst.dst_expanded);
        codes.push(REX_PREFIX_BASE | REX_PREFIX_WBIT | dst_expanded_bit);

        // idiv-opcode
        codes.push(0xf7);

        // modr/m (Mだけど /7 なのでマスクする)
        let rm_field = Self::modrm_rm_field(inst.dst_regnumber);
        codes.push(MODRM_REGISTER_REGISTER | rm_field | 0x38);
    }
}

impl X64Instruction {
    pub fn change_idiv_opcode(op_size: &OperandSize, _op: &X64Operand) -> X64InstName {
        match op_size {
            // idiv r/m64
            _ => X64InstName::IDIVRM64,
        }
    }
}

#[cfg(test)]
mod call_opcode_tests {
    use super::*;
    use crate::assembler::arch::x64::file::X64AssemblyFile;
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_change_idivrm64() {
        // main:
        //   idiv r10
        let mut assembler = preprocess("main:\n  idiv r10\n");
        assembler.analyze();
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let idiv_inst = &symbol.insts[0];
            assert_eq!(X64InstName::IDIVRM64, idiv_inst.name);
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
