use crate::assembler::arch::x64::X64Assembler;
use crate::elf::elf64::ehdr::Ehdr64;
use crate::elf::elf64::shdr::Shdr64;
use crate::elf::elf64::symbol::Symbol64;
use crate::elf::elf64::*;

impl ELF64 {
    pub fn add_text_section_x64(&mut self, assembler: &X64Assembler) {
        // 全ての機械語を一つのVectorに統合させる.
        // それがそのまま.textセクションの中身になる.
        let mut total_machine_code: Vec<u8> = Vec::new();
        for (_name, symbol) in assembler.src_file.symbols_map.iter() {
            let mut src_codes = symbol.codes.clone();
            total_machine_code.append(&mut src_codes);
        }

        // .textセクションヘッダの作成
        let text_header = Shdr64::init_text_header(total_machine_code.len() as Elf64Xword);
        self.add_section(total_machine_code, text_header, ".text");
    }
    pub fn add_symtab_section_x64(&mut self, assembler: &X64Assembler) {
        // 必ずnullシンボルを含む
        let mut symbols: Vec<Symbol64> = vec![Symbol64::new_null_symbol()];

        // シンボルを走査する
        // name_indexの操作も行う.
        // また,各シンボルのオフセットも計算する.
        let mut symbol_name_index: Elf64Word = 1; // 最初のnull文字を飛ばす
        let mut symbol_offset: Elf64Addr = 0; // st_value用

        // 注意! symbols_mapからイテレーションしたときの順番が一意であることを決め打ち.
        for (symbol_name, asm_symbol) in assembler.src_file.symbols_map.iter() {
            // 後々実体を参照するだけのasm_symbolが存在する為このifを用いる
            if asm_symbol.is_defined() {
                let symbol_length = asm_symbol.codes.len() as Elf64Xword;

                let defined_symbol =
                    Symbol64::new_global_function(symbol_name_index, symbol_length, symbol_offset);

                symbols.push(defined_symbol);
            } else {
                // let reference_symbol = Symbol64::init_reference_symbol();
            }

            // シンボル名を指すインデックスの更新( null byte を見越して+1する)
            symbol_name_index += symbol_name.len() as Elf64Word + 1;

            // オフセットの更新
            // 後ろのシンボルのオフセット <- 前のシンボルのサイズの総合値
            symbol_offset += asm_symbol.codes.len() as Elf64Addr;
        }

        // Vec<Symbol64> をバイナリ列に変換する
        let mut symbol_tables: Vec<u8> = Vec::new();
        for symbol in symbols.iter() {
            let mut symbol_entry = symbol.to_binary();
            symbol_tables.append(&mut symbol_entry);
        }

        let symtab_header = Shdr64::init_symtab_header(symbol_tables.len() as Elf64Xword);
        self.add_section(symbol_tables, symtab_header, ".symtab");
    }
    pub fn add_strtab_section_x64(&mut self, assembler: &X64Assembler) {
        // シンボルマップをイテレートして,名前を集める.
        let symbol_names: Vec<&str> = assembler
            .src_file
            .symbols_map
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<&str>>();

        let symbol_string_table = Self::build_strtab_from_names(symbol_names);
        let symbol_strtab_header =
            Shdr64::init_strtab_header(symbol_string_table.len() as Elf64Xword);

        self.add_section(symbol_string_table, symbol_strtab_header, ".strtab");
    }
    pub fn add_shstrtab_section_x64(&mut self, section_names: Vec<&str>) {
        let section_string_table = Self::build_strtab_from_names(section_names);
        let section_strtab_header =
            Shdr64::init_strtab_header(section_string_table.len() as Elf64Xword);
        self.add_section(section_string_table, section_strtab_header, ".shstrtab");
    }

    // セクション文字列のインデックスなど,全てを整理する.
    pub fn finalize(&mut self) {
        // ファイル先頭からのオフセット
        let mut file_offset = Ehdr64::size() as u64;
        // 各セクションのオフセットを設定
        for section in self.sections.iter_mut() {
            section.header.sh_offset += file_offset;
            file_offset += section.header.sh_size;
        }

        // SectionHeaderTable のスタートの設定 -> Ehdr + 全セクションサイズ
        let start: u64 = 0x40;
        self.ehdr.e_shoff = self
            .sections
            .iter()
            .fold(start, |sum, section| sum + section.bytes.len() as u64);

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
    fn build_strtab_from_names(names: Vec<&str>) -> Vec<u8> {
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

#[cfg(test)]
mod x64_elf_tests {
    use super::*;
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::assembler::arch::x64::X64AssemblyFile;
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_add_null_section() {
        let mut test_elf = ELF64::new_object_file();

        assert_eq!(test_elf.sections.len(), 0);

        test_elf.add_null_section();

        assert_eq!(test_elf.sections.len(), 1);
    }

    #[test]
    fn test_add_text_section_x64() {
        let assembler = preprocess(".global main\nmain:\n  mov rax, 30\n  ret\n");
        let mut test_elf = ELF64::new_object_file();

        assert_eq!(test_elf.sections.len(), 0);
        test_elf.add_text_section_x64(&assembler);
        assert_eq!(test_elf.sections.len(), 1);

        let test_section = &test_elf.sections[0];

        assert_eq!(test_section.bytes.len() % 4, 0);
    }

    fn preprocess(input: &str) -> X64Assembler {
        let target = Target::new();
        let assembly_file = AssemblyFile::new_intel_file(input.to_string(), target);
        let x64_assembly_file = X64AssemblyFile::new(assembly_file);
        let mut assembler = X64Assembler::new(x64_assembly_file);

        lex_intel::lexing_intel_syntax(&mut assembler);
        assembler.parse_intel_syntax();
        assembler.analyze();
        assembler.codegen();
        assembler
    }
}
