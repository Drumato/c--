use crate::compiler::error::{Error, ErrorKind, ErrorMsg};
use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::frontend::token::{Token, TokenKind};
use crate::compiler::frontend::Manager;

impl Manager {
    pub fn parse(&mut self) {
        self.expr = self.parse_expression();
    }

    fn parse_expression(&mut self) -> Node {
        let mut left_node: Node = self.parse_term(); // TODO: multiple_division later
        loop {
            // TODO: 減算記号も加える
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
            // TODO: Manager構造体にエラーのVectorを用意して,そこに追加していくほうが良さそう
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
        &self.tokens[self.cur_token]
    }

    fn looking_token_clone(&mut self) -> Token {
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
        let source_file = SrcFile::new("100");
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);

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
        let source_file = SrcFile::new("+");
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);

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

        let source_file = SrcFile::new("100 + 200");
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);

        // 加算ノードを受け取れるか.
        let actual = manager.parse_expression();
        assert_eq!(expected, actual);
    }
}