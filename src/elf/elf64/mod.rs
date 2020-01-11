pub mod ehdr;
pub mod phdr;
pub mod rela;
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
    pub phdrs: Vec<phdr::Phdr64>,
}

impl ELF64 {
    pub fn new_object_file() -> Self {
        Self {
            ehdr: ehdr::Ehdr64::new_for_object_file(),
            sections: Vec::new(),
            phdrs: Vec::new(),
        }
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = Vec::new();
        // ehdr bytes
        let mut ehdr_binary = self.ehdr.to_binary();
        binary.append(&mut ehdr_binary);

        // program header table
        for phdr in self.phdrs.iter() {
            let mut phdr_binary = phdr.to_binary();
            binary.append(&mut phdr_binary);
        }

        // each section bytes
        for section in self.sections.iter() {
            let mut section_binary = section.bytes.clone();
            binary.append(&mut section_binary);
        }

        // section header table
        for section in self.sections.iter() {
            let mut shdr_binary = section.header.to_binary();
            binary.append(&mut shdr_binary);
        }

        binary
    }
    pub fn get_section_binary(&self, name: &str) -> Vec<u8> {
        for section in self.sections.iter() {
            if section.name == name {
                return section.bytes.clone();
            }
        }
        eprintln!("{} not found!", name);
        Vec::new()
    }
    pub fn get_section_number(&self, name: &str) -> usize {
        for (i, section) in self.sections.iter().enumerate() {
            if section.name == name {
                return i;
            }
        }
        eprintln!("{} not found!", name);
        42
    }
    pub fn get_symbol_table(&self) -> Vec<symbol::Symbol64> {
        let one_sym_size = symbol::Symbol64::size();
        let symbol_table_binary: Vec<u8> = self.get_section_binary(".symtab");
        let symbol_count: usize = symbol_table_binary.len() / one_sym_size;
        let mut symbols: Vec<symbol::Symbol64> = vec![symbol::Symbol64::new_null_symbol()];

        // nullシンボルを取り除いている
        for i in 0..symbol_count - 1 {
            let symbol_table_entry = symbol::Symbol64::new_unsafe(
                symbol_table_binary[(i + 1) * one_sym_size..].to_vec(),
            );
            symbols.push(symbol_table_entry);
        }
        symbols
    }
    pub fn get_reloc_table(&self, name: &str) -> Vec<rela::Rela64> {
        let one_rel_size = rela::Rela64::size();
        let reloc_table_binary: Vec<u8> = self.get_section_binary(name);
        let reloc_count: usize = reloc_table_binary.len() / one_rel_size;
        let mut relocs: Vec<rela::Rela64> = Vec::new();

        // nullシンボルを取り除いている
        for i in 0..reloc_count {
            let reloc_table_entry =
                rela::Rela64::new_unsafe(reloc_table_binary[i * one_rel_size..].to_vec());
            relocs.push(reloc_table_entry);
        }
        relocs
    }
    pub fn add_null_section(&mut self) {
        self.sections.push(section::Section64::new_null_section());
    }
    pub fn add_section(&mut self, bytes: Vec<u8>, header: shdr::Shdr64, name: &str) {
        let section = section::Section64::new(name.to_string(), header, bytes);
        self.sections.push(section);
    }

    // セクション文字列のインデックスなど,全てを整理する.
    pub fn finalize(&mut self) {
        // ファイル先頭からのオフセット
        let file_offset = ehdr::Ehdr64::size() as u64;
        // 各セクションのオフセットを設定
        self.clean_sections_offset(file_offset);

        // SectionHeaderTable のスタートの設定 -> Ehdr + 全セクションサイズ
        // TODO: スタティックリンクしたバイナリは 0x1000 から
        self.ehdr.e_shoff = self.sum_given_and_section_sizes(ehdr::Ehdr64::size() as u64);

        // セクション数,shstrtabの位置の設定
        // TODO: `.shstrtab` 名のインデックスを返す関数で対応したほうが良い
        self.ehdr.e_shnum = self.sections.len() as u16;
        self.ehdr.e_shstrndx = self.ehdr.e_shnum - 1;

        // セクション名を揃える
        let name_count = self.sections[self.ehdr.e_shstrndx as usize]
            .bytes
            .iter()
            .filter(|num| *num == &0x00)
            .collect::<Vec<&u8>>()
            .len()
            - 1;
        let mut sh_name = 1;
        for (idx, bb) in self.sections[self.ehdr.e_shstrndx as usize]
            .bytes
            .to_vec()
            .splitn(name_count, |num| *num == 0x00)
            .enumerate()
        {
            if idx == 0 || idx >= self.ehdr.e_shnum as usize {
                continue;
            }
            let b: Vec<&u8> = bb
                .iter()
                .take_while(|num| *num != &0x00)
                .collect::<Vec<&u8>>();
            self.sections[idx].header.sh_name = sh_name as u32;
            sh_name += b.len() as u32 + 1;
        }
    }
    pub fn clean_sections_offset(&mut self, mut base: u64) {
        for section in self.sections.iter_mut() {
            section.header.sh_offset += base;
            base += section.header.sh_size;
        }
    }
    pub fn sum_given_and_section_sizes(&mut self, base: u64) -> u64 {
        self.sections
            .iter()
            .fold(base, |sum, section| sum + section.bytes.len() as u64)
    }
    pub fn build_strtab_from_names(names: Vec<&str>) -> Vec<u8> {
        // ELFの文字列テーブルは null-byte + (name + null-byte) * n という形状に
        // それに合うようにバイト列を構築.
        let mut string_table: Vec<u8> = vec![0x00];
        for name in names {
            for byte in name.as_bytes() {
                string_table.push(*byte);
            }
            string_table.push(0x00);
        }

        // アラインメントの調整
        let md = string_table.len() % 4;
        for _ in 0..(4 - md) {
            string_table.push(0x00);
        }

        string_table
    }
}
