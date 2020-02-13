use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::frontend::types::Type;
use crate::error::{Error, ErrorKind, ErrorMsg};

impl Manager {
    pub fn semantics(&mut self) {
        let mut functions = self.functions.clone();
        let functions_number = functions.len();
        for func_idx in 0..functions_number {
            let mut statements = functions[func_idx].stmts.clone();
            let statements_number = functions[func_idx].stmts.len();
            for stmt_idx in 0..statements_number {
                self.walk_statement(&mut statements[stmt_idx]);
            }
            functions[func_idx].stmts = statements;
        }
        self.functions = functions;
    }
    fn walk_statement(&mut self, stmt: &mut Node) {
        match stmt.kind {
            NodeKind::RETURNSTMT(ref mut return_expr) => {
                self.walk_expression(return_expr);
            }
            NodeKind::LABELEDSTMT(ref mut _label_name, ref mut any_stmt) => {
                self.walk_statement(any_stmt);
            }
            NodeKind::COMPOUNDSTMT(ref mut stmts) => {
                for st in stmts.iter_mut() {
                    self.walk_statement(st);
                }
            }
            NodeKind::EXPRSTMT(ref mut expr) => {
                self.walk_expression(expr);
            }
            NodeKind::IFSTMT(ref mut expr, ref mut stmt) => {
                self.walk_expression(expr);
                self.walk_statement(stmt);
            }
            NodeKind::WHILESTMT(ref mut expr, ref mut stmt) => {
                self.walk_expression(expr);
                self.walk_statement(stmt);
            }
            NodeKind::DOWHILESTMT(ref mut stmt, ref mut expr) => {
                self.walk_statement(stmt);
                self.walk_expression(expr);
            }
            NodeKind::FORSTMT(ref mut cl, ref mut ex, ref mut ex2, ref mut stmt) => {
                self.walk_expression(cl);
                self.walk_expression(ex);
                self.walk_expression(ex2);
                self.walk_statement(stmt);
            }
            NodeKind::IFELSESTMT(ref mut expr, ref mut stmt, ref mut alt) => {
                self.walk_expression(expr);
                self.walk_statement(stmt);
                self.walk_statement(alt);
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
mod walk_tests {}
