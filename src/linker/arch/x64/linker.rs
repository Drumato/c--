use crate::elf::elf64;
use elf64::{ehdr, phdr, rela, symbol};

pub static BASE_ADDRESS: u64 = 0x400000;
pub static PAGE_SIZE: u64 = 0x1000;

pub struct X64StaticLinker {
    pub exec_file: elf64::ELF64,
}

impl X64StaticLinker {
    pub fn new(obj_file: elf64::ELF64) -> Self {
        Self {
            exec_file: obj_file,
        }
    }
    pub fn link(&mut self) {
        // .textセクションだけをまとめたセグメントを作る
        self.init_phdr();
        self.prepare_ehdr_for_staticlink();

        // ページサイズアラインの為にパディング
        // これはgccもやっている方法.
        self.padding_null_byte_to_null_section();

        // 実際のリンク
        self.link_symbols();
        self.resolve_symbols();

        // 調整をかける
        self.conditioning_section_offset();
    }
    // 定義済みシンボルにアドレスを割り当てる
    fn link_symbols(&mut self) {
        // シンボル名検索用
        let strtab: Vec<u8> = self.exec_file.get_section_binary(".strtab");

        // 扱いやすくするため構造体列に変換
        let mut symbols: Vec<symbol::Symbol64> = self.exec_file.get_symbol_table();
        for symbol in symbols.iter_mut() {
            // スタートアップルーチンであればエントリポイントに指定
            if strtab[symbol.st_name as usize] as char == '_' {
                self.exec_file.ehdr.e_entry = BASE_ADDRESS + symbol.st_value;
            }

            // ファイルオフセット + 決め打ちアドレス
            symbol.st_value += BASE_ADDRESS;
        }

        // バイナリに再変換して格納
        let symtab_number: usize = self.exec_file.get_section_number(".symtab");
        self.exec_file.sections[symtab_number].bytes = symbol::Symbol64::symbols_to_binary(symbols);
    }

    // 再配置テーブル等を利用したリンク
    fn resolve_symbols(&mut self) {
        let symbols: Vec<symbol::Symbol64> = self.exec_file.get_symbol_table();
        let mut relas: Vec<rela::Rela64> = self.exec_file.get_reloc_table(".rela.text");

        for rel in relas.iter_mut() {
            // Relaオブジェクトに対応するシンボルテーブルエントリからアドレスを取り出す
            let symbol_table_entry_index = rela::Rela64::bind(rel.r_info);
            let address = symbols[symbol_table_entry_index].st_value as u32;

            // アドレスをバイト列に変換,機械語に書き込むことでアドレス解決
            for (idx, b) in address.to_le_bytes().to_vec().iter().enumerate() {
                let text_number: usize = self.exec_file.get_section_number(".text");
                self.exec_file.sections[text_number].bytes[rel.r_offset as usize + idx] = *b;
            }
        }
    }

    fn init_phdr(&mut self) {
        let mut phdr: phdr::Phdr64 = phdr::Phdr64::new();
        // 機械語命令 -> PT_LOADに配置
        phdr.p_type = phdr::PT_LOAD;
        // Linux環境ではページサイズアラインされている必要あり
        phdr.p_offset = PAGE_SIZE;
        phdr.p_align = PAGE_SIZE;

        // 決め打ちしたアドレスにロード
        phdr.p_vaddr = BASE_ADDRESS;
        phdr.p_paddr = BASE_ADDRESS;

        let text: Vec<u8> = self.exec_file.get_section_binary(".text");

        // .bssではないので filesz/memsz は同じ
        phdr.p_filesz = text.len() as u64; // remove the hardcode
        phdr.p_memsz = text.len() as u64; // remove the hardcode

        // 全フラグを立てておく
        phdr.p_flags = phdr::PF_R | phdr::PF_X | phdr::PF_W;
        self.exec_file.phdrs.push(phdr);
    }
    fn prepare_ehdr_for_staticlink(&mut self) {
        // スタティックリンク -> ET_EXEC
        self.exec_file.ehdr.e_type = ehdr::ET_EXEC;

        // program header tableはEhdrのすぐ後ろ
        self.exec_file.ehdr.e_phoff = ehdr::Ehdr64::size() as u64;

        self.exec_file.ehdr.e_phnum = self.exec_file.phdrs.len() as u16;
        self.exec_file.ehdr.e_phentsize = phdr::Phdr64::size() as u16;

        // パディング後のオフセットを渡す
        let all_section_size = self.exec_file.sum_given_and_section_sizes(0);
        self.exec_file.ehdr.e_shoff = PAGE_SIZE + all_section_size;
    }
    fn padding_null_byte_to_null_section(&mut self) {
        // 0x00を nullセクションに書き込む
        // null-section-header の値は変えないので,どのセクションにも属さないバイナリを作る
        let ph_table_size = phdr::Phdr64::size() as u64 * self.exec_file.phdrs.len() as u64;
        for _ in 0..PAGE_SIZE - ehdr::Ehdr64::size() as u64 - ph_table_size {
            self.exec_file.sections[0].bytes.push(0x00);
        }
    }
    fn conditioning_section_offset(&mut self) {
        // 各セクションオフセットを調整
        // パディング後のオフセットに合わせる.
        let _ = self
            .exec_file
            .sections
            .iter_mut()
            .map(|section| {
                section.header.sh_offset =
                    PAGE_SIZE - ehdr::Ehdr64::size() as u64 + section.header.sh_offset;
            })
            .collect::<()>();

        // .textセクションのアドレスをエントリポイントになおしておく.
        let text_number: usize = self.exec_file.get_section_number(".text");
        self.exec_file.sections[text_number].header.sh_addr = self.exec_file.ehdr.e_entry;
    }
}
