use std::collections::BTreeMap;

use crate::compiler::ir::three_address_code::tac::ThreeAddressCode;

type RegisterNumber = usize;
type LiveIn = usize;
type LiveOut = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub tacs: Vec<ThreeAddressCode>,

    pub living: BTreeMap<RegisterNumber, (LiveIn, LiveOut)>,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label: label,
            tacs: Vec::new(),
            living: BTreeMap::new(),
        }
    }
    pub fn dump_tacs_to_stderr(&self) {
        eprintln!("{}'s IR:", self.label);
        for t in self.tacs.iter() {
            eprintln!("\t{}", t.to_string());
        }
    }
    pub fn dump_tacs_to_stderr_with_physical(&self) {
        eprintln!("{}'s IR:", self.label);
        for t in self.tacs.iter() {
            eprintln!("\t{}", t.to_string_physical());
        }
    }
    pub fn dump_liveness(&self) {
        for (reg_number, range) in self.living.iter() {
            eprintln!("t{} --> {}...{}", reg_number, range.0, range.1);
        }
    }
}
