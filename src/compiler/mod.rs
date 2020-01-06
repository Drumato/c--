pub mod backend;
pub mod error;
pub mod file;
pub mod frontend;
pub mod ir;
pub mod util;

use crate::structure::AssemblyFile;
use crate::target::Target;

pub fn compile(
    matches: &clap::ArgMatches,
    source_file: file::SrcFile,
    target: Target,
) -> AssemblyFile {
    // フロントエンド部の処理
    let manager = frontend::frontend_process(matches, source_file, &target);

    // バックエンド部の処理
    let s = backend::backend_process(matches, manager.entry_block, &target);

    AssemblyFile::new(s, target)
}
