pub mod lex;
pub mod node;
pub mod parse;
pub mod sema;
pub mod token;
pub mod types;

use crate::compiler::file;
use crate::compiler::ir::three_address_code::ThreeAddressCode;
pub struct Manager {
    src_file: file::SrcFile,
    tokens: Vec<token::Token>,
    // TODO: 後々 Vec<Function> に変更
    pub expr: node::Node,

    // パース処理用
    cur_token: usize,
    next_token: usize,

    // 3番地コード列
    pub tacs: Vec<ThreeAddressCode>,

    // レジスタ番号
    pub virt: usize,
    // pub label: usize,
}

impl Manager {
    pub fn new(src: file::SrcFile) -> Self {
        Self {
            src_file: src,
            tokens: Vec::new(),
            expr: node::Node::new((0, 0), node::NodeKind::INVALID),
            cur_token: 0,
            next_token: 1,
            tacs: Vec::new(),
            virt: 0,
        }
    }
}
