pub mod arch;

use crate::elf::elf64;
use crate::target::*;

pub fn link(object_file: elf64::ELF64) -> elf64::ELF64 {
    match check_elf_architecture(&object_file.ehdr) {
        Architecture::X86_64 => arch::x64::link(object_file),
        _ => {
            panic!("invalid architecture found");
        }
    }
}

fn check_elf_architecture(_ehdr: &elf64::ehdr::Ehdr64) -> Architecture {
    // if ehdr.e_machine == 0x3e
    Architecture::X86_64
}
