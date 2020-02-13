use crate::compiler::ir::three_address_code::function::IRFunction;

// 機械独立なバックエンド操作を行う
pub struct HighOptimizer {
    pub functions: Vec<IRFunction>,
}

impl HighOptimizer {
    pub fn new(functions: Vec<IRFunction>) -> Self {
        Self { functions }
    }
    pub fn dump_tacs_to_stderr(&self) {
        for func in self.functions.iter() {
            eprintln!("{}'s blocks:", func.name);
            for block in func.blocks.iter() {
                block.dump_tacs_to_stderr_with_physical();
            }
        }
    }
}
