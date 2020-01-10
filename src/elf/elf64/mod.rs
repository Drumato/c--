pub mod ehdr;
pub mod phdr;
pub mod section;
pub mod shdr;
pub mod symbol;

/* Type for a 16-bit quantity.  */
pub type Elf64Half = u16;

/* Types for signed and unsigned 32-bit quantities.  */
pub type Elf64Word = u32;
#[allow(dead_code)]
pub type Elf64Sword = i32;

/* Types for signed and unsigned 64-bit quantities.  */
pub type Elf64Xword = u64;
#[allow(dead_code)]
pub type Elf64Sxword = i64;

/* Type of addresses.  */
pub type Elf64Addr = u64;

/* Type of file offsets.  */
pub type Elf64Off = u64;

/* Type for section indices, which are 16-bit quantities.  */
pub type Elf64Section = u16;

/* Type for version symbol information.  */
#[allow(dead_code)]
pub type Elf64Versym = Elf64Half;

pub type NIdentSize = u128;

#[repr(C)]
pub struct ELF64 {
    pub ehdr: ehdr::Ehdr64,
    pub sections: Vec<section::Section64>,
    // phdrs: Vec<Phdr64>,
}

impl ELF64 {
    pub fn new_object_file() -> Self {
        Self {
            ehdr: ehdr::Ehdr64::new_for_object_file(),
            sections: Vec::new(),
            // phdrs: Vec::new(),
        }
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = Vec::new();
        // ehdr bytes
        let mut ehdr_binary = self.ehdr.to_binary();
        binary.append(&mut ehdr_binary);

        // each section bytes
        for section in self.sections.iter() {
            let mut section_binary = section.bytes.clone();
            binary.append(&mut section_binary);
        }

        // each section header table
        for section in self.sections.iter() {
            let mut shdr_binary = section.header.to_binary();
            binary.append(&mut shdr_binary);
        }

        binary
    }
    pub fn add_null_section(&mut self) {
        self.sections.push(section::Section64::new_null_section());
    }
    pub fn add_section(&mut self, bytes: Vec<u8>, header: shdr::Shdr64, name: &str) {
        let section = section::Section64::new(name.to_string(), header, bytes);
        self.sections.push(section);
    }
}
