use crate::assembler::arch::x64::symbol::X64Symbol;
use crate::elf::elf64;
use crate::structure::AssemblyFile;

use std::collections::BTreeMap;
pub struct X64AssemblyFile {
    pub base_file: AssemblyFile,
    pub symbols_map: BTreeMap<String, X64Symbol>,

    pub relocations_map: BTreeMap<String, elf64::rela::Rela64>,
}

impl X64AssemblyFile {
    pub fn new(base_file: AssemblyFile) -> Self {
        Self {
            base_file: base_file,
            symbols_map: BTreeMap::new(),
            relocations_map: BTreeMap::new(),
        }
    }
}
