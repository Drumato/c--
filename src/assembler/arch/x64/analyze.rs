use crate::assembler::arch::x64::inst::{X64InstKind, X64Instruction, X64OpeKind, X64Operand};
use crate::assembler::arch::x64::X64Assembler;

#[derive(PartialEq, Debug, Clone)]
pub enum OperandSize {
    BYTE,       // 8bit
    WORD,       // 16bit
    DOUBLEWORD, // 32bit
    QUADWORD,   // 64bit
    UNKNOWN,
}

impl X64Assembler {
    pub fn analyze(&mut self) {
        for (_name, symbol) in self.src_file.symbols_map.iter_mut() {
            for inst in symbol.insts.iter_mut() {
                inst.analyze_operand();
            }
        }
    }
}

impl X64Instruction {
    fn analyze_operand(&mut self) {
        match &self.kind {
            X64InstKind::NOOPERAND => (),
            X64InstKind::UNARY(op) => {
                self.operand_size = op.check_operand_size();
                self.dst_expanded = op.check_used_register_is_expand();
            }
            X64InstKind::BINARY(src, dst) => {
                // dstに数値リテラルが来ることは無い.
                // dstのみチェックすれば,比較的簡単にチェック可能.
                self.operand_size = dst.check_operand_size();

                // r8~r15を使っているかチェック
                self.src_expanded = src.check_used_register_is_expand();
                self.dst_expanded = dst.check_used_register_is_expand();
            }
            X64InstKind::LABEL(_name) => (),
        }
    }
}

impl X64Operand {
    fn check_operand_size(&self) -> OperandSize {
        match &self.kind {
            X64OpeKind::REG(name) => Self::check_register_name(name),
            _ => OperandSize::UNKNOWN,
        }
    }
    fn check_register_name(name: &String) -> OperandSize {
        match name.as_str() {
            // 64bit registers
            "rax" | "rcx" | "rdx" | "rbx" | "rsp" | "rbp" | "rsi" | "rdi" | "r8" | "r9" | "r10"
            | "r11" | "r12" | "r13" | "r14" | "r15" => OperandSize::QUADWORD,
            // 32bit registers
            "eax" | "ecx" | "edx" | "ebx" | "esp" | "ebp" | "esi" | "edi" | "r8d" | "r9d"
            | "r10d" | "r11d" | "r12d" | "r13d" | "r14d" | "r15d" => OperandSize::DOUBLEWORD,
            // 16bit registers
            "ax" | "cx" | "dx" | "bx" | "sp" | "bp" | "si" | "di" | "r8w" | "r9w" | "r10w"
            | "r11w" | "r12w" | "r13w" | "r14w" | "r15w" => OperandSize::WORD,
            // 8bit registers
            "ah" | "al" | "ch" | "cl" | "dh" | "dl" | "bh" | "bl" | "spl" | "bpl" | "sil"
            | "dil" | "r8b" | "r9b" | "r10b" | "r11b" | "r12b" | "r13b" | "r14b" | "r15b" => {
                OperandSize::BYTE
            }

            _ => OperandSize::UNKNOWN,
        }
    }
    fn check_used_register_is_expand(&self) -> bool {
        if let X64OpeKind::REG(name) = &self.kind {
            // 2文字目が数字じゃなければ非拡張レジスタ,数字なら拡張レジスタ
            (name.as_bytes()[1] as char).is_ascii_digit()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod analyze_tests {
    use super::*;
    use crate::assembler::arch::x64::lex_intel;
    use crate::assembler::arch::x64::X64AssemblyFile;
    use crate::structure::{AssemblyFile, Syntax};
    use crate::target::Target;

    #[test]
    fn test_operand_size_checker() {
        // .global main
        // main:
        //   mov rax, 3
        //   mov r8, 3
        //   add rax, r8
        //   ret
        let mut assembler =
            preprocess(".global main\nmain:\n  mov rax, 3\n mov r8, 3\n add rax, r8\n  ret\n");
        assembler.analyze();

        // dstオペランドの拡張レジスタケース
        let expect_dst_expanded = vec![false, true, false];

        for (_name, symbol) in assembler.src_file.symbols_map.iter() {
            for (i, inst) in symbol.insts.iter().enumerate() {
                match &inst.kind {
                    X64InstKind::BINARY(_src, _dst) => {
                        assert_eq!(OperandSize::QUADWORD, inst.operand_size);
                        assert_eq!(expect_dst_expanded[i], inst.dst_expanded);
                    }
                    _ => (),
                }
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
        assembler
    }
}
