use crate::compiler::error::{Error, ErrorKind, ErrorMsg};
use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::frontend::types::Type;
use crate::compiler::frontend::Manager;

impl Manager {
    pub fn semantics(&mut self) {
        let mut n = self.expr.clone();
        self.walk_expression(&mut n);
        self.expr = n;
    }
    fn walk_expression(&mut self, n: &mut Node) -> Type {
        match n.kind {
            NodeKind::INTEGER(_val) => {
                n.ctype = Type::new_integer();
                n.ctype.clone()
            }
            NodeKind::ADD(ref mut left, ref mut right) => {
                let left_type = self.walk_expression(left);
                let right_type = self.walk_expression(right);
                if left_type == right_type {
                    n.ctype = left_type;
                    return right_type;
                }
                // floatとintなど,暗黙的型変換が可能な組み合わせならそれを返す
                // self.implicit_type_inference();

                let err = Error::new(
                    ErrorKind::Type,
                    left.position,
                    ErrorMsg::MustBeSameTypeInBinaryOperation,
                );
                err.found();
                Type::new_unknown()
            }
            NodeKind::INVALID => {
                let err = Error::new(
                    ErrorKind::Type,
                    n.position,
                    ErrorMsg::InvalidNodeCantHaveType,
                );
                err.found();
                Type::new_unknown()
            }
        }
    }
}

// ASTノードへの型付けについてテスト
#[cfg(test)]
mod walk_tests {
    use super::*;
    use crate::compiler::file::SrcFile;
    use crate::compiler::frontend::lex;
    #[test]
    fn test_walk_with_single_integer_node() {
        let mut manager = preprocess("100");

        // 意味解析前
        assert_eq!(manager.expr.ctype, Type::new_unknown());
        manager.semantics();

        // 意味解析後
        assert_eq!(manager.expr.ctype, Type::new_integer());
    }

    #[test]
    fn test_walk_with_add_node() {
        let mut manager = preprocess("100 + 200");

        // 意味解析前
        assert_eq!(manager.expr.ctype, Type::new_unknown());
        manager.semantics();

        // 意味解析後
        assert_eq!(manager.expr.ctype, Type::new_integer());

        // 子ノードのチェック
        if let NodeKind::ADD(left, right) = manager.expr.kind {
            assert_eq!(left.ctype, Type::new_integer());
            assert_eq!(right.ctype, Type::new_integer());
        }
    }

    fn preprocess(input: &str) -> Manager {
        let source_file = SrcFile::new(input);
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);
        manager.parse();
        manager
    }
}
