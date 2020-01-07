extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

mod assembler;
mod compiler;
mod error;
mod structure;
mod target;

use compiler::file::SrcFile;
use target::Target;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // 環境変数のチェック
    if let Err(r) = std::env::var("C_ROOT") {
        panic!("{} -> C_ROOT", r);
    };

    let file_name = matches.value_of("source").unwrap();

    let source_file = SrcFile::new(file_name);

    // デバッグ用.読み込んだファイルに関する情報を出力する.
    if matches.is_present("d-source") {
        source_file.dump_to_stderr();
    }

    // compile phase
    let mut assembly_file = compiler::compile(&matches, source_file, Target::new());

    if matches.is_present("stop-compile") {
        // 取り敢えず標準出力.
        // 後々ファイルダンプを考える
        println!("{}", assembly_file.code);
        return Ok(());
    }

    // assemble phase
    if matches.is_present("at-and-t-syntax") {
        assembly_file.syntax = structure::Syntax::ATANDT;
    }
    assembler::assemble(&matches, assembly_file);
    Ok(())
}
