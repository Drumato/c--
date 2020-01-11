extern crate colored;
use colored::*;

use crate::elf::elf64;

use std::io::{BufWriter, Write};
use std::os::unix::fs::OpenOptionsExt;

pub fn colored_prefix_to_stderr(msg: &str) {
    eprintln!("++++++++ {} ++++++++", msg.bold().green());
}

pub fn colored_message_to_stderr(msg: &str) {
    eprintln!("{}", msg.bold().green());
}
pub fn object_file_dump(output_path: String, obj_file: elf64::ELF64) {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .mode(0o755)
        .open(output_path)
        .unwrap();
    let mut writer = BufWriter::new(file);
    match writer.write_all(&obj_file.to_binary()) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    match writer.flush() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}

pub fn read_file_contents(path: String) -> String {
    use std::fs;

    fs::read_to_string(path).unwrap()
}
