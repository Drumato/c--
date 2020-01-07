pub mod asmtoken;
pub mod inst;
pub mod lex_intel;
pub mod parse;
pub mod parse_intel;

use crate::structure::{AssemblyFile, Syntax};

use std::collections::BTreeMap;

pub fn assemble(_matches: &clap::ArgMatches, assembly_file: AssemblyFile) {
    let x64_assembly_file = X64AssemblyFile::new(assembly_file);
    let mut assembler = X64Assembler::new(x64_assembly_file);

    // 字句解析
    // 現状Intel記法のみ対応
    if let Syntax::INTEL = &assembler.src_file.base_file.syntax {
        lex_intel::lexing_intel_syntax(&mut assembler);
    } else {
        // lex_atandt::lexing_atandt_syntax(&mut assembler);
    }

    // 構文解析
    // ASTみたいなのは作らず,単純に命令列を作成する.
    // 命令列を持つ構造体はX64AssemblyFileとする.
    // この構造体に各シンボルの情報も格納していく.
    if let Syntax::INTEL = &assembler.src_file.base_file.syntax {
        assembler.parse_intel_syntax();
    } else {
    }
}

pub struct X64Assembler {
    src_file: X64AssemblyFile,
    // アセンブリコードを字句解析してここに格納
    tokens: Vec<asmtoken::AsmToken>,

    // パース処理用
    cur_token: usize,
    next_token: usize,
}

impl X64Assembler {
    fn new(file: X64AssemblyFile) -> Self {
        Self {
            src_file: file,
            tokens: Vec::new(),
            cur_token: 0,
            next_token: 1,
        }
    }
}

pub struct X64AssemblyFile {
    base_file: AssemblyFile,
    symbols_map: BTreeMap<String, X64Symbol>,
    // relocations
}

impl X64AssemblyFile {
    fn new(base_file: AssemblyFile) -> Self {
        Self {
            base_file: base_file,
            symbols_map: BTreeMap::new(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct X64Symbol {
    code_size: usize,
    insts: Vec<inst::X64Instruction>,
    is_global: bool,
}

impl X64Symbol {
    fn new_global() -> Self {
        Self {
            code_size: 0,
            insts: Vec::new(),
            is_global: true,
        }
    }
    fn new_local() -> Self {
        Self {
            code_size: 0,
            insts: Vec::new(),
            is_global: false,
        }
    }
}
