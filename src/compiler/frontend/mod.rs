extern crate clap;

pub mod lex;
pub mod node;
pub mod parse;
pub mod sema;
pub mod token;
pub mod types;

use crate::compiler::file;
use crate::compiler::ir::three_address_code::BasicBlock;
use crate::compiler::target::Target;
use crate::compiler::util;
pub struct Manager {
    src_file: file::SrcFile,
    tokens: Vec<token::Token>,
    // TODO: 後々 Vec<Function> に変更
    pub expr: node::Node,

    // パース処理用
    cur_token: usize,
    next_token: usize,

    // 3番地コード列
    // TODO: Vec<IRFunction> -> BasicBlock -> Vec<ThreeAddressCode> にする
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

pub fn frontend_process(
    matches: &clap::ArgMatches,
    source_file: file::SrcFile,
    _target: &Target,
) -> Manager {
    let mut manager = Manager::new(source_file);

    // 字句解析
    lex::tokenize(&mut manager);

    // 構文解析
    manager.parse();

    // 意味解析
    manager.semantics();

    // TODO: 駆動レコード生成部を作る
    // manager.alloc_frame();

    // 3番地コード生成
    manager.generate_three_address_code();

    if matches.is_present("d-higher-ir") {
        util::colored_prefix_to_stderr("dump three address code");
        manager.dump_tacs_to_stderr();
    }

    manager
}
