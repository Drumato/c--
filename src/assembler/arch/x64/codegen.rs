use crate::assembler::arch::x64::assembler::X64Assembler;
use crate::assembler::arch::x64::inst::{inst_kind::X64InstKind, inst_name::X64InstName};

use std::collections::BTreeMap;

pub const REX_PREFIX_BASE: u8 = 0x40;
pub const REX_PREFIX_WBIT: u8 = 0x08;
pub const REX_PREFIX_RBIT: u8 = 0x04;
// pub const REX_PREFIX_XBIT: u8 = 0x02;
pub const REX_PREFIX_BBIT: u8 = 0x01;

pub const MODRM_REGISTER_REGISTER: u8 = 0xc0;
pub const MODRM_REGISTER_DISPLACEMENT8: u8 = 0x40;
impl X64Assembler {
    pub fn codegen(&mut self) {
        // BTreeMap<LabelName, (CodeIndex, Offset)>
        let mut jump_map: BTreeMap<String, (usize, usize)> = BTreeMap::new();
        for (_name, symbol) in self.src_file.symbols_map.iter_mut() {
            // コードの初期化
            let mut codes: Vec<u8> = Vec::new();

            // 各命令を機械語に変換
            for inst in symbol.insts.iter() {
                match &inst.name {
                    X64InstName::LABEL => {
                        if let X64InstKind::LABEL(name) = &inst.kind {
                            // jump系命令がラベルの前に存在した場合
                            if let Some(tup) = jump_map.get_mut(name) {
                                // jump系命令の位置 - 現在位置 - 1 => 相対オフセット
                                tup.1 = codes.len() - tup.1 - 1;
                                continue;
                            }

                            // ラベルがjump系命令の前に存在した場合
                            jump_map.insert(name.to_string(), (0, codes.len()));
                        }
                    }
                    X64InstName::JZREL32 => {
                        // opcode
                        codes.push(0x0f);
                        codes.push(0x84);
                        if let X64InstKind::UNARY(op) = &inst.kind {
                            let label_name = op.label_name();

                            if let Some(tup) = jump_map.get_mut(&label_name) {
                                // ラベルがjump系命令の前に存在した場合
                                tup.0 = codes.len();
                                tup.1 = !(codes.len() + 4 - tup.1) + 1;
                            } else {
                                // jump系命令がラベルの前に存在した場合
                                jump_map
                                    .insert(label_name.to_string(), (codes.len(), codes.len() + 3));
                            }
                        }

                        // immediate-value
                        for b in (0x00 as u32).to_le_bytes().to_vec().iter() {
                            codes.push(*b);
                        }
                    }
                    X64InstName::JMPREL32 => {
                        // opcode
                        codes.push(0xe9);
                        if let X64InstKind::UNARY(op) = &inst.kind {
                            let label_name = op.label_name();

                            if let Some(tup) = jump_map.get_mut(&label_name) {
                                // ラベルがjump系命令の前に存在した場合
                                tup.0 = codes.len();
                                tup.1 = !(codes.len() + 4 - tup.1) + 1;
                            } else {
                                // jump系命令がラベルの前に存在した場合
                                jump_map
                                    .insert(label_name.to_string(), (codes.len(), codes.len() + 3));
                            }
                        }

                        // immediate-value
                        for b in (0x00 as u32).to_le_bytes().to_vec().iter() {
                            codes.push(*b);
                        }
                    }
                    // add
                    X64InstName::ADDRM64R64 => Self::generate_addrm64r64_inst(&mut codes, &inst),
                    X64InstName::ADDRM64IMM32 => {
                        Self::generate_addrm64imm32_inst(&mut codes, &inst)
                    }
                    // call
                    X64InstName::CALLRM64 => {
                        /* ただのcallではなく,raxにアドレス即値をmovしてからcallするやつに変換 */
                        // mov-rex-prefix
                        codes.push(0x48);

                        // mov-opcode
                        codes.push(0xc7);

                        // mov-modrm
                        codes.push(0xc0);
                        if let X64InstKind::UNARY(op) = &inst.kind {
                            let label_name = op.label_name();
                            if let Some(rela) = self.src_file.relocations_map.get_mut(&label_name) {
                                rela.r_offset = self.all_bytes + codes.len() as u64;
                            }
                        }
                        //  TODO: 関数実装したときに追加
                        //  if let None = self.src_file.symbols_map.get(label_name) {
                        //     self.src_file.symbols_map.insert(label_name, new_global_symbol());
                        //  }
                        // mov-immediate-value
                        for b in (0 as u32).to_le_bytes().to_vec().iter() {
                            codes.push(*b);
                        }

                        Self::generate_callrm64_inst(&mut codes, &inst);
                    }
                    X64InstName::MOVRM64R64 => Self::generate_movrm64r64_inst(&mut codes, &inst),
                    X64InstName::MOVRM64IMM32 => {
                        Self::generate_movrm64imm32_inst(&mut codes, &inst)
                    }
                    X64InstName::MOVR64RM64 => Self::generate_movr64rm64_inst(&mut codes, &inst),
                    X64InstName::IMULR64RM64IMM32 => {
                        Self::generate_imulr64rm64imm32_inst(&mut codes, &inst)
                    }
                    X64InstName::IMULR64RM64 => Self::generate_imulr64rm64_inst(&mut codes, &inst),
                    X64InstName::IDIVRM64 => Self::generate_idivrm64_inst(&mut codes, &inst),
                    X64InstName::SUBRM64R64 => Self::generate_subrm64r64_inst(&mut codes, &inst),
                    X64InstName::SUBRM64IMM32 => {
                        Self::generate_subrm64imm32_inst(&mut codes, &inst)
                    }
                    X64InstName::CMPRM64R64 => Self::generate_cmprm64r64_inst(&mut codes, &inst),
                    X64InstName::CMPRM64IMM32 => {
                        Self::generate_cmprm64imm32_inst(&mut codes, &inst)
                    }
                    X64InstName::CQO => Self::generate_cqo_inst(&mut codes, &inst),
                    X64InstName::RET => Self::generate_ret_inst(&mut codes, &inst),
                    X64InstName::SYSCALL => Self::generate_syscall_inst(&mut codes),
                    X64InstName::PUSHR64 => Self::generate_pushr64_inst(&mut codes, &inst),
                    X64InstName::POPR64 => Self::generate_popr64_inst(&mut codes, &inst),
                    X64InstName::NEGRM64 => Self::generate_negrm64_inst(&mut codes, &inst),
                    _ => {
                        eprintln!("not generate ... {:?}", inst.name);
                    }
                }
            }

            // ジャンプ系命令のオフセットを解決する
            for inst in symbol.insts.iter() {
                match &inst.name {
                    X64InstName::JMPREL32 => {
                        if let X64InstKind::UNARY(op) = &inst.kind {
                            let label_name = op.label_name();

                            if let Some(tup) = jump_map.get(&label_name) {
                                for (idx, b) in (tup.1 as u32).to_le_bytes().iter().enumerate() {
                                    codes[idx + tup.0] = *b;
                                }
                            }
                        }
                    }
                    X64InstName::JZREL32 => {
                        if let X64InstKind::UNARY(op) = &inst.kind {
                            let label_name = op.label_name();

                            if let Some(tup) = jump_map.get(&label_name) {
                                for (idx, b) in (tup.1 as u32).to_le_bytes().iter().enumerate() {
                                    codes[idx + tup.0] = *b;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }

            // アラインメント調整
            let rest_bytes = codes.len() % 4;
            for _ in 0..(4 - rest_bytes) {
                codes.push(0x00);
            }

            // シンボルに格納
            self.all_bytes += codes.len() as u64;
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
    use crate::assembler::arch::x64::file::X64AssemblyFile;
    use crate::assembler::arch::x64::lexer::lex_intel;
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
                eprintln!("{} 0x{:x} = 0x{:x}", i, expected_codes[i], *b);
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
