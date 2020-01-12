use crate::assembler::arch::x64::analyze::OperandSize;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::{
    inst_kind::X64Operand, inst_name::X64InstName, X64Instruction,
};

impl X64Assembler {
    pub fn generate_callrm64_inst(codes: &mut Vec<u8>, _inst: &X64Instruction) {
        // call-opcode
        codes.push(0xff);

        // call - register
        codes.push(0xd0);
    }
}

impl X64Instruction {
    pub fn change_call_opcode(op_size: &OperandSize, _op: &X64Operand) -> X64InstName {
        match op_size {
            // call r/m64
            _ => X64InstName::CALLRM64,
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
    fn test_change_callrm64() {
        // main:
        //   call foo
        let mut assembler = preprocess("main:\n  call foo\n");
        assembler.analyze();
        if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
            let call_inst = &symbol.insts[0];
            assert_eq!(X64InstName::CALLRM64, call_inst.name);
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
