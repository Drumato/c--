use crate::compiler::ir::arch::x64::ir::X64IR;
#[derive(Clone)]
pub struct X64BasicBlock {
    pub label: String,
    pub irs: Vec<X64IR>,
}
impl X64BasicBlock {
    pub fn new(label: String, irs: Vec<X64IR>) -> Self {
        Self {
            label: label,
            irs: irs,
        }
    }
}
