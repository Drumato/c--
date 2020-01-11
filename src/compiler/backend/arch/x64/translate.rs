use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::arch::x64::*;
use crate::compiler::ir::three_address_code as tac;
use tac::TacKind;

impl HighOptimizer {
    // ここでは抽象的なIRにしておく.
    pub fn translate_tacs_to_x64(high_opt: Self) -> X64Optimizer {
        let mut low_irs: Vec<X64IR> = Vec::new();

        // TAC列のイテレーション
        for t in high_opt.entry_block.tacs.iter() {
            match t.kind.clone() {
                TacKind::EXPR(var_bf, operator_bf, left_bf, right_bf) => {
                    // 各構成要素を変換
                    let left = Self::tac_operand_to_x64(left_bf);
                    let right = Self::tac_operand_to_x64(right_bf);
                    let opcode: X64IRKind = Self::opcode_from_operator(operator_bf);
                    let dst = Self::tac_operand_to_x64(var_bf);

                    // 左が数値リテラル -> 先にdstにロードしてから,演算
                    // それ以外         -> 演算してからdstにロード
                    if let X64OpeKind::INTLIT(_) = &left.kind {
                        let load_ir = X64IR::new_mov(dst.clone(), left.clone());
                        low_irs.push(load_ir);

                        // 演算命令
                        Self::add_ir_matching_opcode(&mut low_irs, opcode, dst, right);
                    } else {
                        // 演算命令
                        Self::add_ir_matching_opcode(&mut low_irs, opcode, left.clone(), right);

                        let load_ir = X64IR::new_mov(dst, left);
                        low_irs.push(load_ir);
                    }
                }
                TacKind::RET(return_bf) => {
                    let return_op = Self::tac_operand_to_x64(return_bf);
                    // new_ret -> 最終的に mov rax, <return_op> ; ret を生成
                    low_irs.push(X64IR::new_ret(return_op));
                }
            }
        }

        X64Optimizer::new(high_opt.entry_block.label, low_irs)
    }

    fn add_ir_matching_opcode(
        low_irs: &mut Vec<X64IR>,
        opcode: X64IRKind,
        left: X64Operand,
        right: X64Operand,
    ) {
        match opcode {
            X64IRKind::ADD(_, _) => {
                low_irs.push(X64IR::new_add(left, right));
            }
            _ => {}
        }
    }
    fn opcode_from_operator(operator: tac::Operator) -> X64IRKind {
        // 返すIRKindの中身は全てINVALID
        match operator {
            tac::Operator::PLUS => X64IRKind::ADD(X64Operand::new_inv(), X64Operand::new_inv()),
        }
    }
    fn tac_operand_to_x64(op: tac::Operand) -> X64Operand {
        let kind = Self::tac_opekind_to_x64(op.kind);
        X64Operand::new(kind, op.virt, op.phys)
    }
    fn tac_opekind_to_x64(kind: tac::OpeKind) -> X64OpeKind {
        match kind {
            tac::OpeKind::INTLIT(val) => X64OpeKind::INTLIT(val),
            tac::OpeKind::REG => X64OpeKind::REG,
            tac::OpeKind::INVALID => X64OpeKind::INVALID,
        }
    }
}
