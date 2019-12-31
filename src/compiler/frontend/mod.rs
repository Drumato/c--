pub mod lex;
pub mod node;
pub mod parse;
pub mod sema;
pub mod token;
pub mod types;

use crate::compiler::file;
pub struct Manager {
    src_file: file::SrcFile,
    tokens: Vec<token::Token>,
    // TODO: 後々 Vec<Function> に変更
    expr: node::Node,

    // パース処理用
    cur_token: usize,
    next_token: usize,
}

impl Manager {
    pub fn new(src: file::SrcFile) -> Self {
        Self {
            src_file: src,
            tokens: Vec::new(),
            expr: node::Node::new((0, 0), node::NodeKind::INVALID),
            cur_token: 0,
            next_token: 1,
        }
    }
}
