extern crate colored;
use colored::*;
extern crate clap;

pub mod backend;
pub mod error;
pub mod file;
pub mod frontend;
pub mod ir;

use error::{Error, ErrorKind, ErrorMsg};
pub fn compile(matches: &clap::ArgMatches, source_file: file::SrcFile) {
    use backend::HighOptimizer;

    // フロントエンド部の処理
    let manager = frontend::frontend_process(matches, source_file);
    if matches.is_present("d-higher-ir") {
        eprintln!(
            "++++++++ {} ++++++++",
            "dump three address code".bold().green()
        );
        manager.dump_tacs_to_stderr();
    }

    // TODO: バックエンド部をまとめる
    // let assembly_file = backend::backend_process();

    let entry_bb = manager.entry_block;
    let mut high_optimizer = HighOptimizer::new(entry_bb);

    // 制御フローグラフ構築
    // TODO: ベーシックブロック間のものを作る必要あり
    let bb = high_optimizer.entry_block.clone();
    high_optimizer.build_cfg_with_bb(bb);

    if matches.is_present("d-controlflow") {
        eprintln!("dump control-flow-graph to cfg.dot...");
        high_optimizer.dump_cfg_to_file();
        eprintln!("{}", "done!".bold().green());
    }

    // データフローグラフ構築(レジスタ割付用の生存解析)
    high_optimizer.append_liveness_informations_to_cfg();

    // used,def集合の情報を含めたダンプ
    if matches.is_present("d-cfg-liveness") {
        eprintln!("dump control-flow-graph to cfg.dot...");
        high_optimizer.dump_cfg_liveness_to_file();
        eprintln!("{}", "done!".bold().green());
    }

    high_optimizer.liveness_analysis();
    if matches.is_present("d-liveness-info") {
        eprintln!(
            "++++++++ {} ++++++++",
            "dump liveness analysis informations".bold().green()
        );
        high_optimizer.entry_block.dump_liveness();
    }

    // レジスタ割付( 仮想レジスタ専用 )
    let available_registers = find_available_registers_each_archs();

    high_optimizer.register_allocation_for_virtual_registers(available_registers);
    if matches.is_present("d-higher-ir-regalloced") {
        eprintln!(
            "++++++++ {} ++++++++",
            "dump three address code( after register-allocation )"
                .bold()
                .green()
        );
        high_optimizer.dump_tacs_to_stderr();
    }

    // アーキテクチャごとの低レベルなIRに変換
    if cfg!(target_arch = "x86_64") {
        // TODO: 低レベルなIRFunctionのVectorが返る方が自然
        use ir::x64::X64Optimizer;
        let _x64_optimizer: X64Optimizer = HighOptimizer::translate_tacs_to_x64(high_optimizer);

    // backend::x64_process(low_irs);
    } else {
        let err = Error::new(
            ErrorKind::Compile,
            (0, 0),
            ErrorMsg::CantSupportSuchAnArchitecture,
        );
        err.compile_error();
    }

    // TODO: コード生成
}

// レジスタ割付で使用可能なレジスタ数をチェック
fn find_available_registers_each_archs() -> usize {
    if cfg!(target_arch = "x86_64") {
        9
    } else {
        let err = Error::new(
            ErrorKind::Compile,
            (0, 0),
            ErrorMsg::CantSupportSuchAnArchitecture,
        );
        err.compile_error();
        0
    }
}
