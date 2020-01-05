use crate::compiler::backend::arch::x64::generate::Registers;
use crate::compiler::backend::arch::x64::X64Optimizer;
use crate::compiler::ir::arch::x64::*;

impl X64Optimizer {
    pub fn generate_assembly_with_intel_syntax(&self) -> String {
        let mut output = String::new();
        // intel記法のprefix
        output += ".intel_syntax noprefix\n";

        // TODO: 本来はちゃんとエントリポイントを打つ
        output += &(format!(".global {}\n", self.entry_bb.label).as_str());

        // BasicBlock本体
        output += &self.entry_bb.to_intel_code();
        output
    }
}

impl X64BasicBlock {
    fn to_intel_code(&self) -> String {
        let mut output = String::new();
        output += &(format!("{}:\n", self.label).as_str());
        for ir in self.irs.iter() {
            output += &(format!("  {}\n", ir.to_intel_code()).as_str());
        }
        output
    }
}

impl X64IR {
    fn to_intel_code(&self) -> String {
        // TODO: 本来はオペランドのサイズで命令を分ける必要がある
        match &self.kind {
            X64IRKind::MOV(dst, src) => {
                // TODO: まだ代入先がレジスタしかありえないので決め打ち
                let dst_reg = Registers::from_number_ir(dst.phys);
                match src.kind {
                    X64OpeKind::REG => {
                        let src_reg = Registers::from_number_ir(src.phys);
                        format!("mov {}, {}", dst_reg.to_string(), src_reg.to_string())
                    }
                    X64OpeKind::INTLIT(src_value) => {
                        format!("mov {}, {}", dst_reg.to_string(), src_value)
                    }
                    _ => {
                        eprintln!("can't emit with INVALID operand!");
                        String::new()
                    }
                }
            }
            X64IRKind::ADD(dst, src) => {
                // TODO: まだレジスタ同士の足し算しかありえないので決め打ち
                let dst_reg = Registers::from_number_ir(dst.phys);
                match src.kind {
                    X64OpeKind::REG => {
                        let src_reg = Registers::from_number_ir(src.phys);
                        format!("add {}, {}", dst_reg.to_string(), src_reg.to_string())
                    }
                    X64OpeKind::INTLIT(src_value) => {
                        format!("add {}, {}", dst_reg.to_string(), src_value)
                    }
                    _ => {
                        eprintln!("can't emit with INVALID operand!");
                        String::new()
                    }
                }
            }
            X64IRKind::RET(return_op) => match return_op.kind {
                X64OpeKind::REG => {
                    let return_reg = Registers::from_number_ir(return_op.phys);
                    format!("mov rax, {}\n  ret", return_reg.to_string())
                }
                X64OpeKind::INTLIT(return_value) => format!("mov rax, {}\n  ret", return_value),
                _ => {
                    eprintln!("can't emit with INVALID operand!");
                    String::new()
                }
            },
        }
    }
}
