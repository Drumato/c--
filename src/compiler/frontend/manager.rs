use crate::compiler::file;
use crate::compiler::frontend::node;
use crate::compiler::frontend::token;
use crate::compiler::ir::three_address_code::BasicBlock;
pub struct Manager {
    pub src_file: file::SrcFile,
    pub tokens: Vec<token::Token>,
    pub expr: node::Node,

    // パース処理用
    pub cur_token: usize,
    pub next_token: usize,

    // 3番地コード列
    pub entry_block: BasicBlock,

    // レジスタ番号
    pub virt: usize,
    // pub label: usize,
}

impl Manager {
    pub fn new(src: file::SrcFile) -> Self {
        let entry_function = src.get_entry_or_default(None);
        Self {
            src_file: src,
            tokens: Vec::new(),
            expr: node::Node::new((0, 0), node::NodeKind::INVALID),
            cur_token: 0,
            next_token: 1,
            entry_block: BasicBlock::new(entry_function),
            virt: 0,
        }
    }
}
