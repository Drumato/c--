use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Function, Node, NodeKind, Priority};
use crate::compiler::frontend::token;
use crate::compiler::frontend::types::Type;
use crate::error::{Error, ErrorKind, ErrorMsg};
use token::{Token, TokenKind};

impl Manager {
    pub fn parse(&mut self) {
        self.parse_toplevel();
    }

    // toplevel -> (global-var | func-def)*
    fn parse_toplevel(&mut self) {
        loop {
            if !self.is_function() {
                // 本当はグローバル変数定義だけど,今はbreak
                break;
            }

            self.entry_func = self.parse_function();
        }
    }

    // declarator = "*"* ("(" declarator ")" | ident) type-suffix
    fn parse_declarator(&mut self, base_type: Type) -> (String, Type) {
        let mut covered_type = base_type;

        // *がある間ポインタ型にくるむ
        loop {
            if !self.consume(TokenKind::ASTERISK) {
                break;
            }
            covered_type = Type::pointer_to(covered_type);
        }

        if self.consume(TokenKind::LPAREN) {
            let placeholder = Type::new_unknown();
            let (name, new_type) = self.parse_declarator(placeholder);
            self.expect(TokenKind::RPAREN);
            return (name, new_type);
        }

        let name = self.expect_ident();
        (name, covered_type)
    }

    // function = basetype declarator "(" params? ")" ("{" stmt* "}" | ";")
    // params   = param ("," param)* | "void"
    // param    = basetype declarator type-suffix
    fn parse_function(&mut self) -> Function {
        let current_position = self.looking_token_clone().position;

        let base_type = self.consume_base_type().unwrap();
        let (name, _dec_type) = self.parse_declarator(base_type);

        let mut func = Function::init(name, current_position);

        self.expect(TokenKind::LPAREN);
        // 引数は無視
        self.expect(TokenKind::RPAREN);

        // 関数のボディ
        self.expect(TokenKind::LBRACKET);

        loop {
            if self.consume(TokenKind::RBRACKET) {
                break;
            }

            let stmt = self.parse_statement();
            func.stmts.push(stmt);
        }

        func
    }
    fn parse_statement(&mut self) -> Node {
        let cur = self.looking_token_clone();
        match cur.kind {
            TokenKind::RETURN => self.parse_return_stmt(),
            _ => panic!("statement must start with return"),
        }
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
    fn consume_base_type(&mut self) -> Option<Type> {
        // TODO: 完全な実装でない
        if !self.is_typename() {
            return None;
        }

        let int_type = self.looking_token_clone();
        self.read_token();

        Some(Type::from_token(int_type))
    }
    fn is_function(&mut self) -> bool {
        // 現在位置を退避,後で戻す
        let cur_token = self.cur_token;
        let next_token = self.next_token;
        let mut is_func = false;

        // 本当は6.9.1 Function definitions に従って正しくチェックする必要あり
        if let Some(base_type) = self.consume_base_type() {
            if !self.consume(TokenKind::SEMICOLON) {
                let (name, _type) = self.parse_declarator(base_type);
                is_func = (name.len() != 0) && self.consume(TokenKind::LPAREN);
            }
        }

        self.cur_token = cur_token;
        self.next_token = next_token;
        is_func
    }
    fn expect_ident(&mut self) -> String {
        if let TokenKind::IDENTIFIER(name) = self.looking_token_clone().kind {
            self.read_token();
            return name.to_string();
        }

        panic!("expected typename");
    }
    fn is_typename(&mut self) -> bool {
        match self.looking_token().kind {
            TokenKind::INT | TokenKind::VOID => true,
            _ => false,
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
    fn consume(&mut self, tk: TokenKind) -> bool {
        if self.looking_token_clone().kind != tk {
            return false;
        }

        self.read_token();
        true
    }
    fn expect(&mut self, tk: TokenKind) {
        if self.looking_token_clone().kind != tk {
            panic!("unexpected token");
        }

        self.read_token();
    }
    fn looking_token(&mut self) -> &Token {
        if self.tokens.len() <= self.cur_token {
            return &token::GLOBAL_EOF_TOKEN;
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
    fn test_parse_main_func() {
        let left_node = Node::new((2, 10), NodeKind::INTEGER(200));
        let right_node = Node::new((2, 16), NodeKind::INTEGER(100));
        let expr = Node::new(
            (2, 14),
            NodeKind::SUB(Box::new(left_node), Box::new(right_node)),
        );
        let return_stmt = Node::new_return((2, 3), expr);

        let func = Function {
            name: "main".to_string(),
            def_position: (1, 1),
            stmts: vec![return_stmt],
        };

        integration_test_parser("int main(){\n  return 200 - 100;\n}", func);
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
    fn integration_test_parser(input: &str, expected: Function) {
        let mut manager = preprocess(input);
        manager.parse();

        assert_eq!(expected, manager.entry_func)
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
