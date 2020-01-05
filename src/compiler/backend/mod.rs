pub mod cfg;
pub mod liveness;

use std::collections::BTreeSet;

use crate::compiler::ir::three_address_code::BasicBlock;

// 機械独立なバックエンド操作を行う
pub struct HighOptimizer {
    // TODO: Vec<IRFunction> に変更する.
    pub entry_block: BasicBlock,
    // TODO: ControlFlowGraphに変更する.
    pub cfg_inbb: ControlFlowGraphInBB,
}

impl HighOptimizer {
    pub fn new(entry_bb: BasicBlock) -> Self {
        let ir_length = entry_bb.tacs.len();
        Self {
            entry_block: entry_bb,
            cfg_inbb: ControlFlowGraphInBB::new(ir_length),
        }
    }
}

type RegisterNumber = usize;
#[allow(dead_code)]
pub struct ControlFlowGraphInBB {
    succ: Vec<BTreeSet<usize>>,
    prev: Vec<BTreeSet<usize>>,
    used: Vec<BTreeSet<RegisterNumber>>,
    def: Vec<BTreeSet<RegisterNumber>>,
}
impl ControlFlowGraphInBB {
    fn new(len: usize) -> Self {
        Self {
            succ: vec![BTreeSet::new(); len],
            prev: vec![BTreeSet::new(); len],
            used: vec![BTreeSet::new(); len],
            def: vec![BTreeSet::new(); len],
        }
    }
}

// TODO: ベーシックブロック間のCFG
// pub strut ControlFlowGraph{
//  BTreeMap<BasicBlockLabel, BTreeSet<usize>>
// }
