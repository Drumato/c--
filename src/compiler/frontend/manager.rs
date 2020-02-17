use crate::compiler::file;
use crate::compiler::frontend::node;
use crate::compiler::frontend::token;
use crate::compiler::frontend::variable;
use crate::compiler::ir::three_address_code::function::IRFunction;

use std::collections::BTreeMap;

pub struct Manager {
    pub src_file: file::SrcFile,
    pub tokens: Vec<token::Token>,

    // TODO: モジュールを受け取るように変更
    pub functions: Vec<node::Function>,

    // パース処理用
    pub cur_token: usize,
    pub next_token: usize,
    pub params: BTreeMap<String, variable::Variable>,
    pub var_map: BTreeMap<String, variable::Variable>,

    // 3番地コード列
    // TODO: モジュールを受け取るように変更
    pub ir_funcs: Vec<IRFunction>,
    pub cur_bb: usize,

    // レジスタ番号
    pub virt: usize,
    pub label: usize,
}

impl Manager {
    pub fn new(src: file::SrcFile) -> Self {
        Self {
            src_file: src,
            tokens: Vec::new(),
            functions: Vec::new(),
            cur_token: 0,
            next_token: 1,
            params: BTreeMap::new(),
            var_map: BTreeMap::new(),
            ir_funcs: Vec::new(),
            cur_bb: 0,
            virt: 0,
            label: 0,
        }
    }
    pub fn dump_ast_to_stderr(&self) {
        for func in self.functions.iter() {
            func.dump_ast();
        }
    }
}
