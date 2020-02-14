use crate::compiler::backend::cfg::ControlFlowGraphInBB;
use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::three_address_code::{
    basicblock::BasicBlock, function::IRFunction, tac_kind::TacKind,
};

use std::collections::BTreeSet;

type LiveInMap = Vec<BTreeSet<usize>>;
type LiveOutMap = Vec<BTreeSet<usize>>;

impl HighOptimizer {
    pub fn setup_liveness_analyze(&mut self) {
        // 生存解析用の情報収集
        self.append_liveness_informations();

        let mut functions = self.functions.clone();

        // iter_mut() では多段的に変更できないのでインデックス指定で
        let function_number = functions.len();
        for func_idx in 0..function_number {
            let block_number = functions[func_idx].blocks.len();

            for blk_idx in 0..block_number {
                let ir_number = functions[func_idx].blocks[blk_idx].tacs.len();

                // 生存情報の収集
                let (live_in, live_out) = self.liveness_analysis(
                    functions[func_idx].blocks[blk_idx].cfg_inbb.clone(),
                    ir_number,
                );

                // 生存情報の反映
                for (reg_number, range) in functions[func_idx].blocks[blk_idx].living.iter_mut() {
                    for ir_idx in 0..ir_number {
                        if !live_in[ir_idx].contains(reg_number)
                            && live_out[ir_idx].contains(reg_number)
                        {
                            range.0 = ir_idx;
                        }
                        if live_in[ir_idx].contains(reg_number)
                            && !live_out[ir_idx].contains(reg_number)
                        {
                            range.1 = ir_idx;
                        }
                    }
                }
            }
        }

        self.functions = functions;
    }
    pub fn append_liveness_informations(&mut self) {
        let mut functions = self.functions.clone();
        let functions_number = functions.len();
        for func_idx in 0..functions_number {
            self.liveness_analyze_to_func(&mut functions[func_idx]);
        }
        self.functions = functions;
    }
    fn liveness_analyze_to_func(&mut self, func: &mut IRFunction) {
        let mut blocks = func.blocks.clone();
        let blocks_number = func.blocks.len();
        for blk_idx in 0..blocks_number {
            self.liveness_analyze_to_bb(&mut blocks[blk_idx]);
        }
        func.blocks = blocks;
    }
    fn liveness_analyze_to_bb(&mut self, bb: &mut BasicBlock) {
        for (i, t) in bb.tacs.iter().enumerate() {
            match &t.kind {
                TacKind::EXPR(var, _operator, left, right) => {
                    // 代入されているオペランドがレジスタであれば定義集合に
                    if var.is_register() {
                        bb.cfg_inbb.def[i].insert(var.virt);
                        bb.living.insert(var.virt, (0, 0));
                    }

                    // 左右オペランドがレジスタであれば使用集合に
                    if left.is_register() {
                        bb.cfg_inbb.used[i].insert(left.virt);
                    }
                    if right.is_register() {
                        bb.cfg_inbb.used[i].insert(right.virt);
                    }
                }
                TacKind::ASSIGN(_dst_op, src_op) => {
                    // 使用オペランドがレジスタなら使用集合に
                    if src_op.is_register() {
                        bb.cfg_inbb.used[i].insert(src_op.virt);
                    }
                }
                TacKind::RET(return_op) => {
                    // 返すオペランドがレジスタなら使用集合に
                    if return_op.is_register() {
                        bb.cfg_inbb.used[i].insert(return_op.virt);
                    }
                }
                TacKind::IFF(op, _label) => {
                    // 返すオペランドがレジスタなら使用集合に
                    if op.is_register() {
                        bb.cfg_inbb.used[i].insert(op.virt);
                    }
                }
                _ => (),
            }
        }
    }
    pub fn liveness_analysis(
        &mut self,
        cfg_inbb: ControlFlowGraphInBB,
        tac_length: usize,
    ) -> (LiveInMap, LiveOutMap) {
        // in集合, out集合の定義
        // foreach n; in[n] <- {}; out[n] <- {};
        let mut live_in: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); tac_length];
        let mut live_out: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); tac_length];

        // repeat
        'outer: loop {
            // 変更をチェックするための集合
            let mut in_dash: Vec<BTreeSet<usize>> = Vec::new();
            let mut out_dash: Vec<BTreeSet<usize>> = Vec::new();

            // foreach n
            for idx in 0..tac_length {
                // 後で変更が無いかチェックする為に保存
                in_dash.push(live_in[idx].clone());
                out_dash.push(live_out[idx].clone());

                // out[n] <- U in[s] (where s ∈ succ[n])
                for s in cfg_inbb.succ[idx].iter() {
                    live_out[idx] = &live_out[idx] | &live_in[*s];
                }

                live_in[idx] = &cfg_inbb.used[idx] | &(&live_out[idx] - &cfg_inbb.def[idx]);
            }

            // until in'[n] == in[n] and out'[n] == out[n] for all n
            let mut chg_flg: bool = true;
            for idx in 0..live_in.len() {
                if live_in[idx] != in_dash[idx] {
                    chg_flg = false;
                }
                if live_out[idx] != out_dash[idx] {
                    chg_flg = false;
                }
            }
            if chg_flg {
                break 'outer;
            }
        }

        (live_in, live_out)
    }
}

#[cfg(test)]
mod liveness_tests {}
