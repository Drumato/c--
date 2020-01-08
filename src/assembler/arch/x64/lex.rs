use crate::assembler::arch::x64::asmtoken::{AsmToken, AsmTokenKind, Position};

use std::collections::BTreeMap;

pub struct AsmLexer {
    pub column: usize,                            // x軸の座標
    pub row: usize,                               // y軸の座標
    pub contents: String, // メモリコピーし, AssemblyrFile構造体の文字列を破壊しないように
    pub keywords: BTreeMap<String, AsmTokenKind>, // 予約語をO(1)で取り出すためのメンバ
}

impl AsmLexer {
    pub fn new(contents: String) -> Self {
        Self {
            row: 1,
            column: 1,
            contents: contents,
            keywords: BTreeMap::new(),
        }
    }
    // 文字列を切り取って,ディレクティブトークンを返す
    pub fn scan_directive(&mut self) -> AsmToken {
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

    // 文字列を切り取って,レジスタ/命令/ラベルトークンを返す
    pub fn scan_word(&mut self) -> AsmToken {
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

    // 数字を切り取って,整数トークンを返す
    pub fn scan_number(&mut self) -> AsmToken {
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
    pub fn skip_whitespace(&mut self) -> AsmToken {
        let ws_length = Self::count_length(&self.contents, |c| {
            c.is_whitespace() || c == &'\t' || c == &','
        });

        self.column += ws_length;
        self.contents.drain(..ws_length);

        // トークン列には追加されないのでポジションはDefaultでいい.
        AsmToken::new((0, 0), AsmTokenKind::BLANK)
    }

    pub fn skip_offset(&mut self, len: usize) {
        self.column += len;
        self.contents.drain(..len);
    }

    pub fn take_conditional_string(input: &str, f: fn(ch: &char) -> bool) -> String {
        input.chars().take_while(f).collect::<String>()
    }

    pub fn check_register(reg: &str) -> Option<usize> {
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
    pub fn count_length(input: &str, f: fn(ch: &char) -> bool) -> usize {
        Self::take_conditional_string(input, f).len()
    }

    // 現在のオフセットを取得
    pub fn current_position(&mut self) -> Position {
        (self.row, self.column)
    }
}

#[cfg(test)]
mod general_lexer_tests {
    use super::*;

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
        assert_eq!((1, 6), cur_position);

        // 文字列が切り取られているか.
        let cur_looking_string = lexer.contents;
        assert_eq!(cur_looking_string, "");
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
        let expected_eof = AsmToken::new((1, 6), AsmTokenKind::EOF);

        let mut lexer = create_lexer("     ");
        let whitespace = lexer.skip_whitespace();

        assert!(whitespace.should_ignore());

        assert_eq!(6, lexer.column);
    }

    fn create_lexer(input: &str) -> AsmLexer {
        let mut lexer = AsmLexer::new(input.to_string());
        lexer
    }
}
