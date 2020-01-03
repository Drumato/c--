use crate::compiler::backend::Optimizer;
use crate::compiler::ir::three_address_code::TacKind;

impl Optimizer {
    pub fn append_liveness_informations_to_cfg(&mut self) {
        for (i, t) in self.entry_block.tacs.iter().enumerate() {
            match &t.kind {
                TacKind::EXPR(var, _operator, left, right) => {
                    // 代入されているオペランドがレジスタであれば定義集合に
                    if var.is_register() {
                        self.cfg_inbb.def[i].insert(var.virt);
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
    use crate::compiler::frontend::{lex, Manager};

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

    fn preprocess(input: &str) -> Optimizer {
        let source_file = SrcFile::new(input);
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);
        manager.parse();
        manager.semantics();
        let entry_bb = manager.entry_block;
        let mut optimizer = Optimizer::new(entry_bb.clone());

        optimizer.build_cfg_with_bb(entry_bb);
        optimizer
    }
}
