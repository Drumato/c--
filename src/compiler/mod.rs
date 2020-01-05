pub mod backend;
pub mod error;
pub mod file;
pub mod frontend;
pub mod ir;
pub mod target;
pub mod util;

pub fn compile(matches: &clap::ArgMatches, source_file: file::SrcFile, target: target::Target) {
    // フロントエンド部の処理
    let manager = frontend::frontend_process(matches, source_file, &target);

    // バックエンド部の処理
    let s = backend::backend_process(matches, manager.entry_block, &target);

    println!("{}", s);
}
