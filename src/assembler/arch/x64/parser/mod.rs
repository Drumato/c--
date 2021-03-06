pub mod parse_atandt;
pub mod parse_intel;

use crate::assembler::arch::x64::asmtoken;
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::inst_kind::X64Operand;
use crate::assembler::arch::x64::symbol::X64Symbol;
use crate::error::*;
use asmtoken::{AsmToken, AsmTokenKind};

impl X64Assembler {
    pub fn consume_operand(&mut self) -> X64Operand {
        let cur = self.looking_token_clone();
        let cur_operand = match cur.kind {
            AsmTokenKind::MINUS => {
                // - <offset> [ <register> ]
                self.read_token();
                let offset_token = self.looking_token_clone();
                if let AsmTokenKind::INTEGER(offset) = offset_token.kind {
                    self.read_token();
                    self.read_token(); // [

                    let reg_token = self.looking_token_clone();
                    if let AsmTokenKind::REG(name) = reg_token.kind {
                        self.read_token();
                        X64Operand::new_addressing(offset, name.to_string())
                    } else {
                        panic!("invalid register in memory addressing");
                    }
                } else {
                    panic!("offset must be integer in memory addressing");
                }
            }
            AsmTokenKind::REG(name) => X64Operand::new_register(name),
            AsmTokenKind::LABEL(name) => X64Operand::new_label(name),
            AsmTokenKind::INTEGER(val) => X64Operand::new_integer(val),
            // エラー生成
            _ => {
                panic!("invalid operand found -> {:?}", cur);
            }
        };
        // オフセットを進める
        self.read_token();
        cur_operand
    }
    pub fn parse_directive(&mut self, directive: String, position: (usize, usize)) {
        // オフセットは次にすすめておく
        self.read_token();

        // directiveのチェック
        if directive.starts_with("global") || directive.starts_with("globl") {
            // グローバルシンボルの指定
            self.parse_global_directive(directive, position);
        }
    }
    pub fn parse_global_directive(&mut self, directive: String, position: (usize, usize)) {
        let symbol_name_vector: Vec<&str> = directive.rsplit(' ').collect();

        // rsplit().len() == 1 -> 後にシンボル名が続いていないのでエラー
        if symbol_name_vector.len() == 1 {
            let err = Error::new(
                ErrorKind::AsmParse,
                position,
                ErrorMsg::MustSpecifySymbolNameInGlobalDirective,
            );
            err.found();
        }

        // グローバルシンボルとして,シンボルマップにエントリを登録しておく
        let symbol_name = symbol_name_vector[0].to_string();
        let global_symbol = X64Symbol::new_global();
        self.src_file.symbols_map.insert(symbol_name, global_symbol);
    }

    pub fn looking_token_clone(&mut self) -> AsmToken {
        if self.tokens.len() <= self.cur_token {
            let last_token_position = self.tokens.last().unwrap().position;
            return AsmToken::new(last_token_position, AsmTokenKind::EOF);
        }
        self.tokens[self.cur_token].clone()
    }

    pub fn read_token(&mut self) {
        self.cur_token += 1;
        self.next_token += 1;
    }
}

// Intel,AT&Tどちらからも用いられる関数のテスト
#[cfg(test)]
mod general_parser_tests {
    use super::*;
    use crate::assembler::arch::x64::file::X64AssemblyFile;
    use crate::assembler::arch::x64::lexer::{lex_atandt, lex_intel};
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_intel_consume_operand() {
        let expected_reg = X64Operand::new_register("rax".to_string());
        let expected_int = X64Operand::new_integer(3);
        let expected_label = X64Operand::new_label("main".to_string());
        let mut assembler = preprocess_intel("main, rax, 3");

        // ラベルチェック
        let actual_label = assembler.consume_operand();
        assert_eq!(expected_label, actual_label);

        // レジスタチェック
        let actual_reg = assembler.consume_operand();
        assert_eq!(expected_reg, actual_reg);

        // 即値チェック
        let actual_int = assembler.consume_operand();
        assert_eq!(expected_int, actual_int);
    }

    #[test]
    fn test_atandt_consume_operand() {
        let expected_reg = X64Operand::new_register("rax".to_string());
        let expected_int = X64Operand::new_integer(3);
        let expected_label = X64Operand::new_label("main".to_string());
        let mut assembler = preprocess_atandt("main, %rax, $3");

        // ラベルチェック
        let actual_label = assembler.consume_operand();
        assert_eq!(expected_label, actual_label);

        // レジスタチェック
        let actual_reg = assembler.consume_operand();
        assert_eq!(expected_reg, actual_reg);

        // 即値チェック
        let actual_int = assembler.consume_operand();
        assert_eq!(expected_int, actual_int);
    }

    #[test]
    fn test_parse_directive() {
        let mut assembler = preprocess_intel("");
        assembler.parse_directive("global main".to_string(), (0, 0));

        // グローバルシンボルは登録されているか
        assert_eq!(1, assembler.src_file.symbols_map.len());

        // シンボル名が期待するものであるか
        assert!(assembler.src_file.symbols_map.contains_key("main"));

        // オフセットが(初期化から)進んでいるか
        assert_eq!(1, assembler.cur_token);
        assert_eq!(2, assembler.next_token);
    }

    fn preprocess_intel(input: &str) -> X64Assembler {
        let target = Target::new();
        let assembly_file = AssemblyFile::new_intel_file(input.to_string(), target);
        let x64_assembly_file = X64AssemblyFile::new(assembly_file);
        let mut assembler = X64Assembler::new(x64_assembly_file);
        lex_intel::lexing_intel_syntax(&mut assembler);
        assembler
    }
    fn preprocess_atandt(input: &str) -> X64Assembler {
        let target = Target::new();
        let assembly_file = AssemblyFile::new_atandt_file(input.to_string(), target);
        let x64_assembly_file = X64AssemblyFile::new(assembly_file);
        let mut assembler = X64Assembler::new(x64_assembly_file);
        lex_atandt::lexing_atandt_syntax(&mut assembler);
        assembler
    }
}
