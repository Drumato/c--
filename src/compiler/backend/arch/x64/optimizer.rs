use crate::compiler::ir::arch::x64::*;

#[derive(Clone)]
pub struct X64Optimizer {
    pub entry_bb: X64BasicBlock,
}
impl X64Optimizer {
    pub fn new(label: String, irs: Vec<X64IR>) -> Self {
        let entry_bb = X64BasicBlock::new(label, irs);
        Self { entry_bb: entry_bb }
    }
}
