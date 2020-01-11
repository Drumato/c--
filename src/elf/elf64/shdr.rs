use crate::elf::elf64::rela::Rela64;
use crate::elf::elf64::symbol::Symbol64;
use crate::elf::elf64::*;

/* definitions for sh_type */
pub const SHT_PROGBITS: Elf64Word = 1;
pub const SHT_SYMTAB: Elf64Word = 2;
pub const SHT_STRTAB: Elf64Word = 3;
pub const SHT_RELA: Elf64Word = 4;

/* definitions for sh_flags */
pub const SHF_ALLOC: Elf64Xword = 1 << 1;
pub const SHF_EXECINSTR: Elf64Xword = 1 << 2;
pub const SHF_INFO_LINK: Elf64Xword = 1 << 6;

#[repr(C)]
pub struct Shdr64 {
    pub sh_name: Elf64Word,
    pub sh_type: Elf64Word,
    pub sh_flags: Elf64Xword,
    pub sh_addr: Elf64Addr,
    pub sh_offset: Elf64Off,
    pub sh_size: Elf64Xword,
    pub sh_link: Elf64Word,
    pub sh_info: Elf64Word,
    pub sh_addralign: Elf64Xword,
    pub sh_entsize: Elf64Xword,
}

impl Shdr64 {
    pub fn to_binary(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for byte in self.sh_name.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_type.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_flags.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_addr.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_offset.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_size.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_link.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_info.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_addralign.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.sh_entsize.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        bytes
    }
    pub fn size() -> usize {
        0x40
    }

    pub fn new_null_shdr() -> Self {
        Self {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }
    pub fn init_text_header(size: Elf64Xword) -> Self {
        Self {
            sh_name: 0,
            sh_type: SHT_PROGBITS,
            sh_flags: SHF_ALLOC | SHF_EXECINSTR,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: size,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 1,
            sh_entsize: 0,
        }
    }
    pub fn init_symtab_header(size: Elf64Xword) -> Self {
        Self {
            sh_name: 0,
            sh_type: SHT_SYMTAB,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: size,
            sh_link: 3, // .strtab が3番目にあることを決め打ち
            sh_info: 1, // グローバルシンボルが1番目にあることを決め打ち
            sh_addralign: 1,
            sh_entsize: Symbol64::size() as Elf64Xword,
        }
    }
    pub fn init_strtab_header(size: Elf64Xword) -> Self {
        Self {
            sh_name: 0,
            sh_type: SHT_STRTAB,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: size,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 1,
            sh_entsize: 0,
        }
    }
    pub fn init_relatext_header(size: Elf64Xword) -> Self {
        Self {
            sh_name: 0,
            sh_type: SHT_RELA,
            sh_flags: SHF_INFO_LINK,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: size,
            sh_link: 2, // シンボルテーブルが二番目にあることを決め打ち
            sh_info: 1, // .textセクションが一番目にあることを決め打ち
            sh_addralign: 8,
            sh_entsize: Rela64::size() as u64,
        }
    }
}
