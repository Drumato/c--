extern crate clap;

pub mod arch;
pub mod cfg;
pub mod high_optimizer;
pub mod liveness;
pub mod regalloc;
pub mod translate_ir;

use crate::compiler::ir::three_address_code::BasicBlock;
use crate::error::Error;
use crate::target::*;
use crate::util;

pub fn backend_process(
    matches: &clap::ArgMatches,
    entry_bb: BasicBlock,
    target: &Target,
) -> String {
    let mut high_opt = high_optimizer::HighOptimizer::new(entry_bb);

    // 制御フローグラフ構築
    let bb = high_opt.entry_block.clone();
    high_opt.build_cfg_with_bb(bb);

    if matches.is_present("d-controlflow") {
        util::colored_message_to_stderr("dump control-flow-graph to cfg.dot...");
        high_opt.dump_cfg_to_file();
        util::colored_message_to_stderr("done!");
    }

    // データフローグラフ構築(レジスタ割付用の生存解析)
    high_opt.append_liveness_informations_to_cfg();

    // used,def集合の情報を含めたダンプ
    if matches.is_present("d-cfg-liveness") {
        util::colored_message_to_stderr("dump control-flow-graph to cfg.dot...");
        high_opt.dump_cfg_liveness_to_file();
        util::colored_message_to_stderr("done!");
    }

    high_opt.liveness_analysis();

    if matches.is_present("d-liveness-info") {
        util::colored_prefix_to_stderr("dump liveness analysis informations");
        high_opt.entry_block.dump_liveness();
    }

    // レジスタ割付( 仮想レジスタ専用 )
    let available_registers = find_available_registers_each_archs(target);

    high_opt.register_allocation_for_virtual_registers(available_registers);
    if matches.is_present("d-higher-ir-regalloced") {
        util::colored_prefix_to_stderr("dump three address code( after register-allocation )");
        high_opt.dump_tacs_to_stderr();
    }

    // アーキテクチャごとの処理に移動
    match target.arch {
        Architecture::X86_64 => arch::x64::x64_process(matches, high_opt),
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
