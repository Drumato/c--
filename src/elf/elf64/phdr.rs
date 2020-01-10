use crate::elf::elf64::*;

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
    pub fn size() -> usize {
        0x38
    }
}
