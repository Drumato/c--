extern crate clap;

pub mod arch;

use crate::elf::elf64;
use crate::structure::AssemblyFile;
use crate::target::*;

pub fn assemble(
    matches: &clap::ArgMatches,
    assembly_file: AssemblyFile,
    do_link: bool,
) -> elf64::ELF64 {
    match assembly_file.target.arch {
        Architecture::X86_64 => arch::x64::assemble(matches, assembly_file, do_link),
        _ => {
            panic!("invalid architecture found");
        }
    }
}
