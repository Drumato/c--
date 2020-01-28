use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::frontend::types::Type;
use crate::error::{Error, ErrorKind, ErrorMsg};

impl Manager {
    pub fn semantics(&mut self) {
        let mut statements = self.entry_func.stmts.clone();
        let statements_number = self.entry_func.stmts.len();
        for stmt_idx in 0..statements_number {
            self.walk_statement(&mut statements[stmt_idx]);
        }
        self.entry_func.stmts = statements;
    }
    fn walk_statement(&mut self, stmt: &mut Node) {
        match stmt.kind {
            NodeKind::RETURNSTMT(ref mut return_expr) => {
                self.walk_expression(return_expr);
            }
            NodeKind::LABELEDSTMT(ref mut _label_name, ref mut any_stmt) => {
                self.walk_statement(any_stmt);
            }
            NodeKind::EXPRSTMT(ref mut expr) => {
                self.walk_expression(expr);
            }
            NodeKind::GOTOSTMT(ref mut _label_name) => {}
            NodeKind::DECLARATION(ref mut _name, ref mut _type) => {}
            _ => {
                self.output_invalid_node_type_error(stmt.position);
            }
        }
    }
    fn walk_expression(&mut self, n: &mut Node) -> Type {
        match n.kind {
            NodeKind::INTEGER(_val) => {
                n.ctype = Type::new_integer();
                n.ctype.clone()
            }
            NodeKind::IDENTIFIER(ref name) => {
                if let Some(var) = self.var_map.get(name) {
                    return var.ctype.clone();
                }
                Type::new_unknown()
            }

            NodeKind::ASSIGN(ref mut lv, ref mut rv) => {
                let left_type = self.walk_expression(lv);
                let right_type = self.walk_expression(rv);
                if left_type == right_type {
                    n.ctype = left_type;
                    return right_type;
                }
                self.output_type_difference_error(lv.position);
                Type::new_unknown()
            }
            // 単項演算
            NodeKind::NEGATIVE(ref mut inner) => {
                let inner_type = self.walk_expression(inner);
                n.ctype = inner_type.clone();
                return inner_type;
            }
            // 二項演算
            NodeKind::ADD(ref mut left, ref mut right)
            | NodeKind::SUB(ref mut left, ref mut right)
            | NodeKind::MUL(ref mut left, ref mut right)
            | NodeKind::DIV(ref mut left, ref mut right) => {
                let left_type = self.walk_expression(left);
                let right_type = self.walk_expression(right);
                if left_type == right_type {
                    n.ctype = left_type;
                    return right_type;
                }
                // floatとintなど,暗黙的型変換が可能な組み合わせならそれを返す
                // self.implicit_type_inference();

                self.output_type_difference_error(left.position);
                Type::new_unknown()
            }
            _ => {
                self.output_invalid_node_type_error(n.position);
                Type::new_unknown()
            }
        }
    }
    fn output_type_difference_error(&mut self, position: (usize, usize)) {
        let err = Error::new(
            ErrorKind::Type,
            position,
            ErrorMsg::MustBeSameTypeInBinaryOperation,
        );
        err.found();
    }
    fn output_invalid_node_type_error(&mut self, position: (usize, usize)) {
        let err = Error::new(ErrorKind::Type, position, ErrorMsg::InvalidNodeCantHaveType);
        err.found();
    }
}

// ASTノードへの型付けについてテスト
#[cfg(test)]
mod walk_tests {
    use super::*;
    use crate::compiler::file::SrcFile;
    use crate::compiler::frontend::lex;
    use crate::compiler::frontend::node::Function;
    #[test]
    fn test_add_types_to_ast_with_main_func() {
        let mut left = Node::new((1, 21), NodeKind::INTEGER(100));
        left.ctype = Type::new_integer();
        let mut right = Node::new((1, 27), NodeKind::INTEGER(200));
        right.ctype = Type::new_integer();
        let mut subtraction = Node::new((1, 25), NodeKind::SUB(Box::new(left), Box::new(right)));
        subtraction.ctype = Type::new_integer();
        let return_stmt = Node::new_return((1, 14), subtraction);
        let expected = Function {
            name: "main".to_string(),
            stmts: vec![return_stmt],
            def_position: (1, 1),
            frame_size: 0,
        };

        integration_test_semantics("int main() { return 100 - 200; }", expected);
    }

    // 統合テスト用
    fn integration_test_semantics(input: &str, expected: Function) {
        let mut manager = preprocess(input);
        manager.semantics();

        assert_eq!(manager.entry_func, expected);
    }

    fn preprocess(input: &str) -> Manager {
        let source_file = SrcFile {
            abs_path: "testcase".to_string(),
            contents: input.to_string(),
        };
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);
        manager.parse();
        manager
    }
}
