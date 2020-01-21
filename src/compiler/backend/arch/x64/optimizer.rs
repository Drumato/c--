use crate::compiler::ir::arch::x64::{basicblock::X64BasicBlock, function::X64Function};

#[derive(Clone)]
pub struct X64Optimizer {
    pub entry_func: X64Function,
}
impl X64Optimizer {
    pub fn new(func_name: String, blocks: Vec<X64BasicBlock>, frame_size: usize) -> Self {
        let entry_func = X64Function::new(func_name, blocks, frame_size);
        Self {
            entry_func: entry_func,
        }
    }
}
