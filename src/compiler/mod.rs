pub mod file;
pub mod frontend;

pub fn compile(source_file: file::SrcFile) {
    use crate::compiler::frontend::lex;
    use crate::compiler::frontend::Manager;
    let mut manager = Manager::new(source_file);
    lex::tokenize(&mut manager);
}
