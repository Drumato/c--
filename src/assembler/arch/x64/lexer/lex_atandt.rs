use crate::assembler::arch::x64::asmtoken::{AsmToken, AsmTokenKind};
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::lexer::AsmLexer;
use crate::error::*;

pub fn lexing_atandt_syntax(assembler: &mut X64Assembler) {
    // ソースコードのメモリコピーをするのは,後ほどエラーメッセージでソースコード本体を表示するため.
    // 本体の変更はしたくない.
    let mut lexer = AsmLexer::new(assembler.src_file.base_file.code.to_string());

    // 予約語の構築
    lexer.build_atandt_keywords();

    let tokens = lexer.build_tokens_for_atandt_syntax();
    assembler.tokens = tokens;
}

impl AsmLexer {
    pub fn build_tokens_for_atandt_syntax(&mut self) -> Vec<AsmToken> {
        let mut tokens: Vec<AsmToken> = Vec::new();
        // 先に.intel_syntaxなどを全部パースしてしまう
        while let Some(t) = self.scan_directive() {
            tokens.push(t);
        }
        while let Some(t) = self.scan_one_atandt_token() {
            // コメントとか改行文字とか
            if t.should_ignore() {
                continue;
            }

            if let &AsmTokenKind::EOF = &t.kind {
                tokens.push(t);
                break;
            }

            // 正常系
            tokens.push(t);
        }
        tokens
    }
    // 特殊文字などはNoneで返す,上位関数ではSome<AsmToken>の間ループ
    fn scan_one_atandt_token(&mut self) -> Option<AsmToken> {
        if self.contents.len() == 0 {
            let cur_position = self.current_position();
            return Some(AsmToken::new(cur_position, AsmTokenKind::EOF));
        }

        let head_char = self.contents.as_bytes()[0] as char;
        match head_char {
            // %の場合 -> レジスタ
            '%' => {
                self.skip_offset(1);
                Some(self.scan_word())
            }

            // アルファベットの場合 -> 命令かシンボル/ラベル
            '_' | '.' => Some(self.scan_word()),
            c if c.is_ascii_alphabetic() => Some(self.scan_word()),

            // $の場合 -> 数値リテラル
            '$' => {
                self.skip_offset(1);
                let cur_position = self.current_position();

                // ちゃんと数値リテラルかチェック
                if !(self.contents.as_bytes()[0] as char).is_ascii_digit() {
                    let err = Error::new(
                        ErrorKind::AsmParse,
                        cur_position,
                        ErrorMsg::MustBeIntegerLiteral,
                    );
                    err.found();
                    None
                } else {
                    Some(self.scan_number())
                }
            }

            // 空白類文字
            ',' | ' ' | '\t' => Some(self.skip_whitespace()),
            '\n' => {
                self.column = 1;
                self.row += 1;
                self.contents.drain(..1);
                Some(AsmToken::new((0, 0), AsmTokenKind::NEWLINE))
            }
            _ => None,
        }
    }

    pub fn build_atandt_keywords(&mut self) {
        // 命令
        self.keywords.insert("addq".to_string(), AsmTokenKind::ADDQ);
        self.keywords.insert("negq".to_string(), AsmTokenKind::NEGQ);
        self.keywords.insert("jmp".to_string(), AsmTokenKind::JMP);
        self.keywords.insert("call".to_string(), AsmTokenKind::CALL);
        self.keywords.insert("cltd".to_string(), AsmTokenKind::CLTD);
        self.keywords.insert("movq".to_string(), AsmTokenKind::MOVQ);
        self.keywords
            .insert("imulq".to_string(), AsmTokenKind::IMULQ);
        self.keywords
            .insert("idivq".to_string(), AsmTokenKind::IDIVQ);
        self.keywords.insert("ret".to_string(), AsmTokenKind::RET);
        self.keywords
            .insert("syscall".to_string(), AsmTokenKind::SYSCALL);
        self.keywords.insert("subq".to_string(), AsmTokenKind::SUBQ);
        self.keywords.insert("push".to_string(), AsmTokenKind::PUSH);
        self.keywords.insert("pop".to_string(), AsmTokenKind::POP);
    }
}

// AT&T記法の字句解析に関するテスト
#[cfg(test)]
mod atandt_lexer_tests {
    use super::*;

    #[test]
    fn test_build_tokens_for_atandt_syntax() {
        // .global main
        // main:
        //   movq $3, %rax
        //   addq $3,%rax
        //   ret

        let expected_tokens = vec![
            AsmToken::new((1, 1), AsmTokenKind::DIRECTIVE("global main".to_string())),
            AsmToken::new((2, 1), AsmTokenKind::LABEL("main".to_string())),
            AsmToken::new((3, 3), AsmTokenKind::MOVQ),
            AsmToken::new((3, 9), AsmTokenKind::INTEGER(3)),
            AsmToken::new((3, 13), AsmTokenKind::REG("rax".to_string())),
            AsmToken::new((4, 3), AsmTokenKind::ADDQ),
            AsmToken::new((4, 9), AsmTokenKind::INTEGER(3)),
            AsmToken::new((4, 12), AsmTokenKind::REG("rax".to_string())),
            AsmToken::new((5, 3), AsmTokenKind::RET),
            AsmToken::new((6, 1), AsmTokenKind::EOF),
        ];
        let mut lexer = AsmLexer::new(
            ".global main\nmain:\n  movq $3, %rax\n  addq $3,%rax\n  ret\n".to_string(),
        );
        lexer.build_atandt_keywords();
        let tokens = lexer.build_tokens_for_atandt_syntax();

        for (i, actual) in tokens.iter().enumerate() {
            assert_eq!(&expected_tokens[i], actual);
        }
    }

    #[test]
    fn test_scan_one_atandt_token_with_single_int() {
        let expected_int = AsmToken::new((1, 2), AsmTokenKind::INTEGER(12345));
        let expected_eof = AsmToken::new((1, 7), AsmTokenKind::EOF);
        let mut lexer = create_lexer("$12345");
        let actual = lexer.scan_one_atandt_token();

        assert_eq!(Some(expected_int), actual);

        let should_eof = lexer.scan_one_atandt_token();
        assert_eq!(Some(expected_eof), should_eof);
    }

    #[test]
    fn test_scan_word_with_atandt_instruction() {
        let expected_movq = AsmToken::new((1, 1), AsmTokenKind::MOVQ);
        let mut lexer = create_lexer("movq");
        let actual_movq = lexer.scan_word();
        assert_eq!(expected_movq, actual_movq);
    }

    #[test]
    fn test_scan_one_atandt_token_with_words() {
        // rbp
        let expected_reg = AsmToken::new((1, 2), AsmTokenKind::REG("rbp".to_string()));
        let expected_label = AsmToken::new((1, 7), AsmTokenKind::LABEL("main".to_string()));
        let expected_add = AsmToken::new((1, 13), AsmTokenKind::ADDQ);

        let mut lexer = create_lexer("%rbp, main: addq");
        let actual_reg = lexer.scan_one_atandt_token();
        assert_eq!(Some(expected_reg), actual_reg);

        // 空白
        lexer.scan_one_atandt_token();

        let actual_label = lexer.scan_one_atandt_token();
        assert_eq!(Some(expected_label), actual_label);

        // 空白
        lexer.scan_one_atandt_token();

        let actual_add = lexer.scan_one_atandt_token();
        assert_eq!(Some(expected_add), actual_add);
    }
    fn create_lexer(input: &str) -> AsmLexer {
        let mut lexer = AsmLexer::new(input.to_string());
        lexer.build_atandt_keywords();
        lexer
    }
}
