extern crate clap;

pub mod arch;
pub mod cfg;
pub mod high_optimizer;
pub mod liveness;
pub mod regalloc;
pub mod translate_ir;

use crate::compiler::ir::three_address_code::function::IRFunction;
use crate::error::Error;
use crate::target::*;
use crate::util;

pub fn backend_process(
    matches: &clap::ArgMatches,
    entry_func: IRFunction,
    target: &Target,
) -> String {
    let mut high_opt = high_optimizer::HighOptimizer::new(entry_func);

    // 制御フローグラフ構築
    high_opt.build_cfg();

    if matches.is_present("d-controlflow") {
        // util::colored_message_to_stderr("dump control-flow-graph to cfg.dot...");
        // high_opt.dump_cfg_to_file();
        // util::colored_message_to_stderr("done!");
    }

    // データフローグラフ構築(レジスタ割付用の生存解析)
    high_opt.append_liveness_informations();

    // used,def集合の情報を含めたダンプ
    if matches.is_present("d-cfg-liveness") {
        // util::colored_message_to_stderr("dump control-flow-graph to cfg.dot...");
        // high_opt.dump_cfg_liveness_to_file();
        // util::colored_message_to_stderr("done!");
    }

    let mut blocks = high_opt.entry_func.blocks.clone();
    let block_number = blocks.len();

    for blk_idx in 0..block_number {
        let ir_number = blocks[blk_idx].tacs.len();

        // 生存情報の収集
        let (live_in, live_out) =
            high_opt.liveness_analysis(blocks[blk_idx].cfg_inbb.clone(), ir_number);

        // 生存情報の反映
        for (reg_number, range) in blocks[blk_idx].living.iter_mut() {
            for ir_idx in 0..ir_number {
                if !live_in[ir_idx].contains(reg_number) && live_out[ir_idx].contains(reg_number) {
                    range.0 = ir_idx;
                }
                if live_in[ir_idx].contains(reg_number) && !live_out[ir_idx].contains(reg_number) {
                    range.1 = ir_idx;
                }
            }
        }
    }

    high_opt.entry_func.blocks = blocks;

    if matches.is_present("d-liveness-info") {
        util::colored_prefix_to_stderr("dump liveness analysis informations");
        for bb in high_opt.entry_func.blocks.iter() {
            bb.dump_liveness();
        }
    }

    // レジスタ割付( 仮想レジスタ専用 )
    let available_registers = find_available_registers_each_archs(target);

    let mut allocated_blocks = Vec::new();
    let blocks = high_opt.entry_func.blocks.clone();
    for block in blocks.iter() {
        let allocated_block =
            high_opt.register_allocation_for_virtual_registers(block.clone(), available_registers);

        allocated_blocks.push(allocated_block);
    }

    high_opt.entry_func.blocks = blocks;

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
