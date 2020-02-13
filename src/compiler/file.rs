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
}

fn read_c_file(s: &str) -> String {
    fs::read_to_string(s).unwrap()
}
