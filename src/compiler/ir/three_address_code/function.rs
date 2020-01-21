use crate::compiler::ir::three_address_code::basicblock::BasicBlock;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IRFunction {
    pub name: String,
    pub blocks: Vec<BasicBlock>,
    pub frame_size: usize,
}

impl IRFunction {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            blocks: Vec::new(),
            frame_size: 0,
        }
    }
}
