use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::ir::three_address_code;
use three_address_code::{
    tac::ThreeAddressCode,
    tac_kind::{Operand, Operator},
};

impl NodeKind {
    fn to_operator(&self) -> Option<Operator> {
        match self {
            Self::ADD(_left, _right) => Some(Operator::PLUS),
            Self::SUB(_left, _right) => Some(Operator::MINUS),
            _ => None,
        }
    }
}

impl Manager {
    pub fn generate_three_address_code(&mut self) {
        // TODO: 今はexprを文のように扱っている
        let n = self.expr.clone();

        self.gen_stmt(n);
    }
    fn gen_stmt(&mut self, stmt: Node) {
        match stmt.kind.clone() {
            NodeKind::RETURNSTMT(child) => {
                let return_operand = self.gen_expr(*child);
                self.entry_block
                    .tacs
                    .push(ThreeAddressCode::new_return_code(return_operand));
            }
            _ => (),
        }
    }
    fn gen_expr(&mut self, n: Node) -> Operand {
        match n.kind.clone() {
            NodeKind::ADD(left, right) | NodeKind::SUB(left, right) => {
                // 左右の子ノードを変換
                let left_op = self.gen_expr(*left);
                let right_op = self.gen_expr(*right);

                // 次に作るべき番号を持つ仮想レジスタを作成
                let variable_reg = self.use_current_virt_reg();

                // 加算コード生成
                let add_code = ThreeAddressCode::new_binop_code(
                    variable_reg.clone(),
                    n.kind.to_operator().unwrap(),
                    left_op,
                    right_op,
                );
                self.entry_block.tacs.push(add_code);

                // 式が代入されたレジスタを上位に返す
                variable_reg
            }
            NodeKind::INTEGER(val) => Operand::new_int_literal(val),

            // NodeKind::INVALID => Operand::new_invalid(),
            _ => Operand::new_invalid(),
        }
    }
    fn use_current_virt_reg(&mut self) -> Operand {
        let current_reg = self.cur_virt_reg();
        self.virt += 1;
        current_reg
    }
    fn cur_virt_reg(&mut self) -> Operand {
        Operand::new_virtreg(self.virt)
    }
}

// 3番地コード生成に関するテスト
#[cfg(test)]
mod generate_tac_tests {
    use super::*;
    use crate::compiler::file::SrcFile;
    use crate::compiler::frontend::lex;
    use crate::compiler::ir::three_address_code::basicblock::BasicBlock;

    #[test]
    fn test_gen_expr_with_return_stmt() {
        let mut expected = BasicBlock::new("main".to_string());
        expected.tacs = vec![
            ThreeAddressCode::new_binop_code(
                Operand::new_virtreg(0),
                Operator::MINUS,
                Operand::new_int_literal(100),
                Operand::new_int_literal(200),
            ),
            ThreeAddressCode::new_return_code(Operand::new_virtreg(0)),
        ];

        integration_test_genir("return 100 - 200;", expected);
    }

    // 統合テスト用
    // TODO: IRFunctionのチェックに変える必要あり
    fn integration_test_genir(input: &str, expected: BasicBlock) {
        let mut manager = preprocess(input);
        manager.generate_three_address_code();

        assert_eq!(manager.entry_block, expected);
    }

    fn preprocess(input: &str) -> Manager {
        let source_file = SrcFile {
            abs_path: "testcase".to_string(),
            contents: input.to_string(),
        };
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);
        manager.parse();
        manager.semantics();
        manager
    }
}
