use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Node, NodeKind, Priority};
use crate::compiler::frontend::token;
use crate::error::{Error, ErrorKind, ErrorMsg};
use token::{Token, TokenKind};

impl Manager {
    pub fn parse(&mut self) {
        self.expr = self.parse_return_stmt();
    }

    fn parse_return_stmt(&mut self) -> Node {
        // return_stmt -> return + expr + `;`
        // return文開始位置を保存
        let current_position = self.looking_token_clone().position;

        self.expect(TokenKind::RETURN);

        let return_expr = self.parse_expression();

        self.expect(TokenKind::SEMICOLON);

        Node::new_return(current_position, return_expr)
    }
    fn parse_expression(&mut self) -> Node {
        // expr -> term | expr_1 (`+`/`-` term)+
        // 最初はPriority::ADDSUBで始まる
        self.parse_node_current_prio()
    }
    fn parse_node_current_prio(&mut self) -> Node {
        // 現在の優先順位に対応した関数を呼ぶ
        match self.priority {
            Priority::ADDSUB => self.parse_binary_node(),
        }
    }
    fn parse_binary_node(&mut self) -> Node {
        // 現在の優先順位よりひとつ上の関数を呼ぶ
        let mut left_node: Node = self.parse_node_next_prio();
        // チェックする演算子の列挙
        let operators = self.current_prio_operators();
        loop {
            // いずれにも合致しなければ終了
            if !self.current_token_is_in(&operators) {
                break;
            }

            // 演算子トークンを退避
            let cur_token = self.looking_token_clone();
            self.read_token();
            let right_node = self.parse_node_next_prio();

            // コンストラクト
            left_node = Node::new_binary_node(&cur_token, left_node, right_node);
        }
        left_node
    }
    fn parse_node_next_prio(&mut self) -> Node {
        // 各関数におけるより優先度の高い関数を定義しておく
        match self.priority {
            Priority::ADDSUB => self.parse_term(),
        }
    }
    fn parse_term(&mut self) -> Node {
        let cur = self.looking_token_clone();
        self.read_token();
        match cur.kind {
            TokenKind::INTEGER(val) => Node::new(cur.position, NodeKind::INTEGER(val)),
            // エラーを吐いてINVALIDを返す
            _ => {
                let err = Error::new(ErrorKind::Parse, cur.position, ErrorMsg::MustBeInteger);
                err.found();
                Node::new((0, 0), NodeKind::INVALID)
            }
        }
    }
    fn current_token_is_in(&mut self, tks: &Vec<TokenKind>) -> bool {
        for t in tks {
            if &self.looking_token().kind == t {
                return true;
            }
        }
        false
    }
    fn current_prio_operators(&mut self) -> Vec<TokenKind> {
        match self.priority {
            Priority::ADDSUB => vec![TokenKind::PLUS, TokenKind::MINUS],
        }
    }
    fn expect(&mut self, tk: TokenKind) {
        if self.looking_token_clone().kind != tk {
            panic!("unexpected token");
        }

        self.read_token();
    }
    fn looking_token(&mut self) -> &Token {
        if self.tokens.len() <= self.cur_token {
            if self.tokens.len() <= self.cur_token {
                return &token::GLOBAL_EOF_TOKEN;
            }
        }
        &self.tokens[self.cur_token]
    }

    fn looking_token_clone(&mut self) -> Token {
        if self.tokens.len() <= self.cur_token {
            let last_token_position = self.tokens.last().unwrap().position;
            return Token::new(last_token_position, TokenKind::EOF);
        }
        self.tokens[self.cur_token].clone()
    }

    fn read_token(&mut self) {
        self.cur_token += 1;
        self.next_token += 1;
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::compiler::file::SrcFile;
    use crate::compiler::frontend::lex;
    #[test]
    fn test_parse_return_statement() {
        let left_node = Node::new((1, 8), NodeKind::INTEGER(200));
        let right_node = Node::new((1, 14), NodeKind::INTEGER(100));
        let expr = Node::new(
            (1, 12),
            NodeKind::SUB(Box::new(left_node), Box::new(right_node)),
        );

        let expected = Node::new_return((1, 1), expr);

        integration_test_parser("return 200 - 100;", expected);
    }

    #[test]
    fn test_parse_term() {
        let expected = Node::new((1, 1), NodeKind::INTEGER(100));
        let mut manager = preprocess("100");

        // 整数ノードをパースできているか
        let actual = manager.parse_term();
        assert_eq!(expected, actual);

        // 次のトークンを指すことができているか
        assert_eq!(1, manager.cur_token);
        assert_eq!(2, manager.next_token);
    }

    #[test]
    fn test_parse_term_without_integer() {
        let expected = Node::new((0, 0), NodeKind::INVALID);
        let mut manager = preprocess("+");

        // エラーを出せているか
        let actual = manager.parse_term();
        assert_eq!(expected, actual);
    }

    // 総合テスト用
    // 後々Function構造体に書き換える?
    fn integration_test_parser(input: &str, expected: Node) {
        let mut manager = preprocess(input);
        manager.parse();

        assert_eq!(expected, manager.expr)
    }

    fn preprocess(input: &str) -> Manager {
        let source_file = SrcFile {
            abs_path: "testcase".to_string(),
            contents: input.to_string(),
        };
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);
        manager
    }
}
