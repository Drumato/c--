extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

mod assembler;
mod compiler;
mod elf;
mod error;
mod linker;
mod structure;
mod target;
mod util;

use compiler::file::SrcFile;
use target::Target;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let source_file = setup(&matches);

    // 'sample.c' -> 'sample'
    let output_base_path = source_file.abs_path.split(".").collect::<Vec<&str>>()[0].to_string();

    // compile phase
    let assembly_file = compiler::compile(&matches, source_file, Target::new());

    if matches.is_present("stop-compile") {
        // 取り敢えず標準出力.
        // 後々ファイルダンプを考える
        println!("{}", assembly_file.code);
        return Ok(());
    }

    // assemble phase
    let do_link = !matches.is_present("stop-assemble");

    let object_file = assembler::assemble(&matches, assembly_file, do_link);

    if !do_link {
        let output_path = output_base_path + ".o";
        util::object_file_dump(output_path, object_file);
        return Ok(());
    }

    // リンクまでやる
    let executable_file = linker::link(object_file);
    util::object_file_dump("a.out".to_string(), executable_file);

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
