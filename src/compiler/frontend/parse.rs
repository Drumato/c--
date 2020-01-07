use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::frontend::token;
use crate::compiler::frontend::Manager;
use crate::error::{Error, ErrorKind, ErrorMsg};
use token::{Token, TokenKind};

impl Manager {
    pub fn parse(&mut self) {
        self.expr = self.parse_expression();
    }

    fn parse_expression(&mut self) -> Node {
        let mut left_node: Node = self.parse_term();
        loop {
            if !self.current_token_is_in(vec![TokenKind::PLUS]) {
                break;
            }
            let cur_token = &self.looking_token_clone();
            self.read_token();
            if let TokenKind::PLUS = cur_token.kind {
                let right_node = self.parse_term();
                let add_node = NodeKind::ADD(Box::new(left_node), Box::new(right_node));
                left_node = Node::new(cur_token.position, add_node);
            }
        }
        left_node
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
    fn current_token_is_in(&mut self, tks: Vec<TokenKind>) -> bool {
        for t in tks {
            if self.looking_token().kind == t {
                return true;
            }
        }
        false
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

    #[test]
    fn test_parse_expression_with_addition() {
        let left_node = Node::new((1, 1), NodeKind::INTEGER(100));
        let right_node = Node::new((7, 1), NodeKind::INTEGER(200));
        let expected = Node::new(
            (5, 1),
            NodeKind::ADD(Box::new(left_node), Box::new(right_node)),
        );

        let mut manager = preprocess("100 + 200");

        // 加算ノードを受け取れるか.
        let actual = manager.parse_expression();
        assert_eq!(expected, actual);
    }

    fn preprocess(input: &str) -> Manager {
        let source_file = SrcFile::new(input);
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);
        manager
    }
}
