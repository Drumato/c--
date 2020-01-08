extern crate colored;
use colored::*;

use std::fs;

pub struct SrcFile {
    pub abs_path: String,
    pub contents: String,
}

impl SrcFile {
    pub fn new(file_name: &str) -> Self {
        Self {
            abs_path: file_name.to_string(),
            contents: read_c_file(file_name),
        }
    }
    pub fn is_file(file_name: &str) -> bool {
        use std::path::Path;
        let filepath: &Path = Path::new(file_name);
        if !filepath.exists() || !filepath.is_file() || filepath.is_dir() {
            return false;
        }
        true
    }
    pub fn get_entry_or_default(&self, specified_name: Option<&str>) -> String {
        if let Some(name) = specified_name {
            return name.to_string();
        }
        String::from("main")
    }
}

fn read_c_file(s: &str) -> String {
    fs::read_to_string(s).unwrap()
}
