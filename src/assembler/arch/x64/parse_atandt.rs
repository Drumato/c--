use crate::assembler::arch::x64::inst::{X64Instruction, X64Operand};
use crate::assembler::arch::x64::{asmtoken, X64Assembler, X64Symbol};
use asmtoken::AsmTokenKind;

impl X64Assembler {
    pub fn parse_atandt_syntax(&mut self) {
        // トップレベルの関数.
        // ディレクティブを全て処理してしまう.
        loop {
            let cur = self.looking_token_clone();
            match cur.kind {
                AsmTokenKind::DIRECTIVE(name) => self.parse_directive(name, cur.position),
                AsmTokenKind::LABEL(name) => {
                    // オフセットを進める
                    self.read_token();

                    // parse_inst()で命令を取り続ける
                    let mut insts_in_label: Vec<X64Instruction> = Vec::new();
                    while let Some(inst) = self.parse_inst_atandt_syntax() {
                        insts_in_label.push(inst);
                    }

                    // グローバルシンボルの定義がされていればすでにエントリがある
                    if let Some(symbol) = self.src_file.symbols_map.get_mut(&name) {
                        symbol.insts = insts_in_label;
                        continue;
                    }

                    // シンボルマップにエントリを登録
                    let mut local_symbol = X64Symbol::new_local();
                    local_symbol.insts = insts_in_label;
                    self.src_file
                        .symbols_map
                        .insert(name.to_string(), local_symbol);
                }
                // パース終了
                _ => break,
            }
        }
    }
    pub fn parse_inst_atandt_syntax(&mut self) -> Option<X64Instruction> {
        // AT&T記法なので,パースしたオペランドは左がsrc,右がdstとなる.
        // ex. mov rax, 3

        let cur = self.looking_token_clone();
        self.read_token();
        match cur.kind {
            AsmTokenKind::ADDQ => {
                // 2つのオペランドを取得
                let src_op = self.consume_operand();
                let dst_op = self.consume_operand();

                Some(X64Instruction::new_add(src_op, dst_op))
            }
            AsmTokenKind::MOVQ => {
                // 2つのオペランドを取得
                let src_op = self.consume_operand();
                let dst_op = self.consume_operand();

                Some(X64Instruction::new_mov(src_op, dst_op))
            }
            AsmTokenKind::RET => Some(X64Instruction::new_ret()),
            _ => None,
        }
    }
}

// AT&T記法のパースに関するテスト
#[cfg(test)]
mod parse_atandt_tests {
    use super::*;
    use crate::assembler::arch::x64::lex_atandt;
    use crate::assembler::arch::x64::X64AssemblyFile;
    use crate::structure::{AssemblyFile, Syntax};
    use crate::target::Target;

    #[test]
    fn test_parse_atandt_syntax_with_no_inst() {
        let expected_main = X64Symbol::new_global();
        // .global main
        // main:
        let mut assembler = preprocess(".global main\n main:");
        assembler.parse_atandt_syntax();

        // mainシンボルが定義されているか
        assert!(assembler.src_file.symbols_map.contains_key("main"));
        if let Some(actual_main) = assembler.src_file.symbols_map.get("main") {
            assert_eq!(&expected_main, actual_main);
        }
    }

    #[test]
    fn test_parse_atandt_syntax_with_inst() {
        let mut expected_main = X64Symbol::new_global();
        expected_main.insts = vec![
            X64Instruction::new_mov(X64Operand::new_integer(3), X64Operand::new_register(0)),
            X64Instruction::new_add(X64Operand::new_integer(3), X64Operand::new_register(0)),
            X64Instruction::new_ret(),
        ];
        // .global main
        // main:
        //   movq $3, %rax
        //   addq $3, %rax
        //   ret
        let mut assembler =
            preprocess(".global main\nmain:\n  movq $3, %rax\n  addq $3, %rax\nret\n");
        assembler.parse_atandt_syntax();

        // mainシンボルが定義されているか
        assert!(assembler.src_file.symbols_map.contains_key("main"));
        if let Some(actual_main) = assembler.src_file.symbols_map.get("main") {
            for (i, inst) in actual_main.insts.iter().enumerate() {
                assert_eq!(&expected_main.insts[i], inst);
            }
        }
    }

    #[test]
    fn test_parse_inst_atandt_syntax_with_mov() {
        let expected_mov =
            X64Instruction::new_mov(X64Operand::new_integer(3), X64Operand::new_register(0));
        let mut assembler = preprocess("movq $3, %rax");
        let actual_opt_inst = assembler.parse_inst_atandt_syntax();

        // ちゃんとSome(inst)を返しているか, 中身がretであるか
        assert_eq!(Some(expected_mov), actual_opt_inst);

        // オフセットが進んでいるか
        assert_eq!(3, assembler.cur_token);
        assert_eq!(4, assembler.next_token);
    }
    #[test]
    fn test_parse_inst_atandt_syntax_with_ret() {
        let expected_ret = X64Instruction::new_ret();
        let mut assembler = preprocess("ret");
        let actual_opt_inst = assembler.parse_inst_atandt_syntax();

        // ちゃんとSome(inst)を返しているか, 中身がretであるか
        assert_eq!(Some(expected_ret), actual_opt_inst);

        // オフセットが進んでいるか
        assert_eq!(1, assembler.cur_token);
        assert_eq!(2, assembler.next_token);
    }

    fn preprocess(input: &str) -> X64Assembler {
        let target = Target::new();
        let assembly_file = AssemblyFile::new_atandt_file(input.to_string(), target);
        let x64_assembly_file = X64AssemblyFile::new(assembly_file);
        let mut assembler = X64Assembler::new(x64_assembly_file);

        lex_atandt::lexing_atandt_syntax(&mut assembler);
        assembler
    }
}
