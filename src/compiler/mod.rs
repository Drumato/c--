extern crate clap;

pub mod error;
pub mod file;
pub mod frontend;
pub mod ir;

pub fn compile(matches: &clap::ArgMatches, source_file: file::SrcFile) {
    use crate::compiler::frontend::lex;
    use crate::compiler::frontend::Manager;
    let mut manager = Manager::new(source_file);

    // 字句解析
    lex::tokenize(&mut manager);

    // 構文解析
    manager.parse();

    // 意味解析
    manager.semantics();

    // 3番地コード生成
    manager.generate_three_address_code();

    if matches.is_present("d-hir") {
        manager.dump_tacs_to_stderr();
    }
}
