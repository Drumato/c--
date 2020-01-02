pub mod cfg;

use std::collections::BTreeSet;

use crate::compiler::ir::three_address_code::{BasicBlock, Operand};

pub struct Optimizer {
    // TODO: Vec<IRFunction> に変更する.
    pub entry_block: BasicBlock,
    // TODO: ControlFlowGraphに変更する.
    pub cfg_inbb: ControlFlowGraphInBB,
}

impl Optimizer {
    pub fn new(entry_bb: BasicBlock) -> Self {
        let ir_length = entry_bb.tacs.len();
        Self {
            entry_block: entry_bb,
            cfg_inbb: ControlFlowGraphInBB::new(ir_length),
        }
    }
}

#[allow(dead_code)]
pub struct ControlFlowGraphInBB {
    succ: Vec<BTreeSet<usize>>,
    prev: Vec<BTreeSet<usize>>,
    used: Vec<BTreeSet<Operand>>,
    def: Vec<BTreeSet<Operand>>,
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
