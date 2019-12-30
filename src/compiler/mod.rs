pub mod error;
pub mod file;
pub mod frontend;

pub fn compile(source_file: file::SrcFile) {
    use crate::compiler::frontend::lex;
    use crate::compiler::frontend::Manager;
    let mut manager = Manager::new(source_file);

    // 字句解析
    lex::tokenize(&mut manager);

    // 構文解析
    manager.parse();
}
