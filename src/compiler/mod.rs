pub mod backend;
pub mod file;
pub mod frontend;
pub mod ir;

use crate::structure::AssemblyFile;
use crate::target::Target;

pub fn compile(
    matches: &clap::ArgMatches,
    source_file: file::SrcFile,
    target: Target,
) -> AssemblyFile {
    if !source_file.abs_path.ends_with(".c") {
        // アセンブリ以下のレイヤが渡されたので,そのまま返す.
        return if source_file.abs_path.ends_with(".s") {
            AssemblyFile::new_intel_file(source_file.contents, target)
        } else {
            AssemblyFile::new_atandt_file(source_file.contents, target)
        };
    }

    // フロントエンド部の処理
    let manager = frontend::frontend_process(matches, source_file, &target);

    // バックエンド部の処理
    let s = backend::backend_process(matches, manager.entry_block, &target);

    if matches.is_present("atandt-syntax") {
        AssemblyFile::new_atandt_file(s, target)
    } else {
        AssemblyFile::new_intel_file(s, target)
    }
}
