use crate::elf::elf64::*;

pub const PT_LOAD: Elf64Word = 1;
pub const PF_X: Elf64Word = 1 << 0;
pub const PF_W: Elf64Word = 1 << 1;
pub const PF_R: Elf64Word = 1 << 2;

#[repr(C)]
pub struct Phdr64 {
    pub p_type: Elf64Word,
    pub p_flags: Elf64Word,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: Elf64Xword,
    pub p_memsz: Elf64Xword,
    pub p_align: Elf64Xword,
}

impl Phdr64 {
    pub fn new() -> Self {
        Self {
            p_type: 0,
            p_flags: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
        }
    }
    pub fn size() -> usize {
        0x38
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for byte in self.p_type.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.p_flags.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.p_offset.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.p_vaddr.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.p_paddr.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.p_filesz.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.p_memsz.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.p_align.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        bytes
    }
}
