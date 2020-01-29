extern crate clap;

pub mod alloc_frame;
pub mod lex;
pub mod manager;
pub mod node;
pub mod parse;
pub mod sema;
pub mod token;
pub mod types;
pub mod variable;

use crate::compiler::file;
use crate::target::Target;
use crate::util;

pub fn frontend_process(
    matches: &clap::ArgMatches,
    source_file: file::SrcFile,
    target: &Target,
) -> manager::Manager {
    let mut manager = manager::Manager::new(source_file);

    // 字句解析
    lex::tokenize(&mut manager);

    // 構文解析
    manager.parse();

    if matches.is_present("d-ast") {
        util::colored_prefix_to_stderr("dump AST");
        manager.dump_ast_to_stderr();
    }

    // 意味解析
    manager.semantics();

    // 駆動レコード部
    // 今はx64だけを想定
    if target.is_x86_64() {
        manager.alloc_frame();
    }

    // 3番地コード生成
    manager.generate_three_address_code();

    if matches.is_present("d-higher-ir") {
        util::colored_prefix_to_stderr("dump three address code");
        manager.dump_tacs_to_stderr();
    }

    manager
}
