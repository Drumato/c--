use crate::assembler::arch::x64::asmtoken;
use crate::assembler::arch::x64::file::X64AssemblyFile;

pub struct X64Assembler {
    pub src_file: X64AssemblyFile,
    // アセンブリコードを字句解析してここに格納
    pub tokens: Vec<asmtoken::AsmToken>,

    // パース処理用
    pub cur_token: usize,
    pub next_token: usize,

    // コード生成用
    pub all_bytes: u64,
}

impl X64Assembler {
    pub fn new(file: X64AssemblyFile) -> Self {
        Self {
            src_file: file,
            tokens: Vec::new(),
            cur_token: 0,
            next_token: 1,
            all_bytes: 0,
        }
    }
    pub fn dump_instructions_to_stderr(&self) {
        for (symbol_name, symbol_info) in self.src_file.symbols_map.iter() {
            eprintln!("{}'s instructions:", symbol_name);
            for inst in symbol_info.insts.iter() {
                eprintln!("\t{}", inst.to_string());
            }
        }
    }
    pub fn setup_relocations(&mut self) {
        for (sym_idx, (sym_name, _symbol)) in self.src_file.symbols_map.iter().enumerate() {
            if let Some(rela) = self.src_file.relocations_map.get_mut(sym_name) {
                rela.r_info = (((sym_idx + 1) << 32) + 1) as u64;
            }
        }
    }
}
