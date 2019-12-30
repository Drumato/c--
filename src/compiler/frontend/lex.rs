use crate::compiler::frontend;
use frontend::token::{Position, Token, TokenKind};

use std::collections::BTreeMap;
pub fn tokenize(manager: &mut frontend::Manager) {
    // ソースコードのメモリコピーをするのは,後ほどエラーメッセージでソースコード本体を表示するため.
    // 本体の変更はしたくない.
    let mut lexer = Lexer::new(manager.src_file.contents.to_string());
    let tokens = lexer.build_tokens();
    manager.tokens = tokens;
}

struct Lexer<'a> {
    column: usize,                          // x軸の座標
    row: usize,                             // y軸の座標
    contents: String, // メモリコピーし, SrcFile構造体の文字列を破壊しないように
    keywords: BTreeMap<&'a str, TokenKind>, // 予約語をO(1)で取り出すためのメンバ
}

impl<'a> Lexer<'a> {
    fn new(contents: String) -> Self {
        Self {
            row: 0,
            column: 0,
            contents: contents,
            keywords: BTreeMap::new(), // TODO: 後々build_keywordsを渡す
        }
    }
    // Vec<Token> をLexerにもたせると,Managerにわたすとき大きなメモリコピーが走る.
    // Lexerは字句解析に必要な最低限の情報だけ持つように.
    fn build_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(t) = self.scan_one_token() {
            // コメントとか改行文字とか
            if t.should_ignore() {
                continue;
            }

            if let &TokenKind::EOF = &t.kind {
                tokens.push(t);
                break;
            }

            // 正常系
            tokens.push(t);
        }
        tokens
    }
    // 特殊文字などはNoneで返す,上位関数ではSome<Token>の間ループ
    fn scan_one_token(&mut self) -> Option<Token> {
        if self.contents.len() == 0 {
            let cur_position = self.current_position();
            return Some(Token::new(cur_position, TokenKind::EOF));
        }

        let head_char = self.contents.as_bytes()[0] as char;
        match head_char {
            number if number.is_ascii_digit() => Some(self.scan_number()),
            _ => None,
        }
    }

    // 数字を切り取って,整数トークンを返す
    fn scan_number(&mut self) -> Token {
        // 数字の範囲を切り取る
        let number_length = Self::count_length(&self.contents, |c| c.is_ascii_digit());
        // 文字列を数値に変換
        let decimal_value = self.contents[..number_length].parse::<i128>().unwrap();

        // 現在のオフセットを退避
        let cur_position = self.current_position();

        // 文字列のオフセットを進める
        self.column += number_length;
        self.contents.drain(..number_length);

        Token::new(cur_position, TokenKind::INTEGER(decimal_value))
    }
    fn count_length(input: &str, f: fn(ch: &char) -> bool) -> usize {
        input.chars().take_while(f).collect::<String>().len()
    }
    fn current_position(&mut self) -> Position {
        (self.column, self.row)
    }
}

// Lexer構造体に関するテスト
#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_count_length() {
        // 数字の範囲
        let number_range = Lexer::count_length("12345", |c| c.is_ascii_digit());

        assert_eq!(number_range, 5);

        // アルファベットの範囲
        let alpha_range = Lexer::count_length("drumato", |c| c.is_alphabetic());
        assert_eq!(alpha_range, 7);

        // 空白の範囲
        let ws_range = Lexer::count_length("          ", |c| c.is_whitespace());
        assert_eq!(ws_range, 10);
    }

    #[test]
    fn test_scan_number() {
        // この関数に入る時点で数字であることは確定なので(パターンマッチによって)
        // 異常系のテストはいらない
        let input = "12345".to_string();
        let mut lexer = Lexer::new(input);
        let actual = lexer.scan_number();

        assert_eq!(TokenKind::INTEGER(12345), actual.kind);
        assert_eq!((0, 0), actual.position);

        // scan_number() 内部でオフセットがちゃんと進んでいるか.
        let cur_position = lexer.current_position();
        assert_eq!((5, 0), cur_position);

        // 文字列が切り取られているか.
        let cur_looking_string = lexer.contents;
        assert_eq!(cur_looking_string, "");
    }

    #[test]
    fn test_scan_one_token_with_single_int() {
        let expected_int = Token::new((0, 0), TokenKind::INTEGER(12345));
        let expected_eof = Token::new((5, 0), TokenKind::EOF);
        let input = "12345".to_string();
        let mut lexer = Lexer::new(input);
        let actual = lexer.scan_one_token();

        assert_eq!(Some(expected_int), actual);

        let should_eof = lexer.scan_one_token();
        assert_eq!(Some(expected_eof), should_eof);
    }

    #[test]
    fn test_scan_one_token_with_invalid_symbol() {
        let input = "@".to_string();
        let mut lexer = Lexer::new(input);
        let actual = lexer.scan_one_token();
        assert_eq!(None, actual);
    }
}
