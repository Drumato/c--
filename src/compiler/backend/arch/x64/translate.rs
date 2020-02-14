use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::arch::x64::{
    basicblock::X64BasicBlock,
    function::X64Function,
    ir::X64IR,
    ir_kind::{X64IRKind, X64OpeKind, X64Operand},
};
use crate::compiler::ir::three_address_code as tac;
use tac::tac_kind;
use tac::{basicblock::BasicBlock, function::IRFunction};

impl HighOptimizer {
    // ここでは抽象的なIRにしておく.
    pub fn translate_tacs_to_x64(high_opt: Self) -> X64Optimizer {
        let mut x64_funcs: Vec<X64Function> = Vec::new();

        for meta_func in high_opt.functions.iter() {
            let x64_func = Self::translate_meta_func_to_x64(meta_func.clone());
            x64_funcs.push(x64_func);
        }

        X64Optimizer::new(x64_funcs)
    }
    fn translate_meta_func_to_x64(meta_func: IRFunction) -> X64Function {
        let mut x64_blocks: Vec<X64BasicBlock> = Vec::new();

        for meta_bb in meta_func.blocks.iter() {
            let x64_block = Self::translate_meta_bb_to_x64(meta_bb);
            x64_blocks.push(x64_block);
        }

        X64Function::new(meta_func.name.to_string(), x64_blocks, meta_func.frame_size)
    }
    fn translate_meta_bb_to_x64(meta_bb: &BasicBlock) -> X64BasicBlock {
        let mut low_irs: Vec<X64IR> = Vec::new();
        // TAC列のイテレーション
        for t in meta_bb.tacs.iter() {
            match t.kind.clone() {
                tac_kind::TacKind::LABEL(_label_name) => {
                    // この時点でBasicBlockに分けられているのでラベルは生成する必要はない.
                    // (3番地コードのときはCFG構築などにラベル情報があると便利だったため利用)
                }
                tac_kind::TacKind::IFF(op, label_name) => {
                    let cmp_op = Self::tac_operand_to_x64(op);
                    low_irs.push(X64IR::new_cmpzero(cmp_op));
                    low_irs.push(X64IR::new_jumpzero(label_name));
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
                    match &left.kind {
                        // 左がレジスタ
                        X64OpeKind::REG => {
                            Self::add_binary_ir_matching_opcode(
                                &mut low_irs,
                                opcode,
                                left.clone(),
                                right,
                            );

                            let load_ir = X64IR::new_mov(dst, left);
                            low_irs.push(load_ir);
                        }
                        X64OpeKind::AUTOVAR(_name, _offset) => {
                            let load_ir = X64IR::new_mov(dst, left.clone());
                            low_irs.push(load_ir);
                            Self::add_binary_ir_matching_opcode(&mut low_irs, opcode, left, right);
                        }
                        // 左が整数値
                        X64OpeKind::INTLIT(_) => {
                            match &right.kind {
                                X64OpeKind::REG => {
                                    // r.g. t1 <- 2 + t1
                                    // -----------------
                                    // rax <- 2
                                    // rax <- rax + t1
                                    // t1 <- rax
                                    let load_ir =
                                        X64IR::new_mov(X64Operand::new_rax(), left.clone());
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
                                }
                                _ => {
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
                            }
                        }
                        X64OpeKind::INVALID => panic!("got invalid operand"),
                    }
                }
            }
        }
        X64BasicBlock::new(meta_bb.label.to_string(), low_irs)
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
