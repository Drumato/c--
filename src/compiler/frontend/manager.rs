use crate::compiler::file;
use crate::compiler::frontend::node;
use crate::compiler::frontend::token;
use crate::compiler::frontend::variable;
use crate::compiler::ir::three_address_code::function::IRFunction;

use std::collections::BTreeMap;

pub struct Manager {
    pub src_file: file::SrcFile,
    pub tokens: Vec<token::Token>,

    // 単一関数のみ許容するように変更
    pub entry_func: node::Function,

    // パース処理用
    pub cur_token: usize,
    pub next_token: usize,
    pub var_map: BTreeMap<String, variable::Variable>,

    // 3番地コード列
    // 単一関数のみ許容するように変更
    pub ir_func: IRFunction,
    pub cur_bb: usize,

    // レジスタ番号
    pub virt: usize,
    // pub label: usize,
}

impl Manager {
    pub fn new(src: file::SrcFile) -> Self {
        let entry_point = src.get_entry_or_default(None);
        Self {
            src_file: src,
            tokens: Vec::new(),
            entry_func: node::Function::init("none".to_string(), (0, 0)),
            cur_token: 0,
            next_token: 1,
            var_map: BTreeMap::new(),
            ir_func: IRFunction::new(entry_point),
            cur_bb: 0,
            virt: 0,
        }
    }
    pub fn dump_ast_to_stderr(&self) {
        self.entry_func.dump_ast();
    }
}
