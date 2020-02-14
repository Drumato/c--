use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Function, Node, NodeKind};
use crate::compiler::frontend::variable::VarKind;
use crate::compiler::ir::three_address_code;
use three_address_code::{
    basicblock::BasicBlock,
    function::IRFunction,
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
            Self::NEGATIVE(_left) => Some(Operator::MINUS),
            _ => None,
        }
    }
}

impl Manager {
    pub fn generate_three_address_code(&mut self) {
        let ast_functions = self.functions.clone();
        for (idx, ast_func) in ast_functions.iter().enumerate() {
            // 単一関数
            let entry_bb = BasicBlock::new("entry".to_string());
            self.add_new_ir_func(&ast_func, entry_bb);

            // 関数内ASTからIRを生成
            self.init_info_for_genir();
            for stmt in ast_func.stmts.iter() {
                self.gen_stmt(idx, stmt.clone());
            }
        }
    }
    fn gen_stmt(&mut self, func_idx: usize, stmt: Node) {
        match stmt.kind.clone() {
            NodeKind::GOTOSTMT(label_name) => {
                let succ_label = format!(".L{}", label_name);
                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_goto(succ_label.to_string()),
                );

                // 新しいベーシックブロックに向ける
                let goto_bb = BasicBlock::new(succ_label);
                self.ir_funcs[func_idx].blocks.push(goto_bb);
                self.cur_bb += 1;
            }
            NodeKind::COMPOUNDSTMT(stmts) => {
                for st in stmts.iter() {
                    self.gen_stmt(func_idx, st.clone());
                }
            }
            NodeKind::RETURNSTMT(child) => {
                let return_operand = self.gen_expr(func_idx, *child);
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_return(return_operand));
            }
            NodeKind::FORSTMT(cl, ex, ex2, stmt) => {
                let _ = self.gen_expr(func_idx, *cl);
                let loop_label = format!(".L{}", self.use_current_label());
                let fin_label = format!(".L{}", self.use_current_label());

                // ループラベルの生成
                let loop_bb = BasicBlock::new(loop_label.clone());
                self.ir_funcs[func_idx].blocks.push(loop_bb);
                self.cur_bb += 1;
                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_label(loop_label.to_string()),
                );

                // 条件式の翻訳
                let cond_op = self.gen_expr(func_idx, *ex);

                // ifジャンプの翻訳,body/gotoの翻訳
                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_iff(cond_op, fin_label.clone()),
                );

                self.gen_stmt(func_idx, *stmt);
                let _ = self.gen_expr(func_idx, *ex2);
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_goto(loop_label));

                // for終了後のラベル/BBを生成
                let succ_bb = BasicBlock::new(fin_label.clone());
                self.ir_funcs[func_idx].blocks.push(succ_bb);
                self.cur_bb += 1;
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_label(fin_label));
            }
            NodeKind::DOWHILESTMT(stmt, cond_expr) => {
                let loop_label = format!(".L{}", self.use_current_label());
                let fin_label = format!(".L{}", self.use_current_label());

                // ループラベルの生成
                let loop_bb = BasicBlock::new(loop_label.clone());
                self.ir_funcs[func_idx].blocks.push(loop_bb);
                self.cur_bb += 1;
                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_label(loop_label.to_string()),
                );

                // bodyの翻訳
                self.gen_stmt(func_idx, *stmt);

                // 条件式の翻訳
                let cond_op = self.gen_expr(func_idx, *cond_expr);
                // ifジャンプの翻訳,gotoの翻訳
                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_iff(cond_op, fin_label.clone()),
                );
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_goto(loop_label));

                // while終了後のラベル/BBを生成
                let succ_bb = BasicBlock::new(fin_label.clone());
                self.ir_funcs[func_idx].blocks.push(succ_bb);
                self.cur_bb += 1;
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_label(fin_label));
            }
            NodeKind::WHILESTMT(cond_expr, stmt) => {
                // 条件式の翻訳
                let cond_op = self.gen_expr(func_idx, *cond_expr);
                let loop_label = format!(".L{}", self.use_current_label());
                let fin_label = format!(".L{}", self.use_current_label());

                // ループラベルの生成
                let loop_bb = BasicBlock::new(loop_label.clone());
                self.ir_funcs[func_idx].blocks.push(loop_bb);
                self.cur_bb += 1;
                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_label(loop_label.to_string()),
                );

                // ifジャンプの翻訳,body/gotoの翻訳
                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_iff(cond_op, fin_label.clone()),
                );

                self.gen_stmt(func_idx, *stmt);
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_goto(loop_label));

                // while終了後のラベル/BBを生成
                let succ_bb = BasicBlock::new(fin_label.clone());
                self.ir_funcs[func_idx].blocks.push(succ_bb);
                self.cur_bb += 1;
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_label(fin_label));
            }
            NodeKind::IFSTMT(cond_expr, any_stmt) => {
                let cond_op = self.gen_expr(func_idx, *cond_expr);
                let lnum = self.use_current_label();
                let fin_label = format!(".L{}", lnum);

                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_iff(cond_op, fin_label.clone()),
                );

                self.gen_stmt(func_idx, *any_stmt);

                // 新しいベーシックブロックに向ける
                let succ_bb = BasicBlock::new(fin_label.clone());
                self.ir_funcs[func_idx].blocks.push(succ_bb);
                self.cur_bb += 1;

                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_label(fin_label));
            }
            NodeKind::IFELSESTMT(cond_expr, stmt, alt_stmt) => {
                let cond_op = self.gen_expr(func_idx, *cond_expr);
                let lnum = self.use_current_label();
                let fin_label = format!(".L{}", lnum);

                let lnum2 = self.use_current_label();
                let else_label = format!(".L{}", lnum2);

                self.add_ir_to_current_bb(
                    func_idx,
                    ThreeAddressCode::new_iff(cond_op, else_label.clone()),
                );

                self.gen_stmt(func_idx, *stmt);
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_goto(fin_label.clone()));

                // elseブロックに向ける
                let else_bb = BasicBlock::new(else_label.clone());
                self.ir_funcs[func_idx].blocks.push(else_bb);
                self.cur_bb += 1;

                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_label(else_label));

                self.gen_stmt(func_idx, *alt_stmt);

                // finブロックに向ける
                let succ_bb = BasicBlock::new(fin_label.clone());
                self.ir_funcs[func_idx].blocks.push(succ_bb);
                self.cur_bb += 1;

                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_label(fin_label));
            }
            NodeKind::LABELEDSTMT(label_name, any_stmt) => {
                // goto時に新しいBasicBlockを向いた状態になっている
                // IRを生成するのはCFG構築などに必要な為.
                let ir_label = format!(".L{}", label_name);
                self.add_ir_to_current_bb(func_idx, ThreeAddressCode::new_label(ir_label));
                self.gen_stmt(func_idx, *any_stmt);
            }
            NodeKind::EXPRSTMT(child) => {
                let _ = self.gen_expr(func_idx, *child);
            }
            _ => (),
        }
    }
    #[allow(irrefutable_let_patterns)]
    fn gen_expr(&mut self, func_idx: usize, n: Node) -> Operand {
        match n.kind.clone() {
            NodeKind::ASSIGN(lv, rv) => {
                // 左右の子ノードを変換
                let right_op = self.gen_expr(func_idx, *rv);
                let left_op = self.gen_expr(func_idx, *lv);

                let assign_code = ThreeAddressCode::new_assign_code(left_op, right_op.clone());

                self.add_ir_to_current_bb(func_idx, assign_code);
                right_op
            }
            // 単項演算
            NodeKind::NEGATIVE(inner) => {
                let inner_op = self.gen_expr(func_idx, *inner);

                // 次に作るべき番号を持つ仮想レジスタを作成
                let variable_reg = self.use_current_virt_reg();

                let unary_code = ThreeAddressCode::new_unop_code(
                    variable_reg.clone(),
                    n.kind.to_operator().unwrap(),
                    inner_op,
                );
                self.add_ir_to_current_bb(func_idx, unary_code);
                variable_reg
            }
            // 二項演算
            NodeKind::ADD(left, right)
            | NodeKind::SUB(left, right)
            | NodeKind::MUL(left, right)
            | NodeKind::DIV(left, right) => {
                // 左右の子ノードを変換
                let left_op = self.gen_expr(func_idx, *left);
                let right_op = self.gen_expr(func_idx, *right);

                // 次に作るべき番号を持つ仮想レジスタを作成
                let variable_reg = self.use_current_virt_reg();

                // 二項演算コード生成
                let binary_code = ThreeAddressCode::new_binop_code(
                    variable_reg.clone(),
                    n.kind.to_operator().unwrap(),
                    left_op,
                    right_op,
                );
                self.add_ir_to_current_bb(func_idx, binary_code);

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
    fn add_new_ir_func(&mut self, ast_func: &Function, bb: BasicBlock) {
        let mut ir_func = IRFunction::new(ast_func.name.to_string());
        ir_func.blocks.push(bb);
        ir_func.frame_size = ast_func.frame_size;
        self.ir_funcs.push(ir_func);
    }
    fn add_ir_to_current_bb(&mut self, func_idx: usize, ir: ThreeAddressCode) {
        self.ir_funcs[func_idx].blocks[self.cur_bb].tacs.push(ir);
    }
    fn use_current_virt_reg(&mut self) -> Operand {
        let current_reg = self.cur_virt_reg();
        self.virt += 1;
        current_reg
    }
    fn use_current_label(&mut self) -> usize {
        let current_label = self.label;
        self.label += 1;
        current_label
    }
    fn cur_virt_reg(&mut self) -> Operand {
        Operand::new_virtreg(self.virt)
    }
    fn init_info_for_genir(&mut self) {
        self.cur_bb = 0;
        self.virt = 0;
        self.label = 0;
    }
}

// 3番地コード生成に関するテスト
#[cfg(test)]
mod generate_tac_tests {}
