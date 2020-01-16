use crate::compiler::backend::arch::x64::generate::Registers;
use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::ir::arch::x64::{
    basicblock::X64BasicBlock, function::X64Function, ir::X64IR, ir_kind::X64IRKind,
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
            // add
            X64IRKind::ADDREGTOREG(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                format!("add {}, {}", dst_reg.to_string(), src_reg.to_string())
            }
            X64IRKind::ADDIMMTOREG(dst, immediate) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                format!("add {}, {}", dst_reg.to_string(), immediate.int_value())
            }

            // mov
            X64IRKind::MOVREGTOREG(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                format!("mov {}, {}", dst_reg.to_string(), src_reg.to_string())
            }
            X64IRKind::MOVIMMTOREG(dst, immediate) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                format!("mov {}, {}", dst_reg.to_string(), immediate.int_value())
            }

            // sub
            X64IRKind::SUBREGTOREG(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                format!("sub {}, {}", dst_reg.to_string(), src_reg.to_string())
            }
            X64IRKind::SUBIMMTOREG(dst, immediate) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                format!("sub {}, {}", dst_reg.to_string(), immediate.int_value())
            }

            // mul
            X64IRKind::MULREGTOREG(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                format!("imul {}, {}", dst_reg.to_string(), src_reg.to_string())
            }
            X64IRKind::MULIMMTOREG(dst, immediate) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                format!("imul {}, {}", dst_reg.to_string(), immediate.int_value())
            }

            // div
            X64IRKind::DIVREGTOREG(dst, src) => {
                let mut output = String::new();
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                output += &(format!("mov rax, {}\n", dst_reg.to_string()).as_str());
                output += "  cqo\n";
                output += &(format!("  idiv {}\n", src_reg.to_string()).as_str());
                output += &(format!("  mov {}, rax", dst_reg.to_string()).as_str());
                output
            }
            X64IRKind::DIVIMMTOREG(dst, immediate) => {
                let mut output = String::new();
                let dst_reg = Registers::from_number_ir(dst.phys);
                output += &(format!("mov rax, {}\n", dst_reg.to_string()).as_str());
                output += &(format!("  mov rcx, {}\n", immediate.int_value()).as_str());
                output += "  cqo\n";
                output += "  idiv rcx\n";
                output += &(format!("  mov {}, rax", dst_reg.to_string()).as_str());
                output
            }

            // ret
            X64IRKind::RETREG(return_op) => {
                let return_reg = Registers::from_number_ir(return_op.phys);
                format!("mov rax, {}\n  ret", return_reg.to_string())
            }
            X64IRKind::RETIMM(return_op) => {
                let return_value = return_op.int_value();
                format!("mov rax, {}\n  ret", return_value)
            }
            _ => {
                eprintln!("can't emit with invalid ir -> {:?}", self.kind);
                String::new()
            }
        }
    }
}
