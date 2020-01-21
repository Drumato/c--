use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Node, NodeKind};
use crate::compiler::frontend::variable::VarKind;
use crate::compiler::ir::three_address_code;
use three_address_code::{
    basicblock::BasicBlock,
    tac::ThreeAddressCode,
    tac_kind::{Operand, Operator},
};

impl NodeKind {
    fn to_operator(&self) -> Option<Operator> {
        match self {
            Self::ADD(_left, _right) => Some(Operator::PLUS),
            Self::SUB(_left, _right) => Some(Operator::MINUS),
            Self::MUL(_left, _right) => Some(Operator::ASTERISK),
            Self::DIV(_left, _right) => Some(Operator::SLASH),
            _ => None,
        }
    }
}

impl Manager {
    pub fn generate_three_address_code(&mut self) {
        // 単一関数
        let entry_bb = BasicBlock::new("entry".to_string());
        self.ir_func.blocks.push(entry_bb);
        self.ir_func.frame_size = self.entry_func.frame_size;

        // 全文を生成
        let statements = self.entry_func.stmts.clone();
        for stmt in statements.iter() {
            self.gen_stmt(stmt.clone());
        }
    }
    fn gen_stmt(&mut self, stmt: Node) {
        match stmt.kind.clone() {
            NodeKind::GOTOSTMT(label_name) => {
                let ir_label = format!(".L{}", label_name);
                self.add_ir_to_current_bb(ThreeAddressCode::new_goto(ir_label.to_string()));

                // 新しいベーシックブロックに向ける
                let goto_bb = BasicBlock::new(ir_label);
                self.ir_func.blocks.push(goto_bb);
                self.cur_bb += 1;
            }
            NodeKind::RETURNSTMT(child) => {
                let return_operand = self.gen_expr(*child);
                self.add_ir_to_current_bb(ThreeAddressCode::new_return(return_operand));
            }
            NodeKind::LABELEDSTMT(label_name, any_stmt) => {
                // goto時に新しいBasicBlockを向いた状態になっている
                // IRを生成するのはCFG構築などに必要な為.
                let ir_label = format!(".L{}", label_name);
                self.add_ir_to_current_bb(ThreeAddressCode::new_label(ir_label));
                self.gen_stmt(*any_stmt);
            }
            NodeKind::EXPRSTMT(child) => {
                let _ = self.gen_expr(*child);
            }
            _ => (),
        }
    }
    #[allow(irrefutable_let_patterns)]
    fn gen_expr(&mut self, n: Node) -> Operand {
        match n.kind.clone() {
            NodeKind::ASSIGN(lv, rv) => {
                let right_op = self.gen_expr(*rv);
                let left_op = self.gen_expr(*lv);

                let assign_code = ThreeAddressCode::new_assign_code(left_op, right_op.clone());

                self.add_ir_to_current_bb(assign_code);
                right_op
            }
            NodeKind::ADD(left, right)
            | NodeKind::SUB(left, right)
            | NodeKind::MUL(left, right)
            | NodeKind::DIV(left, right) => {
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
                self.add_ir_to_current_bb(add_code);

                // 式が代入されたレジスタを上位に返す
                variable_reg
            }
            NodeKind::INTEGER(val) => Operand::new_int_literal(val),
            NodeKind::IDENTIFIER(name) => {
                if let Some(var) = self.var_map.get(&name) {
                    if let VarKind::LOCAL(offset) = var.kind {
                        return Operand::new_auto_var(name.to_string(), offset);
                    }
                }
                eprintln!("not found such an var -> {}", name);
                Operand::new_invalid()
            }

            // NodeKind::INVALID => Operand::new_invalid(),
            _ => Operand::new_invalid(),
        }
    }
    fn add_ir_to_current_bb(&mut self, ir: ThreeAddressCode) {
        self.ir_func.blocks[self.cur_bb].tacs.push(ir);
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
    use crate::compiler::ir::three_address_code::function::IRFunction;

    #[test]
    fn test_gen_expr_with_return_stmt() {
        let mut entry_bb = BasicBlock::new("entry".to_string());
        entry_bb.tacs = vec![
            ThreeAddressCode::new_binop_code(
                Operand::new_virtreg(0),
                Operator::MINUS,
                Operand::new_int_literal(100),
                Operand::new_int_literal(200),
            ),
            ThreeAddressCode::new_return(Operand::new_virtreg(0)),
        ];
        let expected = IRFunction {
            name: "main".to_string(),
            blocks: vec![entry_bb],
            frame_size: 0,
        };

        integration_test_genir("int main(){ return 100 - 200; }", expected);
    }

    // 統合テスト用
    fn integration_test_genir(input: &str, expected: IRFunction) {
        let mut manager = preprocess(input);
        manager.generate_three_address_code();

        assert_eq!(manager.ir_func, expected);
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
