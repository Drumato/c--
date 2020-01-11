pub mod analyze;
pub mod asmtoken;
pub mod assembler;
pub mod codegen;
pub mod elf;
pub mod file;
pub mod inst;
pub mod lexer;
pub mod opcodes;
pub mod parser;
pub mod symbol;

use crate::elf::elf64;
use crate::structure::{AssemblyFile, Syntax};
use crate::util;

pub fn assemble(
    matches: &clap::ArgMatches,
    mut assembly_file: AssemblyFile,
    do_link: bool,
) -> elf64::ELF64 {
    if do_link {
        // アセンブリコードにスタートアップルーチンを追加
        let start_up_routine = if let Syntax::INTEL = assembly_file.syntax {
            std::env::var("C_ROOT").unwrap() + "/lib/start_up_linux64.s"
        } else {
            std::env::var("C_ROOT").unwrap() + "/lib/start_up_linux64.S"
        };
        eprintln!(
            "using default start up routine ... -> {}",
            &start_up_routine
        );
        assembly_file.code += &util::read_file_contents(start_up_routine);
    }

    let x64_assembly_file = file::X64AssemblyFile::new(assembly_file);
    let mut assembler = assembler::X64Assembler::new(x64_assembly_file);

    // 字句解析
    if let Syntax::INTEL = &assembler.src_file.base_file.syntax {
        lexer::lex_intel::lexing_intel_syntax(&mut assembler);
    } else {
        lexer::lex_atandt::lexing_atandt_syntax(&mut assembler);
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

    // 再配置情報の構築
    // シンボルテーブルを検索して,再配置テーブルに存在すれば情報更新
    assembler.setup_relocations();

    // オブジェクトファイル生成
    // 再配置可能オブジェクトファイルを表現する構造体にまとめる
    let mut reloc_elf = elf64::ELF64::new_object_file();

    /* (null section) */
    reloc_elf.add_null_section();
    /* .text */
    reloc_elf.add_text_section_x64(&assembler);
    /* .symtab */
    reloc_elf.add_symtab_section_x64(&assembler);
    /* .strtab */
    reloc_elf.add_strtab_section_x64(&assembler);
    /* .rela.text */
    reloc_elf.add_relatext_section_x64(&assembler);
    /* .shstrtab */
    let section_names = vec![".text", ".symtab", ".strtab", ".rela.text", ".shstrtab"];
    reloc_elf.add_shstrtab_section_x64(section_names);

    reloc_elf.finalize();
    reloc_elf
}
