use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Function, Node, NodeKind, Priority};
use crate::compiler::frontend::token;
use crate::compiler::frontend::types::Type;
use crate::compiler::frontend::variable::Variable;
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
        if self.is_typename() {
            return self.parse_declaration();
        }
        let cur = self.looking_token_clone();
        match cur.kind {
            // return-statement
            TokenKind::RETURN => self.parse_return_stmt(),
            // goto-statement
            TokenKind::GOTO => self.parse_goto_stmt(),
            // labeled-statement
            TokenKind::IDENTIFIER(_name) if self.next_token_is(TokenKind::COLON) => {
                self.parse_labeled_stmt()
            }
            // expression-statement
            _ => {
                let current_position = self.looking_token_clone().position;
                let expression = self.parse_expression();
                self.expect(TokenKind::SEMICOLON);
                Node::new_exprstmt(current_position, expression)
            }
        }
    }

    // declaration =  basetype declarator type-suffix ("=" lvar-initializer)? ";"
    //              | basetype ";"
    fn parse_declaration(&mut self) -> Node {
        let current_position = self.looking_token_clone().position;
        let base_type = self.consume_base_type().unwrap();

        let (var_name, var_type) = self.parse_declarator(base_type);

        // TODO: type-suffix は考えない

        // TODO: 今は初期化を実装する必要はない.
        // if self.consume(TokenKind::ASSIGN) {
        // ASTノードに意味を持たせない実装もあるが,ここでは持たせている.
        // return Node::new_declaration(current_position, base_type);
        // }

        // マップにエントリを登録
        let local_var = Variable::init_local(var_type.clone());
        self.var_map.insert(var_name.to_string(), local_var);
        self.expect(TokenKind::SEMICOLON);
        Node::new_declaration(current_position, var_name, var_type)
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

    fn parse_goto_stmt(&mut self) -> Node {
        // goto_stmt -> goto + identifier + `;`
        // gotol文開始位置を保存
        let current_position = self.looking_token_clone().position;
        self.expect(TokenKind::GOTO);
        let label_name = self.expect_ident();
        self.expect(TokenKind::SEMICOLON);

        Node::new_goto(current_position, label_name)
    }
    fn parse_labeled_stmt(&mut self) -> Node {
        // labeled_stmt -> identifier + `:` + statement
        // label文開始位置を保存
        let current_position = self.looking_token_clone().position;
        let label_name = self.expect_ident();
        self.expect(TokenKind::COLON);
        let any_statement = self.parse_statement();

        Node::new_labeled(current_position, label_name, any_statement)
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
    // expr -> assign ("," assign)*
    fn parse_expression(&mut self) -> Node {
        let assign_node = self.parse_assign();
        assign_node
    }
    // assign -> conditional (assign-op assign)?
    // assign-op = "=" | "+=" | "-=" | "*=" | "/=" | "<<=" | ">>="
    //           | "&=" | "|=" | "^="
    #[allow(unconditional_recursion)]
    fn parse_assign(&mut self) -> Node {
        // TODO: 今はconditionalをパースできない
        // 現状最も優先度の低いadditiveを呼び出しておく
        let lvalue_node = self.parse_additive();

        // TODO: 今は `=` のみサポート
        let current_position = self.looking_token_clone().position;
        if self.consume(TokenKind::ASSIGN) {
            return Node::new_assign(current_position, lvalue_node, self.parse_assign());
        }

        lvalue_node
    }
    // additive -> multiplicative | additive-expression (+|-) multiplicative-expression
    fn parse_additive(&mut self) -> Node {
        let mut left_node: Node = self.parse_multiplicative();

        // チェックする演算子の列挙
        let operators = self.current_prio_operators(Priority::ADDITIVE);
        loop {
            // いずれにも合致しなければ終了
            if !self.current_token_is_in(&operators) {
                break;
            }

            // 演算子トークンを退避
            let cur_token = self.looking_token_clone();
            self.read_token();
            let right_node = self.parse_multiplicative();

            // コンストラクト
            left_node = Node::new_binary_node(&cur_token, left_node, right_node);
        }

        left_node
    }
    // multiplicative -> unary-expression | multiplicative-expression (*|/) unary-expression
    fn parse_multiplicative(&mut self) -> Node {
        let mut left_node: Node = self.parse_unary();

        // チェックする演算子の列挙
        let operators = self.current_prio_operators(Priority::MULTIPLICATIVE);
        loop {
            // いずれにも合致しなければ終了
            if !self.current_token_is_in(&operators) {
                break;
            }

            // 演算子トークンを退避
            let cur_token = self.looking_token_clone();
            self.read_token();
            let right_node = self.parse_unary();

            // コンストラクト
            left_node = Node::new_binary_node(&cur_token, left_node, right_node);
        }

        left_node
    }
    // unary -> primary-expression | ("+" | "-")? unary-expression
    fn parse_unary(&mut self) -> Node {
        let cur = self.looking_token_clone();
        match cur.kind {
            TokenKind::MINUS => {
                self.read_token();
                Node::new_unary_node(&cur, self.parse_unary())
            }
            _ => self.parse_primary(),
        }
    }
    // primary -> identifier | constant | ( expression ) | string-literal | generic_selection
    fn parse_primary(&mut self) -> Node {
        let cur = self.looking_token_clone();
        self.read_token();
        match cur.kind {
            TokenKind::INTEGER(val) => Node::new(cur.position, NodeKind::INTEGER(val)),
            // TODO: 関数コール等をチェックすべき
            TokenKind::IDENTIFIER(name) => {
                Node::new(cur.position, NodeKind::IDENTIFIER(name.to_string()))
            }
            TokenKind::LPAREN => {
                let paren_expr = self.parse_expression();
                self.expect(TokenKind::RPAREN);
                paren_expr
            }
            // エラーを吐いてINVALIDを返す
            _ => {
                let err = Error::new(ErrorKind::Parse, cur.position, ErrorMsg::MustBePrimary);
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
    fn current_prio_operators(&mut self, priority: Priority) -> Vec<TokenKind> {
        match priority {
            Priority::ADDITIVE => vec![TokenKind::PLUS, TokenKind::MINUS],
            // TODO: '%' を足す
            Priority::MULTIPLICATIVE => vec![TokenKind::ASTERISK, TokenKind::SLASH],
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
        let cur = self.looking_token_clone();
        if cur.kind != tk {
            panic!(
                "unexpected token -> expected {:?} but got {:?}",
                tk, cur.position
            );
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
    fn next_token_is(&mut self, tk: TokenKind) -> bool {
        if self.tokens.len() <= self.next_token {
            return false;
        }
        self.tokens[self.next_token].clone().kind == tk
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
            NodeKind::MUL(Box::new(left_node), Box::new(right_node)),
        );
        let return_stmt = Node::new_return((2, 3), expr);

        let func = Function {
            name: "main".to_string(),
            def_position: (1, 1),
            stmts: vec![return_stmt],
            frame_size: 0,
        };

        integration_test_parser("int main(){\n  return 200 * 100;\n}", func);
    }
    #[test]
    fn test_parse_assign_expression() {
        let declaration = Node::new_declaration((2, 3), "x".to_string(), Type::new_integer());
        let var = Node::new((3, 3), NodeKind::IDENTIFIER("x".to_string()));
        let rvalue = Node::new((3, 7), NodeKind::INTEGER(30));
        let assign = Node::new_assign((3, 5), var, rvalue);
        let expr_stmt = Node::new_exprstmt((3, 3), assign);

        let func = Function {
            name: "main".to_string(),
            def_position: (1, 1),
            stmts: vec![declaration, expr_stmt],
            frame_size: 0,
        };

        integration_test_parser("int main(){\n  int x;\n  x = 30;\n}", func);
    }

    #[test]
    fn test_parse_labeled_statement() {
        let goto_stmt = Node::new_goto((2, 1), "fin".to_string());
        let return_stmt = Node::new_return((3, 7), Node::new((3, 14), NodeKind::INTEGER(2)));
        let labeled_stmt = Node::new_labeled((3, 1), "fin".to_string(), return_stmt);

        let func = Function {
            name: "main".to_string(),
            def_position: (1, 1),
            stmts: vec![goto_stmt, labeled_stmt],
            frame_size: 0,
        };

        integration_test_parser("int main(){\ngoto fin;\nfin:  return 2;\n}", func);
    }

    #[test]
    fn test_parse_primary() {
        let expected = Node::new((1, 1), NodeKind::INTEGER(100));
        let mut manager = preprocess("100");

        // 整数ノードをパースできているか
        let actual = manager.parse_primary();
        assert_eq!(expected, actual);

        // 次のトークンを指すことができているか
        assert_eq!(1, manager.cur_token);
        assert_eq!(2, manager.next_token);
    }

    #[test]
    fn test_parse_primary_without_integer() {
        let expected = Node::new((0, 0), NodeKind::INVALID);
        let mut manager = preprocess("+");

        // エラーを出せているか
        let actual = manager.parse_primary();
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
