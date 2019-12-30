extern crate yaml_rust;
#[macro_use]
extern crate clap;
use clap::App;

mod compiler;
use compiler::file::SrcFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // 環境変数のチェック
    if let Err(r) = std::env::var("C_ROOT") {
        panic!("{} -> C_ROOT", r);
    };

    // 現状Linuxのみ対応
    if cfg!(target_os = "linux") {
        linux_main(&matches)
    } else {
        panic!("We're not support on this os...");
    }
}

fn linux_main(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = matches.value_of("source").unwrap();

    let source_file = SrcFile::new(file_name);

    // デバッグ用.読み込んだファイルに関する情報を出力する.
    if matches.is_present("d-source") {
        source_file.dump_to_stderr();
    }

    // 後々コンパイル後の構造体を吐くように設定.
    compiler::compile(source_file);

    Ok(())
}
