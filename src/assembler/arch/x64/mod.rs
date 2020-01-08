pub mod analyze;
pub mod asmtoken;
pub mod codegen;
pub mod inst;
pub mod lex;
pub mod lex_atandt;
pub mod lex_intel;
pub mod opcodes;
pub mod parse;
pub mod parse_atandt;
pub mod parse_intel;

use crate::structure::{AssemblyFile, Syntax};
use crate::util;

use std::collections::BTreeMap;

pub fn assemble(matches: &clap::ArgMatches, assembly_file: AssemblyFile) {
    let x64_assembly_file = X64AssemblyFile::new(assembly_file);
    let mut assembler = X64Assembler::new(x64_assembly_file);

    // 字句解析
    if let Syntax::INTEL = &assembler.src_file.base_file.syntax {
        lex_intel::lexing_intel_syntax(&mut assembler);
    } else {
        lex_atandt::lexing_atandt_syntax(&mut assembler);
    }

    // 構文解析
    // ASTみたいなのは作らず,単純に命令列を作成する.
    if let Syntax::INTEL = &assembler.src_file.base_file.syntax {
        assembler.parse_intel_syntax();
    } else {
        assembler.parse_atandt_syntax();
    }

    if matches.is_present("d-instructions") {
        util::colored_prefix_to_stderr("dump x64 instructions");
        assembler.dump_instructions_to_stderr();
    }

    // オペランド解析
    // コード生成の為に必要な情報をInst構造体に保存する
    assembler.analyze();

    // コード生成
    // symbols_mapの各エントリが機械語を保持するように
    assembler.codegen();

    util::colored_prefix_to_stderr("dump x64 machine-code");
    if let Some(symbol) = assembler.src_file.symbols_map.get("main") {
        for b in symbol.codes.iter() {
            eprintln!("0x{:x}", b);
        }
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
    fn dump_instructions_to_stderr(&self) {
        for (symbol_name, symbol_info) in self.src_file.symbols_map.iter() {
            eprintln!("{}'s instructions:", symbol_name);
            for inst in symbol_info.insts.iter() {
                eprintln!("\t{}", inst.to_string());
            }
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
    codes: Vec<u8>,
    insts: Vec<inst::X64Instruction>,
    is_global: bool,
}

impl X64Symbol {
    fn new_global() -> Self {
        Self {
            codes: Vec::new(),
            insts: Vec::new(),
            is_global: true,
        }
    }
    fn new_local() -> Self {
        Self {
            codes: Vec::new(),
            insts: Vec::new(),
            is_global: false,
        }
    }
}
