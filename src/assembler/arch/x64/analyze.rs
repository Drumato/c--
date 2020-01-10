use crate::assembler::arch::x64::inst;
use crate::assembler::arch::x64::X64Assembler;
use inst::{X64InstKind, X64InstName, X64Instruction, X64OpeKind, X64Operand};

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

                // レジスタ番号を割り付ける
                self.src_regnumber = src.register_number();
                self.dst_regnumber = dst.register_number();

                // 即値も取得しておく
                self.immediate_value = src.immediate_value();

                // オペランドの種類からオペコードを一意にする.
                self.name = Self::change_binary_opcode(&self.name, &self.operand_size, src, dst);
            }
            X64InstKind::LABEL(_name) => (),
        }
    }
    fn change_binary_opcode(
        name: &X64InstName,
        size: &OperandSize,
        src: &X64Operand,
        dst: &X64Operand,
    ) -> X64InstName {
        match name {
            X64InstName::ADD => Self::change_add_opcode(size, src, dst),
            X64InstName::MOV => Self::change_mov_opcode(size, src, dst),
            // 何も変化させない
            _ => X64InstName::ADD,
        }
    }
    fn change_add_opcode(op_size: &OperandSize, src: &X64Operand, dst: &X64Operand) -> X64InstName {
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
    fn change_mov_opcode(op_size: &OperandSize, src: &X64Operand, dst: &X64Operand) -> X64InstName {
        match op_size {
            OperandSize::QUADWORD => {
                if dst.is_register() && src.is_immediate() {
                    // mov r/m64, imm32
                    return X64InstName::MOVRM64IMM32;
                }

                if dst.is_register() && src.is_register() {
                    // mov r/m64, r64
                    return X64InstName::MOVRM64R64;
                }
                X64InstName::MOV
            }
            // 何も変化させない
            _ => X64InstName::MOV,
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
    fn is_register(&self) -> bool {
        match &self.kind {
            X64OpeKind::REG(_n) => true,
            _ => false,
        }
    }
    fn is_immediate(&self) -> bool {
        match &self.kind {
            X64OpeKind::INTEGER(_v) => true,
            _ => false,
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
    fn immediate_value(&self) -> i128 {
        if let X64OpeKind::INTEGER(val) = &self.kind {
            *val
        } else {
            0
        }
    }
    fn register_number(&self) -> usize {
        if let X64OpeKind::REG(name) = &self.kind {
            match name.as_str() {
                "al" | "ax" | "eax" | "rax" | "r8" => 0,
                "cl" | "cx" | "ecx" | "rcx" | "r9" => 1,
                "dl" | "dx" | "edx" | "rdx" | "r10" => 2,
                "bl" | "bx" | "ebx" | "rbx" | "r11" => 3,
                "ah" | "sp" | "esp" | "rsp" | "r12" => 4,
                "ch" | "bp" | "ebp" | "rbp" | "r13" => 5,
                "dh" | "si" | "esi" | "rsi" | "r14" => 6,
                "bh" | "di" | "edi" | "rdi" | "r15" => 7,
                _ => 0,
            }
        } else {
            0
        }
    }
}

#[cfg(test)]
mod analyze_tests {
    use super::*;
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::assembler::arch::x64::X64AssemblyFile;
    use crate::structure::AssemblyFile;
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
