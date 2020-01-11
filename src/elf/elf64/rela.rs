use crate::elf::elf64::*;
#[derive(Debug)]
pub struct Rela64 {
    pub r_offset: Elf64Addr,
    pub r_info: Elf64Xword,
    pub r_addend: Elf64Sxword,
}

impl Rela64 {
    pub fn new(addend: Elf64Sxword) -> Self {
        Self {
            r_offset: 0,
            r_info: 0,
            r_addend: addend,
        }
    }
    pub fn new_unsafe(binary: Vec<u8>) -> Self {
        unsafe { std::ptr::read(binary.as_ptr() as *const Self) }
    }
    pub fn bind(info: u64) -> usize {
        info as usize >> 32
    }
    pub fn size() -> usize {
        24
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for byte in self.r_offset.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.r_info.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.r_addend.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        bytes
    }
}
