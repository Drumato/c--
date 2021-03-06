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

            self.params.clear();
            self.var_map.clear();
            let mut func = self.parse_function();
            func.local_map = self.var_map.clone();
            func.params = self.params.clone();
            self.functions.push(func);
        }
    }

    // function = basetype declarator "(" params? ")" ("{" stmt* "}" | ";")
    // params   = param ("," param)* | "void"
    // param    = basetype declarator type-suffix
    fn parse_function(&mut self) -> Function {
        let current_position = self.looking_token_clone().position;

        let base_type = self.consume_base_type().unwrap();
        let (name, dec_type) = self.parse_declarator(base_type);

        let mut func = Function::init(name, current_position, dec_type);

        self.expect(TokenKind::LPAREN);
        // voidは単純に無視すればいい
        self.consume(TokenKind::VOID);

        // 引数のパース
        loop {
            if self.consume(TokenKind::RPAREN) {
                break;
            }

            let base_type = self.consume_base_type().unwrap();
            let (arg_name, dec_type) = self.parse_declarator(base_type);
            let argument = Variable::init_local(dec_type.clone());
            self.params.insert(arg_name.to_string(), argument);

            self.consume(TokenKind::COMMA);
        }

        // 関数のボディ
        self.expect(TokenKind::LBRACE);

        loop {
            if self.consume(TokenKind::RBRACE) {
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
            // compound-statement
            TokenKind::LBRACE => self.parse_compound_stmt(),
            // selection-statement
            TokenKind::IF => self.parse_selection_stmt(),
            // iteration-statement
            TokenKind::FOR => self.parse_for_stmt(),
            TokenKind::DO => self.parse_do_while_stmt(),
            TokenKind::WHILE => self.parse_while_stmt(),
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

    // compound_stmt -> `{` statement * n `}`
    fn parse_compound_stmt(&mut self) -> Node {
        let current_position = self.looking_token_clone().position;
        self.expect(TokenKind::LBRACE);

        let mut stmts: Vec<Node> = Vec::new();
        loop {
            if self.consume(TokenKind::RBRACE) {
                break;
            }
            stmts.push(self.parse_statement());
        }
        Node::new_compound(current_position, stmts)
    }
    // do_while_stmt -> do statement while `(` expression `)`
    fn parse_do_while_stmt(&mut self) -> Node {
        let current_position = self.looking_token_clone().position;
        self.expect(TokenKind::DO);

        let stmt = self.parse_statement();

        self.expect(TokenKind::WHILE);
        self.expect(TokenKind::LPAREN);
        let cond_expr = self.parse_expression();
        self.expect(TokenKind::RPAREN);

        self.expect(TokenKind::SEMICOLON);

        Node::new_do_while(current_position, stmt, cond_expr)
    }
    // while_stmt -> while `(` expression `)` statement
    fn parse_while_stmt(&mut self) -> Node {
        let current_position = self.looking_token_clone().position;
        self.expect(TokenKind::WHILE);
        self.expect(TokenKind::LPAREN);

        let cond_expr = self.parse_expression();
        self.expect(TokenKind::RPAREN);

        let stmt = self.parse_statement();
        Node::new_while(current_position, cond_expr, stmt)
    }

    // for_stmt -> for `(` clause_1 `;` expr_2 `;` expr_3 `)` statement
    fn parse_for_stmt(&mut self) -> Node {
        let current_position = self.looking_token_clone().position;

        let mut clause = Node::new_nop();
        let mut expr_2 = Node::new_nop();
        self.expect(TokenKind::FOR);
        self.expect(TokenKind::LPAREN);

        if !self.consume(TokenKind::SEMICOLON) {
            clause = self.parse_expression();
            self.expect(TokenKind::SEMICOLON);
        }
        if !self.consume(TokenKind::SEMICOLON) {
            expr_2 = self.parse_expression();
            self.expect(TokenKind::SEMICOLON);
        }
        let expr_3 = self.parse_expression();
        self.expect(TokenKind::RPAREN);

        let stmt = self.parse_statement();
        Node::new_for(current_position, clause, expr_2, expr_3, stmt)
    }

    fn parse_selection_stmt(&mut self) -> Node {
        // selection_stmt -> if + `(` + expression `)` + statement
        // if文開始位置を保存
        let current_position = self.looking_token_clone().position;
        self.expect(TokenKind::IF);
        self.expect(TokenKind::LPAREN);
        let cond_expr = self.parse_expression();
        self.expect(TokenKind::RPAREN);

        let any_statement = self.parse_statement();

        if !self.consume(TokenKind::ELSE) {
            return Node::new_if(current_position, cond_expr, any_statement);
        }

        // if-else
        let alter_statement = self.parse_statement();
        Node::new_if_else(current_position, cond_expr, any_statement, alter_statement)
    }

    // goto_stmt -> goto + identifier + `;`
    fn parse_goto_stmt(&mut self) -> Node {
        let current_position = self.looking_token_clone().position;
        self.expect(TokenKind::GOTO);
        let label_name = self.expect_ident();
        self.expect(TokenKind::SEMICOLON);

        Node::new_goto(current_position, label_name)
    }
    // labeled_stmt -> identifier + `:` + statement
    fn parse_labeled_stmt(&mut self) -> Node {
        let current_position = self.looking_token_clone().position;
        let label_name = self.expect_ident();
        self.expect(TokenKind::COLON);
        let any_statement = self.parse_statement();

        Node::new_labeled(current_position, label_name, any_statement)
    }
    // return_stmt -> return + expr + `;`
    fn parse_return_stmt(&mut self) -> Node {
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
    // unary -> postfix-expression | ("+" | "-")? unary-expression
    fn parse_unary(&mut self) -> Node {
        let cur = self.looking_token_clone();
        match cur.kind {
            TokenKind::MINUS => {
                self.read_token();
                Node::new_unary_node(&cur, self.parse_unary())
            }
            _ => self.parse_postfix(),
        }
    }
    // postfix -> primary-expression | postfix_expression `(` argument-expression-list_opt `)`
    fn parse_postfix(&mut self) -> Node {
        let primary_expr = self.parse_primary();
        let cur = self.looking_token_clone();

        match cur.kind {
            // 関数呼び出し
            // TODO: とりあえず引数なし
            TokenKind::LPAREN => {
                self.read_token();
                let mut params: Vec<Node> = Vec::new();
                loop {
                    if self.consume(TokenKind::RPAREN) {
                        break;
                    }

                    params.push(self.parse_assign());
                    self.consume(TokenKind::COMMA);
                }
                Node::new_call(cur.position, primary_expr, params)
            }
            // ただのprimary-expressionとしてパース
            _ => primary_expr,
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
mod parser_tests {}
