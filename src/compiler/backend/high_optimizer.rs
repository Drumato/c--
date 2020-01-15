use crate::compiler::ir::three_address_code::function::IRFunction;

// 機械独立なバックエンド操作を行う
pub struct HighOptimizer {
    pub entry_func: IRFunction,
}

impl HighOptimizer {
    pub fn new(entry_func: IRFunction) -> Self {
        Self {
            entry_func: entry_func,
        }
    }
    pub fn dump_tacs_to_stderr(&self) {
        eprintln!("{}'s blocks:", self.entry_func.name);
        for block in self.entry_func.blocks.iter() {
            block.dump_tacs_to_stderr_with_physical();
        }
    }
}
