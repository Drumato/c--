use crate::elf::elf64::*;
use phdr::Phdr64;
use shdr::Shdr64;

const ELFCLASS64: NIdentSize = 0x02;
const ELFDATA2LSB: NIdentSize = 0x01;
const ELFOSABI_SYSV: NIdentSize = 0x00;
const EV_CURRENT: NIdentSize = 0x01;

pub const ET_REL: Elf64Half = 0x01;
pub const ET_EXEC: Elf64Half = 0x02;
const EM_X86_64: Elf64Half = 0x3e;
#[repr(C)]
pub struct Ehdr64 {
    pub e_ident: NIdentSize,
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

impl Ehdr64 {
    pub fn new_for_object_file() -> Self {
        Self {
            e_ident: Self::magic_number()
                | Self::elf_class64()
                | Self::elf_lsb()
                | Self::current_version()
                | Self::elf_osabi_sysv(),
            e_type: ET_REL,
            e_machine: EM_X86_64,
            e_version: EV_CURRENT as Elf64Word,
            e_entry: 0x0,
            e_phoff: 0x0,
            e_shoff: 0x0,
            e_flags: 0x0,
            e_ehsize: Self::size() as Elf64Half,
            e_phentsize: Phdr64::size() as Elf64Half,
            e_phnum: 0x0,
            e_shentsize: Shdr64::size() as Elf64Half,
            e_shnum: 0x0,
            e_shstrndx: 0x0,
        }
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for byte in self.e_ident.to_be_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_type.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_machine.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_version.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_entry.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_phoff.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_shoff.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_flags.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_ehsize.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_phentsize.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_phnum.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_shentsize.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_shnum.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        for byte in self.e_shstrndx.to_le_bytes().to_vec() {
            bytes.push(byte);
        }
        bytes
    }
    pub fn size() -> u16 {
        0x40
    }
    fn magic_number() -> NIdentSize {
        0x7f454c46 << 96
    }
    fn elf_class64() -> NIdentSize {
        ELFCLASS64 << 88
    }
    fn elf_lsb() -> NIdentSize {
        ELFDATA2LSB << 80
    }
    fn current_version() -> NIdentSize {
        EV_CURRENT << 72
    }
    fn elf_osabi_sysv() -> NIdentSize {
        ELFOSABI_SYSV << 64
    }
}

#[cfg(test)]
mod ehdr_tests {
    use super::*;

    #[test]
    fn test_new_object_file() {
        let ehdr = Ehdr64::new_for_object_file();
        assert_eq!(0x7f454c46020101000000000000000000, ehdr.e_ident);
        assert_eq!(ET_REL, ehdr.e_type);
        assert_eq!(EM_X86_64, ehdr.e_machine);
        assert_eq!(EV_CURRENT as Elf64Word, ehdr.e_version);
        assert_eq!(0x00, ehdr.e_entry);
        assert_eq!(0x00, ehdr.e_phoff);
        assert_eq!(0x00, ehdr.e_shoff);
        assert_eq!(0x00, ehdr.e_flags);
        assert_eq!(0x40, ehdr.e_ehsize);
        assert_eq!(0x38, ehdr.e_phentsize);
        assert_eq!(0x00, ehdr.e_phnum);
        assert_eq!(0x40, ehdr.e_shentsize);
        assert_eq!(0x00, ehdr.e_shnum);
        assert_eq!(0x00, ehdr.e_shstrndx);
    }

    #[test]
    fn test_magic_number() {
        assert_eq!(0x7f454c46000000000000000000000000, Ehdr64::magic_number());
    }
    #[test]
    fn test_elf_class64() {
        assert_eq!(0x020000000000000000000000, Ehdr64::elf_class64());
    }
    #[test]
    fn test_elf_lsb() {
        assert_eq!(0x0100000000000000000000, Ehdr64::elf_lsb());
    }
    #[test]
    fn test_current_version() {
        assert_eq!(0x01000000000000000000, Ehdr64::current_version());
    }
    #[test]
    fn test_elf_osabi_sysv() {
        assert_eq!(0x00, Ehdr64::elf_osabi_sysv());
    }
}
