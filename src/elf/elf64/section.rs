use crate::elf::elf64::shdr::Shdr64;
pub struct Section64 {
    pub name: String,
    pub header: Shdr64,
    pub bytes: Vec<u8>,
}

impl Section64 {
    pub fn new(name: String, header: Shdr64, bytes: Vec<u8>) -> Self {
        Self {
            name: name,
            header: header,
            bytes: bytes,
        }
    }
    pub fn new_null_section() -> Self {
        Self {
            name: "null".to_string(),
            header: Shdr64::new_null_shdr(),
            bytes: Vec::new(),
        }
    }
}
