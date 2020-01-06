extern crate clap;

pub mod arch;

use crate::structure::AssemblyFile;
use crate::target::*;

pub fn assemble(matches: &clap::ArgMatches, assembly_file: AssemblyFile) {
    match assembly_file.target.arch {
        Architecture::X86_64 => arch::x64::assemble(matches, assembly_file),
        _ => {}
    }
}
