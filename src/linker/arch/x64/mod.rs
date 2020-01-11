pub mod linker;

use crate::elf::elf64;

pub fn link(object_file: elf64::ELF64) -> elf64::ELF64 {
    let mut x64_linker = linker::X64StaticLinker::new(object_file);
    x64_linker.link();
    x64_linker.exec_file
}
