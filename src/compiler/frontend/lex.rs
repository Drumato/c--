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
            row: 1,
            column: 1,
            contents: contents,
            keywords: BTreeMap::new(),
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
            // 数字の場合
            number if number.is_ascii_digit() => Some(self.scan_number()),

            // 記号の場合
            '+' => Some(self.scan_symbol(TokenKind::PLUS)),

            // 空白類文字
            ' ' | '\t' => Some(self.skip_whitespace()),
            '\n' => {
                self.column = 1;
                self.row += 1;
                self.contents.drain(..1);
                Some(Token::new((0, 0), TokenKind::NEWLINE))
            }
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
        self.skip_offset(number_length);

        Token::new(cur_position, TokenKind::INTEGER(decimal_value))
    }

    // 記号を切り取って,トークンを返す.
    fn scan_symbol(&mut self, kind: TokenKind) -> Token {
        // 現在のオフセットを退避
        let cur_position = self.current_position();

        // 文字列のオフセットを進める.
        self.skip_offset(1);

        Token::new(cur_position, kind)
    }

    // 空白類文字を読み飛ばす.
    fn skip_whitespace(&mut self) -> Token {
        let ws_length = Self::count_length(&self.contents, |c| c.is_whitespace() || c == &'\t');

        self.skip_offset(ws_length);

        // トークン列には追加されないのでポジションはDefaultでいい.
        Token::new((0, 0), TokenKind::BLANK)
    }

    fn skip_offset(&mut self, len: usize) {
        self.column += len;
        self.contents.drain(..len);
    }

    fn take_conditional_string(input: &str, f: fn(ch: &char) -> bool) -> String {
        input.chars().take_while(f).collect::<String>()
    }

    fn count_length(input: &str, f: fn(ch: &char) -> bool) -> usize {
        Self::take_conditional_string(input, f).len()
    }

    // 現在のオフセットを取得
    fn current_position(&mut self) -> Position {
        (self.column, self.row)
    }
}

// Lexer構造体に関するテスト
#[cfg(test)]
mod lexer_tests {
    use super::*;

    #[test]
    fn test_build_tokens() {
        let expected_tokens = vec![
            Token::new((1, 1), TokenKind::INTEGER(12345)),
            Token::new((7, 1), TokenKind::PLUS),
            Token::new((9, 1), TokenKind::INTEGER(678910)),
            Token::new((15, 1), TokenKind::EOF),
        ];
        let mut lexer = create_lexer("12345 + 678910");
        let tokens = lexer.build_tokens();

        for (i, actual) in tokens.iter().enumerate() {
            assert_eq!(&expected_tokens[i], actual);
        }
    }

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
        let mut lexer = create_lexer("12345");
        let actual = lexer.scan_number();

        assert_eq!(TokenKind::INTEGER(12345), actual.kind);
        assert_eq!((1, 1), actual.position);

        // scan_number() 内部でオフセットがちゃんと進んでいるか.
        let cur_position = lexer.current_position();
        assert_eq!((6, 1), cur_position);

        // 文字列が切り取られているか.
        let cur_looking_string = lexer.contents;
        assert_eq!(cur_looking_string, "");
    }

    #[test]
    fn test_scan_one_token_with_single_int() {
        let expected_int = Token::new((1, 1), TokenKind::INTEGER(12345));
        let expected_eof = Token::new((6, 1), TokenKind::EOF);
        let mut lexer = create_lexer("12345");
        let actual = lexer.scan_one_token();

        assert_eq!(Some(expected_int), actual);

        let should_eof = lexer.scan_one_token();
        assert_eq!(Some(expected_eof), should_eof);
    }

    #[test]
    fn test_scan_one_token_with_invalid_symbol() {
        let mut lexer = create_lexer("@");
        let actual = lexer.scan_one_token();
        assert_eq!(None, actual);
    }

    #[test]
    fn test_scan_symbol() {
        let expected = Token::new((1, 1), TokenKind::PLUS);
        let mut lexer = create_lexer("+  ");
        let actual = lexer.scan_symbol(TokenKind::PLUS);

        assert_eq!(expected, actual);

        // オフセットが進んでいるか
        let cur_position = lexer.current_position();
        assert_eq!((2, 1), cur_position);

        // 文字列が切り取られているか.
        let cur_looking_string = lexer.contents;
        assert_eq!(cur_looking_string, "  ");
    }

    #[test]
    fn test_skip_whitespace() {
        let expected_eof = Token::new((6, 1), TokenKind::EOF);

        let mut lexer = create_lexer("     ");
        let whitespace = lexer.skip_whitespace();

        assert!(whitespace.should_ignore());

        assert_eq!(6, lexer.column);
        let should_eof = lexer.scan_one_token();

        assert_eq!(Some(expected_eof), should_eof);
    }

    fn create_lexer(input: &str) -> Lexer {
        Lexer::new(input.to_string())
    }
}
