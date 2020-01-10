use crate::elf::elf64::*;

/* definitions for st_info(bind) */
pub const STB_GLOBAL: u8 = 1; /* Global symbol */

/* definitions for st_info(type) */
pub const STT_FUNC: u8 = 2; /* Symbol is a code object */

#[repr(C)]
pub struct Symbol64 {
    pub st_name: Elf64Word,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: Elf64Section,
    pub st_value: Elf64Addr,
    pub st_size: Elf64Xword,
}
impl Symbol64 {
    pub fn size() -> usize {
        24
    }
    pub fn new_null_symbol() -> Self {
        Self {
            st_name: 0,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
            st_value: 0,
            st_size: 0,
        }
    }
    pub fn new_global_function(name_i: Elf64Word, length: Elf64Xword, offset: Elf64Addr) -> Self {
        Self {
            st_name: name_i,
            st_info: (STB_GLOBAL << 4) + STT_FUNC,
            st_other: 0,
            st_shndx: 1, // .text セクションが1番目にあることを決め打ち
            st_value: offset,
            st_size: length,
        }
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for byte in self.st_name.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.st_info.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.st_other.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.st_shndx.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.st_value.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.st_size.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        bytes
    }
}
