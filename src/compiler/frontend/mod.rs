pub mod lex;
pub mod token;

use crate::compiler::file;
pub struct Manager {
    src_file: file::SrcFile,
    tokens: Vec<token::Token>,
}

impl Manager {
    pub fn new(src: file::SrcFile) -> Self {
        Self {
            src_file: src,
            tokens: Vec::new(),
        }
    }
}
