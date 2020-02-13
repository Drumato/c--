use crate::compiler::ir::arch::x64::function::X64Function;

#[derive(Clone)]
pub struct X64Optimizer {
    pub functions: Vec<X64Function>,
}
impl X64Optimizer {
    pub fn new(functions: Vec<X64Function>) -> Self {
        Self { functions }
    }
}
