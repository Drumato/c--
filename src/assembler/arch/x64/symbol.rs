use crate::assembler::arch::x64::inst;

#[derive(PartialEq, Debug, Clone)]
pub struct X64Symbol {
    pub codes: Vec<u8>,
    pub insts: Vec<inst::X64Instruction>,
    pub is_global: bool,
}

#[allow(dead_code)]
impl X64Symbol {
    pub fn new_global() -> Self {
        Self {
            codes: Vec::new(),
            insts: Vec::new(),
            is_global: true,
        }
    }
    pub fn new_local() -> Self {
        Self {
            codes: Vec::new(),
            insts: Vec::new(),
            is_global: false,
        }
    }
    pub fn is_defined(&self) -> bool {
        self.codes.len() != 0
    }
}
