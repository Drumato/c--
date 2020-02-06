use crate::assembler::arch::x64::asmtoken::{AsmToken, AsmTokenKind};
use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::lexer::AsmLexer;

pub fn lexing_intel_syntax(assembler: &mut X64Assembler) {
    // ソースコードのメモリコピーをするのは,後ほどエラーメッセージでソースコード本体を表示するため.
    // 本体の変更はしたくない.
    let mut lexer = AsmLexer::new(assembler.src_file.base_file.code.to_string());
    // 予約語の構築
    lexer.build_intel_keywords();
    let tokens = lexer.build_tokens_for_intel_syntax();
    assembler.tokens = tokens;
}

impl AsmLexer {
    fn build_tokens_for_intel_syntax(&mut self) -> Vec<AsmToken> {
        let mut tokens: Vec<AsmToken> = Vec::new();
        // 先に.intel_syntaxなどを全部パースしてしまう
        while let Some(t) = self.scan_directive() {
            tokens.push(t);
        }
        while let Some(t) = self.scan_one_intel_token() {
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
    fn scan_one_intel_token(&mut self) -> Option<AsmToken> {
        if self.contents.len() == 0 {
            let cur_position = self.current_position();
            return Some(AsmToken::new(cur_position, AsmTokenKind::EOF));
        }

        let head_char = self.contents.as_bytes()[0] as char;
        match head_char {
            // アルファベットの場合
            c if c.is_ascii_alphabetic() => Some(self.scan_word()),
            '_' | '.' => Some(self.scan_word()),

            // 数字の場合
            number if number.is_ascii_digit() => Some(self.scan_number()),

            // 記号の場合
            '[' => Some(self.scan_symbol(AsmTokenKind::LBRACKET)),
            ']' => Some(self.scan_symbol(AsmTokenKind::RBRACKET)),
            '-' => Some(self.scan_symbol(AsmTokenKind::MINUS)),
            ',' => Some(self.scan_symbol(AsmTokenKind::COMMA)),

            // comment
            '#' => Some(self.scan_comment()),

            // 空白類文字
            ' ' | '\t' => Some(self.skip_whitespace()),
            '\n' => {
                self.column = 1;
                self.row += 1;
                self.contents.drain(..1);
                Some(AsmToken::new((0, 0), AsmTokenKind::NEWLINE))
            }
            _ => None,
        }
    }

    // 予約語の構築
    fn build_intel_keywords(&mut self) {
        // 命令
        self.keywords.insert("add".to_string(), AsmTokenKind::ADD);
        self.keywords.insert("call".to_string(), AsmTokenKind::CALL);
        self.keywords.insert("jmp".to_string(), AsmTokenKind::JMP);
        self.keywords.insert("jz".to_string(), AsmTokenKind::JZ);
        self.keywords.insert("cmp".to_string(), AsmTokenKind::CMP);
        self.keywords.insert("cqo".to_string(), AsmTokenKind::CQO);
        self.keywords.insert("imul".to_string(), AsmTokenKind::IMUL);
        self.keywords.insert("idiv".to_string(), AsmTokenKind::IDIV);
        self.keywords.insert("mov".to_string(), AsmTokenKind::MOV);
        self.keywords.insert("ret".to_string(), AsmTokenKind::RET);
        self.keywords.insert("sub".to_string(), AsmTokenKind::SUB);
        self.keywords.insert("push".to_string(), AsmTokenKind::PUSH);
        self.keywords.insert("pop".to_string(), AsmTokenKind::POP);
        self.keywords.insert("neg".to_string(), AsmTokenKind::NEG);
        self.keywords
            .insert("syscall".to_string(), AsmTokenKind::SYSCALL);
        self.keywords
            .insert("QWORD".to_string(), AsmTokenKind::QWORD);
        self.keywords.insert("PTR".to_string(), AsmTokenKind::PTR);
    }
}

// Intel記法の字句解析に関するテスト
#[cfg(test)]
mod intel_lexer_tests {
    use super::*;

    #[test]
    fn test_build_tokens_for_intel_syntax() {
        // .intel_syntax noprefix
        // main:
        //   mov rax, 3
        //   ret

        let expected_tokens = vec![
            AsmToken::new(
                (1, 1),
                AsmTokenKind::DIRECTIVE("intel_syntax noprefix".to_string()),
            ),
            AsmToken::new((2, 1), AsmTokenKind::LABEL("main".to_string())),
            AsmToken::new((3, 3), AsmTokenKind::MOV),
            AsmToken::new((3, 7), AsmTokenKind::REG("rax".to_string())),
            AsmToken::new((3, 12), AsmTokenKind::INTEGER(3)),
            AsmToken::new((4, 3), AsmTokenKind::RET),
            AsmToken::new((5, 1), AsmTokenKind::EOF),
        ];
        let mut lexer =
            AsmLexer::new(".intel_syntax noprefix\nmain:\n  mov rax, 3\n  ret\n".to_string());
        lexer.build_intel_keywords();
        let tokens = lexer.build_tokens_for_intel_syntax();

        for (i, actual) in tokens.iter().enumerate() {
            assert_eq!(&expected_tokens[i], actual);
        }
    }

    #[test]
    fn test_scan_one_intel_token_with_single_int() {
        let expected_int = AsmToken::new((1, 1), AsmTokenKind::INTEGER(12345));
        let expected_eof = AsmToken::new((1, 6), AsmTokenKind::EOF);
        let mut lexer = create_lexer("12345");
        let actual = lexer.scan_one_intel_token();

        assert_eq!(Some(expected_int), actual);

        let should_eof = lexer.scan_one_intel_token();
        assert_eq!(Some(expected_eof), should_eof);
    }

    #[test]
    fn test_scan_one_intel_token_with_invalid_symbol() {
        let mut lexer = create_lexer("@");
        let actual = lexer.scan_one_intel_token();
        assert_eq!(None, actual);
    }

    #[test]
    fn test_scan_one_intel_token_with_words() {
        // rbp
        let expected_reg = AsmToken::new((1, 1), AsmTokenKind::REG("rbp".to_string()));
        let expected_label = AsmToken::new((1, 6), AsmTokenKind::LABEL("main".to_string()));
        let expected_add = AsmToken::new((1, 12), AsmTokenKind::ADD);

        let mut lexer = create_lexer("rbp, main: add");
        let actual_reg = lexer.scan_one_intel_token();
        assert_eq!(Some(expected_reg), actual_reg);

        // 空白
        lexer.scan_one_intel_token();

        let actual_label = lexer.scan_one_intel_token();
        assert_eq!(Some(expected_label), actual_label);

        // 空白
        lexer.scan_one_intel_token();

        let actual_add = lexer.scan_one_intel_token();
        assert_eq!(Some(expected_add), actual_add);
    }

    #[test]
    fn test_scan_word_with_intel_instruction() {
        let expected_mov = AsmToken::new((1, 1), AsmTokenKind::MOV);
        let mut lexer = create_lexer("mov");
        let actual_mov = lexer.scan_word();
        assert_eq!(expected_mov, actual_mov);
    }

    fn create_lexer(input: &str) -> AsmLexer {
        let mut lexer = AsmLexer::new(input.to_string());
        lexer.build_intel_keywords();
        lexer
    }
}
