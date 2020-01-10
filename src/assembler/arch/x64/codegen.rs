use crate::assembler::arch::x64::inst::X64InstName;
use crate::assembler::arch::x64::X64Assembler;

pub const REX_PREFIX_BASE: u8 = 0x40;
pub const REX_PREFIX_WBIT: u8 = 0x08;
pub const REX_PREFIX_RBIT: u8 = 0x04;
// pub const REX_PREFIX_XBIT: u8 = 0x02;
pub const REX_PREFIX_BBIT: u8 = 0x01;

pub const MODRM_REGISTER_REGISTER: u8 = 0xc0;
impl X64Assembler {
    pub fn codegen(&mut self) {
        for (_name, symbol) in self.src_file.symbols_map.iter_mut() {
            // コードの初期化
            let mut codes: Vec<u8> = Vec::new();

            // 各命令を機械語に変換
            for inst in symbol.insts.iter() {
                match &inst.name {
                    X64InstName::ADDRM64R64 => Self::generate_addrm64r64_inst(&mut codes, &inst),
                    X64InstName::ADDRM64IMM32 => {
                        Self::generate_addrm64imm32_inst(&mut codes, &inst)
                    }
                    X64InstName::MOVRM64R64 => Self::generate_movrm64r64_inst(&mut codes, &inst),
                    X64InstName::MOVRM64IMM32 => {
                        Self::generate_movrm64imm32_inst(&mut codes, &inst)
                    }
                    // X64InstName::MOV => ,
                    X64InstName::RET => Self::generate_ret_inst(&mut codes, &inst),
                    _ => (),
                }
            }

            // アラインメント調整
            let rest_bytes = codes.len() % 4;
            for _ in 0..(4 - rest_bytes) {
                codes.push(0x00);
            }

            // シンボルに格納
            symbol.codes = codes;
        }
    }
    pub fn rex_prefix_rbit(cond: bool) -> u8 {
        if cond {
            REX_PREFIX_RBIT
        } else {
            0
        }
    }
    pub fn rex_prefix_bbit(cond: bool) -> u8 {
        if cond {
            REX_PREFIX_BBIT
        } else {
            0
        }
    }
    pub fn modrm_reg_field(reg_number: usize) -> u8 {
        (reg_number as u8) << 3
    }
    pub fn modrm_rm_field(reg_number: usize) -> u8 {
        reg_number as u8
    }
}

#[cfg(test)]
mod codegen_tests {
    use super::*;
    use crate::assembler::arch::x64::lexer::lex_intel;
    use crate::assembler::arch::x64::X64AssemblyFile;
    use crate::structure::AssemblyFile;
    use crate::target::Target;

    #[test]
    fn test_codegen_with_add_calculus() {
        // 48 c7 c7 01 00 00 00    mov    rdi,0x1
        // 48 81 c7 02 00 00 00    add    rdi,0x2
        // 48 89 f8                mov    rax,rdi
        // c3                      ret
        let expected_codes: Vec<u8> = vec![
            0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00, 0x48, 0x81, 0xc7, 0x02, 0x00, 0x00, 0x00,
            0x48, 0x89, 0xf8, 0xc3, 0x00, 0x00,
        ];

        let mut assembler =
            preprocess("main:\n  mov rdi, 1\n  add rdi, 2\n  mov rax, rdi\n  ret\n");

        assembler.codegen();
        for (_name, symbol) in assembler.src_file.symbols_map.iter() {
            for (i, b) in symbol.codes.iter().enumerate() {
                assert_eq!(expected_codes[i], *b);
            }
        }
    }

    fn preprocess(input: &str) -> X64Assembler {
        let target = Target::new();
        let assembly_file = AssemblyFile::new_intel_file(input.to_string(), target);
        let x64_assembly_file = X64AssemblyFile::new(assembly_file);
        let mut assembler = X64Assembler::new(x64_assembly_file);

        lex_intel::lexing_intel_syntax(&mut assembler);
        assembler.parse_intel_syntax();
        assembler.analyze();
        assembler
    }
}
