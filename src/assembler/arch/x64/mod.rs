pub mod asmtoken;
pub mod lex_intel;

use crate::structure::AssemblyFile;
pub struct X64Assembler {
    src_file: AssemblyFile,
    // アセンブリコードを字句解析してここに格納
    tokens: Vec<asmtoken::AsmToken>,
}

impl X64Assembler {
    fn new(file: AssemblyFile) -> Self {
        Self {
            src_file: file,
            tokens: Vec::new(),
        }
    }
}

pub fn assemble(_matches: &clap::ArgMatches, assembly_file: AssemblyFile) {
    let mut assembler = X64Assembler::new(assembly_file);

    // 字句解析
    // 現状Intel記法のみ対応
    lex_intel::lexing_intel_syntax(&mut assembler);
}
