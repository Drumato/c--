use crate::assembler::arch::x64::asmtoken::{AsmToken, AsmTokenKind, Position};
use crate::assembler::arch::x64::X64Assembler;

use std::collections::BTreeMap;

pub fn lexing_intel_syntax(assembler: &mut X64Assembler) {
    // ソースコードのメモリコピーをするのは,後ほどエラーメッセージでソースコード本体を表示するため.
    // 本体の変更はしたくない.
    let mut lexer = AsmLexer::new(assembler.src_file.base_file.code.to_string());
    // 予約語の構築
    lexer.build_keywords();
    let tokens = lexer.build_tokens_for_intel_syntax();
    assembler.tokens = tokens;
}

struct AsmLexer {
    column: usize,                            // x軸の座標
    row: usize,                               // y軸の座標
    contents: String, // メモリコピーし, AssemblyrFile構造体の文字列を破壊しないように
    keywords: BTreeMap<String, AsmTokenKind>, // 予約語をO(1)で取り出すためのメンバ
}

impl AsmLexer {
    fn new(contents: String) -> Self {
        Self {
            row: 1,
            column: 1,
            contents: contents,
            keywords: BTreeMap::new(),
        }
    }
    fn build_tokens_for_intel_syntax(&mut self) -> Vec<AsmToken> {
        let mut tokens: Vec<AsmToken> = Vec::new();
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
            // 数字の場合
            number if number.is_ascii_digit() => Some(self.scan_number()),

            // .intel_syntax等のディレクティブ
            '.' => Some(self.scan_directive()),

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

    // 文字列を切り取って,レジスタ/命令/ラベルトークンを返す
    fn scan_word(&mut self) -> AsmToken {
        // 現在のオフセットを退避
        let cur_position = self.current_position();

        // 空白,改行までの文字列を読み取る
        let word = Self::take_conditional_string(&self.contents, |c| c != &' ' && c != &'\n');

        // オフセットを進める
        self.skip_offset(word.len());

        // ,や:があればトリム
        let word_trimmed = word.trim_end_matches(',').trim_end_matches(':');

        // 命令かチェック
        if let Some(t_kind) = self.keywords.get(word_trimmed) {
            return AsmToken::new(cur_position, t_kind.clone());
        }

        // レジスタかチェック
        if let Some(reg_number) = Self::check_register(&word_trimmed) {
            return AsmToken::new(cur_position, AsmTokenKind::REG(reg_number));
        }

        // ラベル
        AsmToken::new(cur_position, AsmTokenKind::LABEL(word_trimmed.to_string()))
    }

    // 文字列を切り取って,ディレクティブトークンを返す
    fn scan_directive(&mut self) -> AsmToken {
        // 現在のオフセットを退避
        let cur_position = self.current_position();

        // . の読み飛ばし
        self.skip_offset(1);

        // 改行までの文字列を読み取る
        // 取得した文字列は後で好きなようにパースする
        let directive = Self::take_conditional_string(&self.contents, |c| c != &'\n');

        // 文字列のオフセットを進める
        self.skip_offset(directive.len());

        AsmToken::new(cur_position, AsmTokenKind::DIRECTIVE(directive))
    }

    // 数字を切り取って,整数トークンを返す
    fn scan_number(&mut self) -> AsmToken {
        // 数字の範囲を切り取る
        let number_length = Self::count_length(&self.contents, |c| c.is_ascii_digit());

        // 文字列を数値に変換
        let decimal_value = self.contents[..number_length].parse::<i128>().unwrap();

        // 現在のオフセットを退避
        let cur_position = self.current_position();

        // 文字列のオフセットを進める
        self.skip_offset(number_length);

        AsmToken::new(cur_position, AsmTokenKind::INTEGER(decimal_value))
    }

    // 空白類文字を読み飛ばす.
    fn skip_whitespace(&mut self) -> AsmToken {
        let ws_length = Self::count_length(&self.contents, |c| c.is_whitespace() || c == &'\t');

        self.column += ws_length;
        self.contents.drain(..ws_length);

        // トークン列には追加されないのでポジションはDefaultでいい.
        AsmToken::new((0, 0), AsmTokenKind::BLANK)
    }

    fn skip_offset(&mut self, len: usize) {
        self.column += len;
        self.contents.drain(..len);
    }

    fn take_conditional_string(input: &str, f: fn(ch: &char) -> bool) -> String {
        input.chars().take_while(f).collect::<String>()
    }

    fn check_register(reg: &str) -> Option<usize> {
        match reg {
            "al" | "ax" | "eax" | "rax" => Some(0),
            "cl" | "cx" | "ecx" | "rcx" => Some(1),
            "edx" | "rdx" => Some(2),
            "ebx" | "rbx" => Some(3),
            "esp" | "rsp" => Some(4),
            "ebp" | "rbp" => Some(5),
            "esi" | "rsi" => Some(6),
            "edi" | "rdi" => Some(7),
            "r8" => Some(8),
            "r9" => Some(9),
            "r10" => Some(10),
            "r11" => Some(11),
            "r12" => Some(12),
            "r13" => Some(13),
            "r14" => Some(14),
            "r15" => Some(15),
            _ => None,
        }
    }
    fn count_length(input: &str, f: fn(ch: &char) -> bool) -> usize {
        Self::take_conditional_string(input, f).len()
    }

    // 現在のオフセットを取得
    fn current_position(&mut self) -> Position {
        (self.column, self.row)
    }

    // 予約語の構築
    fn build_keywords(&mut self) {
        // 命令
        self.keywords.insert("add".to_string(), AsmTokenKind::ADD);
        self.keywords.insert("mov".to_string(), AsmTokenKind::MOV);
        self.keywords.insert("ret".to_string(), AsmTokenKind::RET);
    }
}

// Intel記法の字句解析に関するテスト
#[cfg(test)]
mod intel_lexer_tests {
    use super::*;

    #[test]
    fn test_build_tokens() {
        // .intel_syntax noprefix
        // main:
        //   mov rax, 3
        //   ret

        let expected_tokens = vec![
            AsmToken::new(
                (1, 1),
                AsmTokenKind::DIRECTIVE("intel_syntax noprefix".to_string()),
            ),
            AsmToken::new((1, 2), AsmTokenKind::LABEL("main".to_string())),
            AsmToken::new((3, 3), AsmTokenKind::MOV),
            AsmToken::new((7, 3), AsmTokenKind::REG(0)),
            AsmToken::new((12, 3), AsmTokenKind::INTEGER(3)),
            AsmToken::new((3, 4), AsmTokenKind::RET),
            AsmToken::new((1, 5), AsmTokenKind::EOF),
        ];
        let mut lexer =
            AsmLexer::new(".intel_syntax noprefix\nmain:\n  mov rax, 3\n  ret\n".to_string());
        lexer.build_keywords();
        let tokens = lexer.build_tokens_for_intel_syntax();

        for (i, actual) in tokens.iter().enumerate() {
            assert_eq!(&expected_tokens[i], actual);
        }
    }

    #[test]
    fn test_count_length() {
        // 数字の範囲
        let number_range = AsmLexer::count_length("12345", |c| c.is_ascii_digit());

        assert_eq!(number_range, 5);

        // アルファベットの範囲
        let alpha_range = AsmLexer::count_length("drumato", |c| c.is_alphabetic());
        assert_eq!(alpha_range, 7);

        // 空白の範囲
        let ws_range = AsmLexer::count_length("          ", |c| c.is_whitespace());
        assert_eq!(ws_range, 10);
    }

    #[test]
    fn test_scan_number() {
        // この関数に入る時点で数字であることは確定なので(パターンマッチによって)
        // 異常系のテストはいらない
        let mut lexer = create_lexer("12345");
        let actual = lexer.scan_number();

        assert_eq!(AsmTokenKind::INTEGER(12345), actual.kind);
        assert_eq!((1, 1), actual.position);

        // scan_number() 内部でオフセットがちゃんと進んでいるか.
        let cur_position = lexer.current_position();
        assert_eq!((6, 1), cur_position);

        // 文字列が切り取られているか.
        let cur_looking_string = lexer.contents;
        assert_eq!(cur_looking_string, "");
    }

    #[test]
    fn test_scan_one_intel_token_with_single_int() {
        let expected_int = AsmToken::new((1, 1), AsmTokenKind::INTEGER(12345));
        let expected_eof = AsmToken::new((6, 1), AsmTokenKind::EOF);
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
        let expected_reg = AsmToken::new((1, 1), AsmTokenKind::REG(5));
        let expected_label = AsmToken::new((6, 1), AsmTokenKind::LABEL("main".to_string()));
        let expected_add = AsmToken::new((12, 1), AsmTokenKind::ADD);

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
    fn test_scan_word_with_instruction() {
        let expected_mov = AsmToken::new((1, 1), AsmTokenKind::MOV);
        let mut lexer = create_lexer("mov");
        let actual_mov = lexer.scan_word();
        assert_eq!(expected_mov, actual_mov);
    }

    #[test]
    fn test_scan_directive() {
        let expected = AsmToken::new(
            (1, 1),
            AsmTokenKind::DIRECTIVE("intel_syntax noprefix".to_string()),
        );
        let mut lexer = create_lexer(".intel_syntax noprefix");
        let actual = lexer.scan_directive();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_skip_whitespace() {
        let expected_eof = AsmToken::new((6, 1), AsmTokenKind::EOF);

        let mut lexer = create_lexer("     ");
        let whitespace = lexer.skip_whitespace();

        assert!(whitespace.should_ignore());

        assert_eq!(6, lexer.column);
        let should_eof = lexer.scan_one_intel_token();

        assert_eq!(Some(expected_eof), should_eof);
    }

    #[test]
    fn test_build_keywords() {
        let mut lexer = create_lexer("");
        lexer.build_keywords();

        assert_eq!(3, lexer.keywords.len());

        assert!(lexer.keywords.contains_key("add"));
        assert!(lexer.keywords.contains_key("mov"));
        assert!(lexer.keywords.contains_key("ret"));
    }

    fn create_lexer(input: &str) -> AsmLexer {
        let mut lexer = AsmLexer::new(input.to_string());
        lexer.build_keywords();
        lexer
    }
}
