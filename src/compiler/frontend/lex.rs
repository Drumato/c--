use crate::compiler::frontend;
use frontend::manager::Manager;
use frontend::token::{Position, Token, TokenKind};

use std::collections::BTreeMap;
pub fn tokenize(manager: &mut Manager) {
    // ソースコードのメモリコピーをするのは,後ほどエラーメッセージでソースコード本体を表示するため.
    // 本体の変更はしたくない.
    let mut lexer = Lexer::new(manager.src_file.contents.to_string());

    // 予約語の構築
    lexer.build_keywords();

    let tokens = lexer.build_tokens();
    manager.tokens = tokens;
}

#[allow(dead_code)]
struct Lexer {
    column: usize,                         // x軸の座標
    row: usize,                            // y軸の座標
    contents: String, // メモリコピーし, SrcFile構造体の文字列を破壊しないように
    keywords: BTreeMap<String, TokenKind>, // 予約語をO(1)で取り出すためのメンバ
}

impl Lexer {
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
            '-' => Some(self.scan_symbol(TokenKind::MINUS)),
            ';' => Some(self.scan_symbol(TokenKind::SEMICOLON)),

            // アルファベットの場合
            c if c.is_ascii_alphabetic() => Some(self.scan_word()),
            '_' => Some(self.scan_word()),

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
    // 文字列を切り取って,予約語/識別子トークンを返す
    pub fn scan_word(&mut self) -> Token {
        // 現在のオフセットを退避
        let cur_position = self.current_position();

        // 空白,改行までの文字列を読み取る
        let word = Self::take_conditional_string(&self.contents, |c| c != &' ' && c != &'\n');

        // オフセットを進める
        self.skip_offset(word.len());

        // 予約語かチェック
        if let Some(t_kind) = self.keywords.get(&word) {
            return Token::new(cur_position, t_kind.clone());
        }

        // TODO: 識別子
        panic!("not implemented when parse identifier");
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

    // 予約語の構築
    fn build_keywords(&mut self) {
        // 命令
        self.keywords
            .insert("return".to_string(), TokenKind::RETURN);
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
        (self.row, self.column)
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
            Token::new((1, 7), TokenKind::PLUS),
            Token::new((1, 9), TokenKind::INTEGER(678910)),
            Token::new((1, 15), TokenKind::EOF),
        ];
        integration_test_lexing("12345 + 678910", expected_tokens);
    }

    #[test]
    fn test_lex_return_statement() {
        let expected_tokens = vec![
            Token::new((1, 1), TokenKind::RETURN),
            Token::new((1, 8), TokenKind::INTEGER(30)),
            Token::new((1, 10), TokenKind::SEMICOLON),
            Token::new((1, 11), TokenKind::EOF),
        ];

        integration_test_lexing("return 30;", expected_tokens);
    }

    #[test]
    fn test_addition_expression() {
        let expected_tokens = vec![
            Token::new((1, 1), TokenKind::INTEGER(30)),
            Token::new((1, 4), TokenKind::PLUS),
            Token::new((1, 6), TokenKind::INTEGER(40)),
            Token::new((1, 8), TokenKind::EOF),
        ];

        integration_test_lexing("30 + 40", expected_tokens);
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
        assert_eq!((1, 6), cur_position);

        // 文字列が切り取られているか.
        let cur_looking_string = lexer.contents;
        assert_eq!(cur_looking_string, "");
    }

    #[test]
    fn test_skip_whitespace() {
        let expected_eof = Token::new((1, 6), TokenKind::EOF);

        let mut lexer = create_lexer("     ");
        let whitespace = lexer.skip_whitespace();

        assert!(whitespace.should_ignore());

        assert_eq!(6, lexer.column);
        let should_eof = lexer.scan_one_token();

        assert_eq!(Some(expected_eof), should_eof);
    }

    #[test]
    fn test_build_keywords() {
        let mut lexer = create_lexer("");
        lexer.build_keywords();

        assert_eq!(1, lexer.keywords.len());

        assert!(lexer.keywords.contains_key("return"));
    }

    // 総合テスト関数
    fn integration_test_lexing(input: &str, expected_tokens: Vec<Token>) {
        let mut lexer = create_lexer(input);
        lexer.build_keywords();
        let tokens = lexer.build_tokens();

        for (i, actual) in tokens.iter().enumerate() {
            assert_eq!(&expected_tokens[i], actual);
        }
    }
    fn create_lexer(input: &str) -> Lexer {
        Lexer::new(input.to_string())
    }
}
