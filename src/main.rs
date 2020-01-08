extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

mod assembler;
mod compiler;
mod error;
mod structure;
mod target;
mod util;

use compiler::file::SrcFile;
use target::Target;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let source_file = setup(&matches);

    // compile phase
    let assembly_file = compiler::compile(&matches, source_file, Target::new());

    if matches.is_present("stop-compile") {
        // 取り敢えず標準出力.
        // 後々ファイルダンプを考える
        println!("{}", assembly_file.code);
        return Ok(());
    }

    // assemble phase
    assembler::assemble(&matches, assembly_file);
    Ok(())
}

fn setup(matches: &clap::ArgMatches) -> SrcFile {
    // 環境変数のチェック
    if let Err(r) = std::env::var("C_ROOT") {
        panic!("{} -> C_ROOT", r);
    };

    let file_name = matches.value_of("source").unwrap();

    // ファイルが存在しなければexit
    if !SrcFile::is_file(&file_name) {
        output_invalid_file_error();
    }

    SrcFile::new(file_name)
}

fn output_invalid_file_error() -> ! {
    let err = error::Error::new(
        error::ErrorKind::Compile,
        (0, 0),
        error::ErrorMsg::InvalidCFileOrDirectory,
    );
    err.compile_error();
    std::process::exit(1);
}
