use crate::assembler::arch::x64::asmtoken;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::X64Instruction;
use crate::assembler::arch::x64::symbol::X64Symbol;
use asmtoken::AsmTokenKind;

impl X64Assembler {
    pub fn parse_intel_syntax(&mut self) {
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
                    while let Some(inst) = self.parse_inst_intel_syntax() {
                        insts_in_label.push(inst);
                    }

                    // シンボルマップにエントリを登録
                    let mut global_symbol = X64Symbol::new_global();
                    global_symbol.insts = insts_in_label;
                    self.src_file
                        .symbols_map
                        .insert(name.to_string(), global_symbol);
                }
                // パース終了
                _ => break,
            }
        }
    }
    pub fn parse_inst_intel_syntax(&mut self) -> Option<X64Instruction> {
        // Intel記法なので,パースしたオペランドは左がdst,右がsrcとなる.
        // ex. mov rax, 3
        // AT&T記法に合わせて格納するので注意.
        // ex. X64InstKind::MOV(X64Operand::INTEGER(3), X64Operand::REG(0))

        let cur = self.looking_token_clone();
        match cur.kind {
            // 2つのオペランドを持つ命令
            AsmTokenKind::ADD | AsmTokenKind::SUB | AsmTokenKind::MOV | AsmTokenKind::IMUL => {
                self.read_token();

                let dst_op = self.consume_operand();
                let src_op = self.consume_operand();

                let inst_name = cur.kind.to_inst_name();
                Some(X64Instruction::new_binary_inst(inst_name, src_op, dst_op))
            }

            // 1つのオペランドを持つ命令
            AsmTokenKind::CALL | AsmTokenKind::IDIV | AsmTokenKind::JMP => {
                self.read_token();

                // 1つのオペランドを取得
                let unop = self.consume_operand();
                let inst_name = cur.kind.to_inst_name();
                Some(X64Instruction::new_unary_inst(inst_name, unop))
            }

            // オペランドを持たない命令
            AsmTokenKind::CQO | AsmTokenKind::RET | AsmTokenKind::SYSCALL => {
                self.read_token();
                let inst_name = cur.kind.to_inst_name();
                Some(X64Instruction::new_noop_inst(inst_name))
            }
            // ラベルはシンボルと違って命令列に含める
            AsmTokenKind::LABEL(name) => {
                // ラベルかどうかを先頭で判定
                if name.starts_with(".") {
                    self.read_token();
                    Some(X64Instruction::new_label(name.to_string()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

// Intel記法のパースに関するテスト
#[cfg(test)]
mod parse_intel_tests {
    use super::*;
    use crate::assembler::arch::x64::file::X64AssemblyFile;
    use crate::assembler::arch::x64::inst::{inst_kind::X64Operand, inst_name::X64InstName};
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_parse_intel_syntax_with_no_inst() {
        let expected_main = X64Symbol::new_global();
        // .global main
        // main:
        let mut assembler = preprocess(".global main\n main:");
        assembler.parse_intel_syntax();

        // mainシンボルが定義されているか
        assert!(assembler.src_file.symbols_map.contains_key("main"));
        if let Some(actual_main) = assembler.src_file.symbols_map.get("main") {
            assert_eq!(&expected_main, actual_main);
        }
    }

    #[test]
    fn test_parse_intel_syntax_with_call_inst() {
        let mut expected_main = X64Symbol::new_global();
        expected_main.insts = vec![
            X64Instruction::new_call(X64Operand::new_label("foo".to_string())),
            X64Instruction::new_ret(),
        ];
        let mut expected_foo = X64Symbol::new_global();
        expected_foo.insts = vec![
            X64Instruction::new_mov(
                X64Operand::new_integer(3),
                X64Operand::new_register("rax".to_string()),
            ),
            X64Instruction::new_ret(),
        ];
        // main:
        //   call foo
        //   ret
        // foo:
        //   mov rax, 3
        //   ret
        let mut assembler = preprocess("main:\n  call foo\n  ret\nfoo:\nmov rax, 3\n  ret\n");
        assembler.parse_intel_syntax();

        // 全シンボルが定義されているか
        assert!(assembler.src_file.symbols_map.contains_key("main"));
        assert!(assembler.src_file.symbols_map.contains_key("foo"));
        if let Some(actual_main) = assembler.src_file.symbols_map.get("main") {
            assert_eq!(&expected_main, actual_main);
        }
        if let Some(actual_foo) = assembler.src_file.symbols_map.get("foo") {
            assert_eq!(&expected_foo, actual_foo);
        }
    }

    #[test]
    fn test_parse_intel_syntax_with_multi_symbols() {
        let mut expected_main = X64Symbol::new_global();
        expected_main.insts = vec![X64Instruction::new_ret()];
        let mut expected_foo = X64Symbol::new_global();
        expected_foo.insts = vec![
            X64Instruction::new_mov(
                X64Operand::new_integer(3),
                X64Operand::new_register("rax".to_string()),
            ),
            X64Instruction::new_ret(),
        ];
        // main:
        //   ret
        // foo:
        //   mov rax, 3
        //   ret
        let mut assembler = preprocess("main:\n  ret\nfoo:\nmov rax, 3\n  ret\n");
        assembler.parse_intel_syntax();

        // 全シンボルが定義されているか
        assert!(assembler.src_file.symbols_map.contains_key("main"));
        assert!(assembler.src_file.symbols_map.contains_key("foo"));
        if let Some(actual_main) = assembler.src_file.symbols_map.get("main") {
            assert_eq!(&expected_main, actual_main);
        }
        if let Some(actual_foo) = assembler.src_file.symbols_map.get("foo") {
            assert_eq!(&expected_foo, actual_foo);
        }
    }

    #[test]
    fn test_parse_intel_syntax_with_inst() {
        let mut expected_main = X64Symbol::new_global();
        // AT&T記法でテストを定義
        expected_main.insts = vec![
            X64Instruction::new_binary_inst(
                X64InstName::MOV,
                X64Operand::new_integer(3),
                X64Operand::new_register("rax".to_string()),
            ),
            X64Instruction::new_binary_inst(
                X64InstName::ADD,
                X64Operand::new_integer(3),
                X64Operand::new_register("rax".to_string()),
            ),
            X64Instruction::new_ret(),
        ];
        // .global main
        // main:
        //   mov rax, 3
        //   add rax, 3
        //   ret
        let mut assembler = preprocess(".global main\n main:\n  mov rax, 3\n add rax, 3\n  ret");
        assembler.parse_intel_syntax();

        // mainシンボルが定義されているか
        assert!(assembler.src_file.symbols_map.contains_key("main"));
        if let Some(actual_main) = assembler.src_file.symbols_map.get("main") {
            for (i, inst) in actual_main.insts.iter().enumerate() {
                assert_eq!(&expected_main.insts[i], inst);
            }
        }
    }

    #[test]
    fn test_parse_inst_intel_syntax_with_ret() {
        let expected_ret = X64Instruction::new_ret();
        let mut assembler = preprocess("ret");
        let actual_opt_inst = assembler.parse_inst_intel_syntax();

        // ちゃんとSome(inst)を返しているか, 中身がretであるか
        assert_eq!(Some(expected_ret), actual_opt_inst);

        // オフセットが進んでいるか
        assert_eq!(1, assembler.cur_token);
        assert_eq!(2, assembler.next_token);
    }

    fn preprocess(input: &str) -> X64Assembler {
        let target = Target::new();
        let assembly_file = AssemblyFile::new_intel_file(input.to_string(), target);
        let x64_assembly_file = X64AssemblyFile::new(assembly_file);
        let mut assembler = X64Assembler::new(x64_assembly_file);

        lex_intel::lexing_intel_syntax(&mut assembler);
        assembler
    }
}
