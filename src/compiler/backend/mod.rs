extern crate clap;

pub mod arch;
pub mod cfg;
pub mod liveness;
pub mod regalloc;
pub mod translate_ir;

use std::collections::BTreeSet;

use crate::compiler::ir::three_address_code::BasicBlock;
use crate::compiler::util;
use crate::error::Error;
use crate::target::*;

// 機械独立なバックエンド操作を行う
pub struct HighOptimizer {
    pub entry_block: BasicBlock,
    pub cfg_inbb: ControlFlowGraphInBB,
}

impl HighOptimizer {
    pub fn new(entry_bb: BasicBlock) -> Self {
        let ir_length = entry_bb.tacs.len();
        Self {
            entry_block: entry_bb,
            cfg_inbb: ControlFlowGraphInBB::new(ir_length),
        }
    }
    pub fn dump_tacs_to_stderr(&self) {
        self.entry_block.dump_tacs_to_stderr_with_physical();
    }
}

type RegisterNumber = usize;
#[allow(dead_code)]
pub struct ControlFlowGraphInBB {
    succ: Vec<BTreeSet<usize>>,
    prev: Vec<BTreeSet<usize>>,
    used: Vec<BTreeSet<RegisterNumber>>,
    def: Vec<BTreeSet<RegisterNumber>>,
}
impl ControlFlowGraphInBB {
    fn new(len: usize) -> Self {
        Self {
            succ: vec![BTreeSet::new(); len],
            prev: vec![BTreeSet::new(); len],
            used: vec![BTreeSet::new(); len],
            def: vec![BTreeSet::new(); len],
        }
    }
}

// pub strut ControlFlowGraph{
//  BTreeMap<BasicBlockLabel, BTreeSet<usize>>
// }
//

pub fn backend_process(
    matches: &clap::ArgMatches,
    entry_bb: BasicBlock,
    target: &Target,
) -> String {
    let mut high_optimizer = HighOptimizer::new(entry_bb);

    // 制御フローグラフ構築
    let bb = high_optimizer.entry_block.clone();
    high_optimizer.build_cfg_with_bb(bb);

    if matches.is_present("d-controlflow") {
        util::colored_message_to_stderr("dump control-flow-graph to cfg.dot...");
        high_optimizer.dump_cfg_to_file();
        util::colored_message_to_stderr("done!");
    }

    // データフローグラフ構築(レジスタ割付用の生存解析)
    high_optimizer.append_liveness_informations_to_cfg();

    // used,def集合の情報を含めたダンプ
    if matches.is_present("d-cfg-liveness") {
        util::colored_message_to_stderr("dump control-flow-graph to cfg.dot...");
        high_optimizer.dump_cfg_liveness_to_file();
        util::colored_message_to_stderr("done!");
    }

    high_optimizer.liveness_analysis();

    if matches.is_present("d-liveness-info") {
        util::colored_prefix_to_stderr("dump liveness analysis informations");
        high_optimizer.entry_block.dump_liveness();
    }

    // レジスタ割付( 仮想レジスタ専用 )
    let available_registers = find_available_registers_each_archs(target);

    high_optimizer.register_allocation_for_virtual_registers(available_registers);
    if matches.is_present("d-higher-ir-regalloced") {
        util::colored_prefix_to_stderr("dump three address code( after register-allocation )");
        high_optimizer.dump_tacs_to_stderr();
    }

    // アーキテクチャごとの処理に移動
    match target.arch {
        Architecture::X86_64 => arch::x64::x64_process(matches, high_optimizer),
        _ => {
            // サポートしてないアーキテクチャはエラー
            Error::found_cant_support_architecture();
            String::new()
        }
    }
}

// レジスタ割付で使用可能なレジスタ数をチェック
fn find_available_registers_each_archs(target: &Target) -> usize {
    match target.arch {
        Architecture::X86_64 => 9,
        _ => {
            Error::found_cant_support_architecture();
            0
        }
    }
}
