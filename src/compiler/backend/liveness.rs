use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::three_address_code::TacKind;

use std::collections::BTreeSet;

impl HighOptimizer {
    pub fn append_liveness_informations_to_cfg(&mut self) {
        for (i, t) in self.entry_block.tacs.iter().enumerate() {
            match &t.kind {
                TacKind::EXPR(var, _operator, left, right) => {
                    // 代入されているオペランドがレジスタであれば定義集合に
                    if var.is_register() {
                        self.cfg_inbb.def[i].insert(var.virt);
                        self.entry_block.living.insert(var.virt, (0, 0));
                    }

                    // 左右オペランドがレジスタであれば使用集合に
                    if left.is_register() {
                        self.cfg_inbb.used[i].insert(left.virt);
                    }
                    if right.is_register() {
                        self.cfg_inbb.used[i].insert(right.virt);
                    }
                }
                TacKind::RET(return_op) => {
                    // 返すオペランドがレジスタなら使用集合に
                    if return_op.is_register() {
                        self.cfg_inbb.used[i].insert(return_op.virt);
                    }
                }
            }
        }
    }
    // TODO: ベーシックブロックを受け取って変更をそのBBに適用する
    pub fn liveness_analysis(&mut self) {
        // in集合, out集合の定義
        // foreach n; in[n] <- {}; out[n] <- {};
        let mut live_in: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); self.entry_block.tacs.len()];
        let mut live_out: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); self.entry_block.tacs.len()];

        // repeat
        'outer: loop {
            // 変更をチェックするための集合
            let mut in_dash: Vec<BTreeSet<usize>> = Vec::new();
            let mut out_dash: Vec<BTreeSet<usize>> = Vec::new();

            // foreach n
            for (idx, _t) in self.entry_block.tacs.iter().rev().enumerate() {
                // 後で変更が無いかチェックする為に保存
                in_dash.push(live_in[idx].clone());
                out_dash.push(live_out[idx].clone());

                // out[n] <- U in[s] (where s ∈ succ[n])
                for s in self.cfg_inbb.succ[idx].iter() {
                    live_out[idx] = &live_out[idx] | &live_in[*s];
                }

                live_in[idx] =
                    &self.cfg_inbb.used[idx] | &(&live_out[idx] - &self.cfg_inbb.def[idx]);
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

        // 生存情報の反映
        for (reg_number, range) in self.entry_block.living.iter_mut() {
            for (idx, _t) in self.entry_block.tacs.iter().enumerate() {
                if !live_in[idx].contains(reg_number) && live_out[idx].contains(reg_number) {
                    range.0 = idx;
                }
                if live_in[idx].contains(reg_number) && !live_out[idx].contains(reg_number) {
                    range.1 = idx;
                }
            }
        }
    }
    pub fn dump_cfg_liveness_to_file(&self) {
        use std::fs::File;
        use std::io::Write;

        let mut out: String = String::new();
        out += "digraph { \n";
        for (i, t) in self.entry_block.tacs.iter().enumerate() {
            out += &(format!("\t{}[label=\"{}\",shape=\"box\"];\n", i, t.to_string()).as_str());
        }
        for idx in 0..self.entry_block.tacs.len() {
            for prev in self.cfg_inbb.prev[idx].iter() {
                // 定義集合を文字列にまとめる
                let mut def_string = String::new();
                for def in self.cfg_inbb.def[*prev].iter() {
                    def_string += &(format!("t{}", def).as_str());
                }

                // 使用集合を文字列にまとめる
                let mut used_string = String::new();
                for used in self.cfg_inbb.used[*prev].iter() {
                    used_string += &(format!("t{}", used).as_str());
                }
                out += &(format!(
                    "\t{} -> {}[label=\"def: {}, used: {}\"];\n",
                    prev, idx, def_string, used_string,
                )
                .as_str());
            }
        }
        out += "}";
        let file_name: String = "cfg.dot".to_string();
        let mut file = File::create(file_name).unwrap();
        file.write_all(out.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod liveness_tests {
    use super::*;
    use crate::compiler::file::SrcFile;
    use crate::compiler::frontend::{lex, manager::Manager};

    #[test]
    fn test_append_liveness_informations_to_cfg_with_add_calculus() {
        let mut optimizer = preprocess("100 + 200 + 300");
        optimizer.append_liveness_informations_to_cfg();

        let expected_used: Vec<Vec<usize>> = vec![vec![], vec![0], vec![1]];
        let expected_def: Vec<Vec<usize>> = vec![vec![0], vec![1], vec![]];

        for (i, used_set) in optimizer.cfg_inbb.used.iter().enumerate() {
            assert_eq!(used_set.len(), expected_used[i].len());
        }

        for (i, def_set) in optimizer.cfg_inbb.def.iter().enumerate() {
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
        let entry_bb = manager.entry_block;
        let mut optimizer = HighOptimizer::new(entry_bb.clone());

        optimizer.build_cfg_with_bb(entry_bb);
        optimizer
    }
}
