extern crate colored;
use colored::*;
extern crate clap;

pub mod backend;
pub mod error;
pub mod file;
pub mod frontend;
pub mod ir;

pub fn compile(matches: &clap::ArgMatches, source_file: file::SrcFile) {
    use backend::Optimizer;
    use frontend::lex;
    use frontend::Manager;
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
        eprintln!(
            "++++++++ {} ++++++++",
            "dump three address code".bold().green()
        );
        manager.dump_tacs_to_stderr();
    }

    // TODO: 正準化

    // TODO: 命令選択

    // バックエンド部
    let entry_bb = manager.entry_block;
    let mut optimizer = Optimizer::new(entry_bb);

    // 制御フローグラフ構築
    // TODO: ベーシックブロック間のものを作る必要あり
    let bb = optimizer.entry_block.clone();
    optimizer.build_cfg_with_bb(bb);

    if matches.is_present("d-controlflow") {
        eprintln!("dump control-flow-graph to cfg.dot...");
        optimizer.dump_cfg_to_file();
        eprintln!("{}", "done!".bold().green());
    }

    // TODO: データフローグラフ構築(生存解析)
    optimizer.append_liveness_informations_to_cfg();

    // used,def集合の情報を含めたダンプ
    if matches.is_present("d-cfg-liveness") {
        eprintln!("dump control-flow-graph to cfg.dot...");
        optimizer.dump_cfg_liveness_to_file();
        eprintln!("{}", "done!".bold().green());
    }

    // TODO: レジスタ割付

    // TODO: コード生成
}
