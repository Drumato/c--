extern crate colored;
use colored::*;

use std::collections::BTreeMap;
use std::fs;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::Path;
use std::process::{Command, Stdio};

// testcase
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数のチェック
    if let Err(r) = std::env::var("C_ROOT") {
        panic!("{} -> C_ROOT", r);
    };

    // samplesディレクトリ以下を全て走査

    let samples_dir = std::env::var("C_ROOT").unwrap() + "/samples";
    let samples_path = Path::new(&samples_dir);

    // テストケースの構築
    let expected_map = build_expected()?;

    for entry in fs::read_dir(samples_path)? {
        let test_file_path = std::env::var("C_ROOT").unwrap()
            + "/samples/"
            + &entry?.file_name().into_string().unwrap();

        // subprocess の起動
        let binary_path = std::env::var("C_ROOT").unwrap() + "/target/debug/c--";

        let file = fs::File::create("test.S").unwrap();
        // ファイルからfdを経由してStdioを作る
        let out = unsafe { Stdio::from_raw_fd(file.as_raw_fd()) };

        let _compile_cmd = Command::new(&binary_path)
            .arg(&test_file_path.clone())
            .arg("-C")
            .stdout(out)
            .status()
            .expect("failed to spawn a process");

        // test.S -> a.out
        let _build_cmd = Command::new("gcc")
            .args(vec!["test.S"])
            .status()
            .expect("failed to spawn a process");

        let execute_status = Command::new("./a.out")
            .status()
            .expect("failed to spawn a process");

        // 終了ステータスのチェック
        if let Some(expected) = expected_map.get(&test_file_path) {
            if execute_status.code().unwrap() != *expected {
                eprintln!(
                    "{} -> {} expected {} but actual {}",
                    test_file_path,
                    "FAILED".bold().red(),
                    *expected,
                    execute_status
                );
            } else {
                eprintln!(
                    "{} -> {} ({})",
                    test_file_path,
                    "PASSED".bold().green(),
                    execute_status
                );
            }
        }
    }

    Ok(())
}

fn build_expected() -> Result<BTreeMap<String, i32>, Box<dyn std::error::Error>> {
    let mut expected_map: BTreeMap<String, i32> = BTreeMap::new();

    // samplesディレクトリ以下を全て走査

    let samples_dir = std::env::var("C_ROOT").unwrap() + "/samples";
    let samples_path = Path::new(&samples_dir);

    for entry in fs::read_dir(samples_path)? {
        // ファイルパスの階層を取り除いてパターンマッチ
        let test_file_path = std::env::var("C_ROOT").unwrap()
            + "/samples/"
            + &entry?.file_name().into_string().unwrap();
        let splitted_path: Vec<&str> = test_file_path.split('/').collect();
        let final_name = splitted_path[splitted_path.len() - 1];

        let exit_status = match final_name {
            "add.c" => 3,
            "huge_add.c" => 55,
            _ => {
                eprintln!("something went wrong -> {}", final_name);
                0
            }
        };
        expected_map.insert(test_file_path.to_string(), exit_status);
    }
    Ok(expected_map)
}
