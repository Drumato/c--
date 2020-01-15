use crate::compiler::backend::arch::x64::generate::Registers;
use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::ir::arch::x64::{
    basicblock::X64BasicBlock,
    function::X64Function,
    ir::X64IR,
    ir_kind::{X64IRKind, X64OpeKind},
};

impl X64Optimizer {
    pub fn generate_assembly_with_intel_syntax(&self) -> String {
        let mut output = String::new();
        // intel記法のprefix
        output += ".intel_syntax noprefix\n";

        output += &(format!(".global {}\n", self.entry_func.func_name).as_str());

        // Function本体
        output += &self.entry_func.to_intel_code();
        output
    }
}

impl X64Function {
    fn to_intel_code(&self) -> String {
        let mut output = String::new();
        output += &(format!("{}:\n", self.func_name).as_str());
        for block in self.blocks.iter() {
            output += &block.to_intel_code();
        }
        output
    }
}

impl X64BasicBlock {
    fn to_intel_code(&self) -> String {
        let mut output = String::new();
        if self.label != "entry" {
            output += &(format!("{}:\n", self.label).as_str());
        }
        for ir in self.irs.iter() {
            output += &(format!("  {}\n", ir.to_intel_code()).as_str());
        }
        output
    }
}

impl X64IR {
    fn to_intel_code(&self) -> String {
        match &self.kind {
            X64IRKind::MOV(dst, src) => {
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
            X64IRKind::SUB(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                match src.kind {
                    X64OpeKind::REG => {
                        let src_reg = Registers::from_number_ir(src.phys);
                        format!("sub {}, {}", dst_reg.to_string(), src_reg.to_string())
                    }
                    X64OpeKind::INTLIT(src_value) => {
                        format!("sub {}, {}", dst_reg.to_string(), src_value)
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
