use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::ir::three_address_code as tac;
use tac::{Operand, Operator, ThreeAddressCode};

impl Manager {
    pub fn generate_three_address_code(&mut self) {
        let n = self.expr.clone();

        let final_reg = self.gen_expr(n);
        self.entry_block
            .tacs
            .push(ThreeAddressCode::new_return_code(final_reg));
    }
    fn gen_expr(&mut self, n: Node) -> Operand {
        match n.kind {
            NodeKind::ADD(left, right) => {
                // 左右の子ノードを変換
                let left_op = self.gen_expr(*left);
                let right_op = self.gen_expr(*right);

                // 次に作るべき番号を持つ仮想レジスタを作成
                let variable_reg = self.use_current_virt_reg();

                // 加算コード生成
                let add_code = ThreeAddressCode::new_binop_code(
                    variable_reg.clone(),
                    Operator::PLUS,
                    left_op,
                    right_op,
                );
                self.entry_block.tacs.push(add_code);

                // 式が代入されたレジスタを上位に返す
                variable_reg
            }
            NodeKind::INTEGER(val) => Operand::new_int_literal(val),
            NodeKind::INVALID => Operand::new_invalid(),
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
    #[test]
    fn test_gen_expr_with_single_integer() {
        let mut manager = preprocess("100");
        let integer_node = Node::new((1, 1), NodeKind::INTEGER(100));
        let return_operand = manager.gen_expr(integer_node);

        // 整数ノード単体では内部でコード生成されない.
        assert_eq!(0, manager.entry_block.tacs.len());

        let expected = Operand::new_int_literal(100);

        assert_eq!(expected, return_operand);
    }

    #[test]
    fn test_gen_expr_with_add_calculus() {
        let mut manager = preprocess("100 + 200");
        let left_node = Node::new((1, 1), NodeKind::INTEGER(100));
        let right_node = Node::new((7, 1), NodeKind::INTEGER(200));
        let add_node_kind = NodeKind::ADD(Box::new(left_node), Box::new(right_node));
        let add_node = Node::new((5, 1), add_node_kind);

        let return_operand = manager.gen_expr(add_node);

        // 加算ノードの場合,まず仮想レジスタに束縛するコードが生成されるはず.
        // gen_expr()の呼び出しによってコード列にそのコード片が追加されている事を期待.
        assert_eq!(1, manager.entry_block.tacs.len());
        let expected = Operand::new_virtreg(0);

        assert_eq!(expected, return_operand);
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
