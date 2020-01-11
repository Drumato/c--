use std::collections::BTreeSet;

use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::three_address_code::{BasicBlock, TacKind};

type RegisterNumber = usize;
#[allow(dead_code)]
pub struct ControlFlowGraphInBB {
    pub succ: Vec<BTreeSet<usize>>,
    pub prev: Vec<BTreeSet<usize>>,
    pub used: Vec<BTreeSet<RegisterNumber>>,
    pub def: Vec<BTreeSet<RegisterNumber>>,
}
impl ControlFlowGraphInBB {
    pub fn new(len: usize) -> Self {
        Self {
            succ: vec![BTreeSet::new(); len],
            prev: vec![BTreeSet::new(); len],
            used: vec![BTreeSet::new(); len],
            def: vec![BTreeSet::new(); len],
        }
    }
}

// pub strut ControlFlowGraph{
//  BTreeMap<BasicBlockLabel, BTreeSet<usize>>
// }
//

impl HighOptimizer {
    pub fn build_cfg_with_bb(&mut self, bb: BasicBlock) {
        for (i, t) in bb.tacs.iter().enumerate() {
            match &t.kind {
                TacKind::EXPR(_var, _operator, _left, _right) => {
                    self.add_succ(bb.tacs.len(), i, i + 1);
                    if i != 0 {
                        self.add_prev(i, i - 1);
                    }
                }
                TacKind::RET(_return_op) => {
                    self.add_succ(bb.tacs.len(), i, i + 1);
                    if i != 0 {
                        self.add_prev(i, i - 1);
                    }
                }
            }
        }
    }
    pub fn dump_cfg_to_file(&self) {
        use std::fs::File;
        use std::io::Write;

        let mut out: String = String::new();
        out += "digraph { \n";
        for (i, t) in self.entry_block.tacs.iter().enumerate() {
            out += &(format!("\t{}[label=\"{}\",shape=\"box\"];\n", i, t.to_string()).as_str());
        }
        for idx in 0..self.entry_block.tacs.len() {
            for prev in self.cfg_inbb.prev[idx].iter() {
                out += &(format!("\t{} -> {};\n", prev, idx).as_str());
            }
        }
        out += "}";
        let file_name: String = "cfg.dot".to_string();
        let mut file = File::create(file_name).unwrap();
        file.write_all(out.as_bytes()).unwrap();
    }
    fn add_succ(&mut self, tac_length: usize, n: usize, edge: usize) {
        if edge < tac_length {
            self.cfg_inbb.succ[n].insert(edge);
        } else {
            // eprintln!("edge out of bound");
        }
    }
    fn add_prev(&mut self, n: usize, edge: usize) {
        if n != 0 {
            self.cfg_inbb.prev[n].insert(edge);
        }
    }
}

#[cfg(test)]
mod build_cfg_tests {
    use super::*;
    use crate::compiler::file::SrcFile;
    use crate::compiler::frontend::{lex, manager::Manager};
    #[test]
    fn test_build_cfg_with_bb_in_add_calculus() {
        let (mut optimizer, bb) = preprocess("100 + 200 + 300");
        optimizer.build_cfg_with_bb(bb);

        // succ_edgeのテスト
        // 最後のコードからsucc-edgeは生えてない
        let expected_succ: Vec<bool> = vec![true, true, false];
        for (i, succ_set) in optimizer.cfg_inbb.succ.iter().enumerate() {
            assert_eq!(succ_set.contains(&i), expected_succ[i]);
        }

        // prev_edgeのテスト
        // 最初のコードからprev-edgeは生えてない
        let expected_prev: Vec<bool> = vec![false, true, true];
        for (i, prev_set) in optimizer.cfg_inbb.prev.iter().enumerate() {
            assert_eq!(prev_set.contains(&i), expected_prev[i]);
        }
    }

    fn preprocess(input: &str) -> (HighOptimizer, BasicBlock) {
        let source_file = SrcFile {
            abs_path: "testcase".to_string(),
            contents: input.to_string(),
        };
        let mut manager = Manager::new(source_file);
        lex::tokenize(&mut manager);
        manager.parse();
        manager.semantics();
        let entry_bb = manager.entry_block;
        let optimizer = HighOptimizer::new(entry_bb.clone());
        (optimizer, entry_bb)
    }
}
