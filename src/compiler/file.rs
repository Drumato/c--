extern crate colored;
use colored::*;
pub struct SrcFile {
    pub abs_path: String,
    pub contents: String,
}

impl SrcFile {
    pub fn new(file_name: &str) -> Self {
        if let Some(contents) = read_c_file(file_name) {
            Self {
                abs_path: file_name.to_string(),
                contents: contents,
            }
        } else {
            Self {
                abs_path: "INVALID".to_string(),
                contents: file_name.to_string(),
            }
        }
    }
    pub fn dump_to_stderr(&self) {
        eprintln!("++++++++ {} ++++++++", self.abs_path.bold().green());
        eprintln!("{}", self.contents);
    }
}

fn read_c_file(s: &str) -> Option<String> {
    use std::fs;
    use std::path::Path;
    let filepath: &Path = Path::new(s);
    if filepath.exists() && filepath.is_file() {
        return Some(fs::read_to_string(s).unwrap());
    }
    if filepath.is_dir() {
        eprintln!("{} is directory.", filepath.to_str().unwrap());
    } else {
        eprintln!("not found such a file.\nusing command-line argument instead.",);
    }
    None
}

// SrcFile構造体に関するテスト
#[cfg(test)]
mod src_file_tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_src_file_with_valid_file() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/samples/add.c", std::env::var("C_ROOT").unwrap());
        let source_file = SrcFile::new(&file_path);
        assert_eq!(source_file.abs_path, file_path);

        let expected = std::fs::File::open(file_path)?;
        let metadata = expected.metadata()?;
        assert_eq!(source_file.contents.len(), metadata.len() as usize);
        Ok(())
    }
    #[test]
    #[ignore]
    fn test_src_file_with_invalid_file() {
        let file_path = "not_exist.c";
        let source_file = SrcFile::new(file_path);
        assert_eq!(source_file.abs_path, "INVALID");
        assert_eq!(source_file.contents.len(), file_path.len());

        let dir_path = std::env::var("C_ROOT").unwrap();
        let source_dir = SrcFile::new(&dir_path);
        assert_eq!(source_dir.abs_path, "INVALID");
        assert_eq!(source_dir.contents.len(), dir_path.len());
    }
}
