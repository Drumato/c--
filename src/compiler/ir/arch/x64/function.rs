use crate::compiler::ir::arch::x64::basicblock::X64BasicBlock;
#[derive(Clone)]
pub struct X64Function {
    pub func_name: String,
    pub blocks: Vec<X64BasicBlock>,
    pub frame_size: usize,
}
impl X64Function {
    pub fn new(func_name: String, blocks: Vec<X64BasicBlock>, frame_size: usize) -> Self {
        Self {
            func_name: func_name,
            blocks: blocks,
            frame_size: frame_size,
        }
    }
}
