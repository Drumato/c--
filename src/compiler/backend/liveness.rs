use crate::compiler::backend::cfg::ControlFlowGraphInBB;
use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::three_address_code::{basicblock::BasicBlock, tac_kind::TacKind};

use std::collections::BTreeSet;

type LiveInMap = Vec<BTreeSet<usize>>;
type LiveOutMap = Vec<BTreeSet<usize>>;

impl HighOptimizer {
    pub fn setup_liveness_analyze(&mut self) {
        // 生存解析用の情報収集
        self.append_liveness_informations();

        // iter_mut() では多段的に変更できないのでインデックス指定で
        let block_number = self.entry_func.blocks.len();

        for blk_idx in 0..block_number {
            let ir_number = self.entry_func.blocks[blk_idx].tacs.len();

            // 生存情報の収集
            let (live_in, live_out) =
                self.liveness_analysis(self.entry_func.blocks[blk_idx].cfg_inbb.clone(), ir_number);

            // 生存情報の反映
            for (reg_number, range) in self.entry_func.blocks[blk_idx].living.iter_mut() {
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
    pub fn append_liveness_informations(&mut self) {
        let mut blocks = self.entry_func.blocks.clone();
        let blocks_number = self.entry_func.blocks.len();
        for blk_idx in 0..blocks_number {
            self.liveness_analyze_to_bb(&mut blocks[blk_idx]);
        }
        self.entry_func.blocks = blocks;
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
mod liveness_tests {
    use super::*;
    use crate::compiler::file::SrcFile;
    use crate::compiler::frontend::{lex, manager::Manager};

    #[test]
    fn test_append_liveness_informations_with_main_func() {
        let mut optimizer = preprocess("int main(){ return 100 + 200 + 300; }");
        optimizer.append_liveness_informations();

        let expected_used: Vec<Vec<usize>> = vec![vec![], vec![0], vec![1]];
        let expected_def: Vec<Vec<usize>> = vec![vec![0], vec![1], vec![]];

        let cfg_inbb = optimizer.entry_func.blocks[0].clone().cfg_inbb;
        for (i, used_set) in cfg_inbb.used.iter().enumerate() {
            assert_eq!(used_set.len(), expected_used[i].len());
        }

        for (i, def_set) in cfg_inbb.def.iter().enumerate() {
            assert_eq!(def_set.len(), expected_def[i].len());
        }
    }

    fn preprocess(input: &str) -> HighOptimizer {
        let source_file = SrcFile {
            abs_path: "contents".to_string(),
            contents: input.to_string(),
        };
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);
        manager.parse();
        manager.semantics();
        manager.generate_three_address_code();
        let entry_func = manager.ir_func;
        let mut optimizer = HighOptimizer::new(entry_func);

        optimizer.build_cfg();
        optimizer
    }
}
