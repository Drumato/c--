use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::arch::x64::{
    basicblock::X64BasicBlock,
    ir::X64IR,
    ir_kind::{X64IRKind, X64OpeKind, X64Operand},
};
use crate::compiler::ir::three_address_code as tac;
use tac::tac_kind;

impl HighOptimizer {
    // ここでは抽象的なIRにしておく.
    pub fn translate_tacs_to_x64(high_opt: Self) -> X64Optimizer {
        let mut x64_blocks: Vec<X64BasicBlock> = Vec::new();

        // BasicBlock列のイテレーション
        for bb in high_opt.entry_func.blocks.iter() {
            let mut low_irs: Vec<X64IR> = Vec::new();

            // TAC列のイテレーション
            for t in bb.tacs.iter() {
                match t.kind.clone() {
                    tac_kind::TacKind::LABEL(_label_name) => {
                        // この時点でBasicBlockに分けられているのでラベルは生成する必要はない.
                        // (3番地コードのときはCFG構築などにラベル情報があると便利だったため利用)
                    }
                    tac_kind::TacKind::GOTO(label_name) => {
                        low_irs.push(X64IR::new_jump(label_name));
                    }
                    tac_kind::TacKind::ASSIGN(lv_bf, rv_bf) => {
                        let src_op = Self::tac_operand_to_x64(rv_bf);
                        let dst_op = Self::tac_operand_to_x64(lv_bf);
                        low_irs.push(X64IR::new_store(dst_op, src_op));
                    }
                    tac_kind::TacKind::RET(return_bf) => {
                        let return_op = Self::tac_operand_to_x64(return_bf);
                        // new_ret -> 最終的に mov rax, <return_op> ; ret を生成
                        low_irs.push(X64IR::new_ret(return_op));
                    }
                    tac_kind::TacKind::UNARYEXPR(var_bf, operator_bf, inner_bf) => {
                        // 各構成要素を変換
                        let inner = Self::tac_operand_to_x64(inner_bf);
                        let opcode: X64IRKind = Self::unary_opcode_from_operator(operator_bf);
                        let dst = Self::tac_operand_to_x64(var_bf);

                        // movしてから演算
                        let load_ir = X64IR::new_mov(dst.clone(), inner);
                        low_irs.push(load_ir);

                        Self::add_unary_ir_matching_opcode(&mut low_irs, opcode, dst);
                    }
                    tac_kind::TacKind::EXPR(var_bf, operator_bf, left_bf, right_bf) => {
                        // 各構成要素を変換
                        let left = Self::tac_operand_to_x64(left_bf);
                        let right = Self::tac_operand_to_x64(right_bf);
                        let opcode: X64IRKind = Self::binary_opcode_from_operator(operator_bf);
                        let dst = Self::tac_operand_to_x64(var_bf);

                        // 左が数値リテラル -> (右がレジスタであれば) raxにロードしてから演算
                        //                  -> (そうでなければ)先にdstにロードしてから,演算
                        // それ以外         -> 演算してからdstにロード
                        if let X64OpeKind::INTLIT(_) = &left.kind {
                            if let X64OpeKind::REG = &right.kind {
                                // r.g. t1 <- 2 + t1
                                // -----------------
                                // rax <- 2
                                // rax <- rax + t1
                                // t1 <- rax
                                let load_ir = X64IR::new_mov(X64Operand::new_rax(), left.clone());
                                low_irs.push(load_ir);

                                // 演算命令
                                Self::add_binary_ir_matching_opcode(
                                    &mut low_irs,
                                    opcode,
                                    X64Operand::new_rax(),
                                    right,
                                );

                                let load_ir = X64IR::new_mov(dst, X64Operand::new_rax());
                                low_irs.push(load_ir);
                            } else {
                                let load_ir = X64IR::new_mov(dst.clone(), left.clone());
                                low_irs.push(load_ir);

                                // 演算命令
                                Self::add_binary_ir_matching_opcode(
                                    &mut low_irs,
                                    opcode,
                                    dst,
                                    right,
                                );
                            }
                            continue;
                        }

                        // 左が非数値リテラル
                        Self::add_binary_ir_matching_opcode(
                            &mut low_irs,
                            opcode,
                            left.clone(),
                            right,
                        );

                        let load_ir = X64IR::new_mov(dst, left);
                        low_irs.push(load_ir);
                    }
                }
            }

            let x64_bb = X64BasicBlock::new(bb.label.to_string(), low_irs);
            x64_blocks.push(x64_bb);
        }

        X64Optimizer::new(
            high_opt.entry_func.name,
            x64_blocks,
            high_opt.entry_func.frame_size,
        )
    }

    fn add_unary_ir_matching_opcode(
        low_irs: &mut Vec<X64IR>,
        opcode: X64IRKind,
        inner: X64Operand,
    ) {
        match opcode {
            X64IRKind::NEGATIVE(_) => {
                low_irs.push(X64IR::new_neg(inner));
            }
            _ => {}
        }
    }
    fn add_binary_ir_matching_opcode(
        low_irs: &mut Vec<X64IR>,
        opcode: X64IRKind,
        left: X64Operand,
        right: X64Operand,
    ) {
        match opcode {
            X64IRKind::ADD(_, _) => {
                low_irs.push(X64IR::new_add(left, right));
            }
            X64IRKind::SUB(_, _) => {
                low_irs.push(X64IR::new_sub(left, right));
            }
            X64IRKind::MUL(_, _) => {
                low_irs.push(X64IR::new_mul(left, right));
            }
            X64IRKind::DIV(_, _) => {
                low_irs.push(X64IR::new_div(left, right));
            }
            _ => {}
        }
    }
    fn unary_opcode_from_operator(operator: tac_kind::Operator) -> X64IRKind {
        // 返すIRKindの中身は全てINVALID
        match operator {
            tac_kind::Operator::MINUS => X64IRKind::NEGATIVE(X64Operand::new_inv()),
            _ => panic!("can't traslate opcode from operator"),
        }
    }
    fn binary_opcode_from_operator(operator: tac_kind::Operator) -> X64IRKind {
        // 返すIRKindの中身は全てINVALID
        match operator {
            tac_kind::Operator::PLUS => {
                X64IRKind::ADD(X64Operand::new_inv(), X64Operand::new_inv())
            }
            tac_kind::Operator::MINUS => {
                X64IRKind::SUB(X64Operand::new_inv(), X64Operand::new_inv())
            }
            tac_kind::Operator::ASTERISK => {
                X64IRKind::MUL(X64Operand::new_inv(), X64Operand::new_inv())
            }
            tac_kind::Operator::SLASH => {
                X64IRKind::DIV(X64Operand::new_inv(), X64Operand::new_inv())
            }
        }
    }
    fn tac_operand_to_x64(op: tac_kind::Operand) -> X64Operand {
        let kind = Self::tac_opekind_to_x64(op.kind);
        X64Operand::new(kind, op.virt, op.phys)
    }
    fn tac_opekind_to_x64(kind: tac_kind::OpeKind) -> X64OpeKind {
        match kind {
            tac_kind::OpeKind::INTLIT(val) => X64OpeKind::INTLIT(val),
            tac_kind::OpeKind::AUTOVARIABLE(name, offset) => {
                X64OpeKind::AUTOVAR(name.to_string(), offset)
            }
            tac_kind::OpeKind::REG => X64OpeKind::REG,
            tac_kind::OpeKind::INVALID => X64OpeKind::INVALID,
        }
    }
}
