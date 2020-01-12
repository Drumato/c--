extern crate colored;
use colored::*;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

// testcase
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数のチェック
    if let Err(r) = std::env::var("C_ROOT") {
        panic!("{} -> C_ROOT", r);
    };

    // samplesディレクトリ以下を全て走査
    let samples_dir = get_testcases_located_path();
    let samples_path = Path::new(&samples_dir);

    // テストケースの構築
    let expected_map = build_expected()?;

    // 各テストケースについて
    for entry in fs::read_dir(samples_path)? {
        let iter_file_name = entry?.file_name().into_string().unwrap();
        let test_file_path = get_single_testcase_path(iter_file_name);

        // subprocess の起動
        let binary_path = get_cminus_binary_path();

        exec_compile_command(binary_path, test_file_path.clone());

        let execute_status = get_executed_elf_status().code().unwrap();

        // 終了ステータスのチェック
        check_given_status_is_success(&expected_map, test_file_path, execute_status);
    }

    Ok(())
}

fn build_expected() -> Result<BTreeMap<String, i32>, Box<dyn std::error::Error>> {
    let mut expected_map: BTreeMap<String, i32> = BTreeMap::new();

    // samplesディレクトリ以下を全て走査

    let samples_dir = get_testcases_located_path();
    let samples_path = Path::new(&samples_dir);

    for entry in fs::read_dir(samples_path)? {
        // ファイルパスの階層を取り除いてパターンマッチ
        let iter_file_name = entry?.file_name().into_string().unwrap();
        let test_file_path = get_single_testcase_path(iter_file_name);

        let splitted_path: Vec<&str> = test_file_path.split('/').collect();
        let final_name = splitted_path[splitted_path.len() - 1];

        let exit_status = match final_name {
            "add.c" => 3,
            "huge_add.c" => 55,
            "sub.c" => 1,
            _ => {
                eprintln!("something went wrong -> {}", final_name);
                0
            }
        };
        expected_map.insert(test_file_path.to_string(), exit_status);
    }
    Ok(expected_map)
}

fn get_testcases_located_path() -> String {
    std::env::var("C_ROOT").unwrap() + "/samples"
}

fn get_single_testcase_path(single_file: String) -> String {
    let samples_dir = get_testcases_located_path();
    samples_dir + "/" + &single_file
}

fn get_cminus_binary_path() -> String {
    std::env::var("C_ROOT").unwrap() + "/target/debug/c--"
}

fn exec_compile_command(binary_path: String, test_file_path: String) {
    // $ c-- <test_file_path>
    let _compile_cmd = Command::new(&binary_path)
        .arg(&test_file_path.clone())
        .status()
        .expect("failed to spawn a process");
}

fn get_executed_elf_status() -> std::process::ExitStatus {
    // $ ./a.out
    Command::new("./a.out")
        .status()
        .expect("failed to spawn a process")
}

fn check_given_status_is_success(
    expected_map: &BTreeMap<String, i32>,
    test_file_path: String,
    execute_status: i32,
) {
    if let Some(expected) = expected_map.get(&test_file_path) {
        if execute_status != *expected {
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
