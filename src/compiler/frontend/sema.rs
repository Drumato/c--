use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::frontend::types::Type;
use crate::error::{Error, ErrorKind, ErrorMsg};

impl Manager {
    pub fn semantics(&mut self) {
        let mut n = self.expr.clone();
        self.walk_expression(&mut n);
        self.expr = n;
    }
    fn walk_expression(&mut self, n: &mut Node) -> Type {
        match n.kind {
            NodeKind::RETURNSTMT(ref mut return_expr) => self.walk_expression(return_expr),
            NodeKind::INTEGER(_val) => {
                n.ctype = Type::new_integer();
                n.ctype.clone()
            }

            // 二項演算
            NodeKind::ADD(ref mut left, ref mut right)
            | NodeKind::SUB(ref mut left, ref mut right) => {
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
            NodeKind::INVALID => {
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
    #[test]
    fn test_add_types_to_ast_with_return_stmt() {
        let mut left = Node::new((1, 8), NodeKind::INTEGER(100));
        left.ctype = Type::new_integer();
        let mut right = Node::new((1, 14), NodeKind::INTEGER(200));
        right.ctype = Type::new_integer();
        let mut subtraction = Node::new((1, 12), NodeKind::SUB(Box::new(left), Box::new(right)));
        subtraction.ctype = Type::new_integer();
        let expected = Node::new_return((1, 1), subtraction);

        integration_test_semantics("return 100 - 200;", expected);
    }

    // 統合テスト用
    fn integration_test_semantics(input: &str, expected: Node) {
        let mut manager = preprocess(input);
        manager.semantics();

        assert_eq!(manager.expr, expected);
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
