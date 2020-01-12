use crate::compiler::backend::cfg::ControlFlowGraphInBB;
use crate::compiler::ir::three_address_code::basicblock::BasicBlock;

// 機械独立なバックエンド操作を行う
pub struct HighOptimizer {
    pub entry_block: BasicBlock,
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
    pub fn dump_tacs_to_stderr(&self) {
        self.entry_block.dump_tacs_to_stderr_with_physical();
    }
}
