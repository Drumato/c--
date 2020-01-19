use crate::compiler::backend::arch::x64::generate::Registers;
use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::ir::arch::x64::{
    basicblock::X64BasicBlock, function::X64Function, ir::X64IR, ir_kind::X64IRKind,
};

impl X64Optimizer {
    pub fn generate_assembly_with_at_and_t_syntax(&self) -> String {
        let mut output = String::new();
        output += &(format!(".global {}\n", self.entry_func.func_name).as_str());

        // Function本体
        output += &self.entry_func.to_at_and_t_code();
        output
    }
}

impl X64Function {
    fn to_at_and_t_code(&self) -> String {
        let mut output = String::new();
        output += &(format!("{}:\n", self.func_name).as_str());
        for block in self.blocks.iter() {
            output += &block.to_at_and_t_code();
        }
        output
    }
}

impl X64BasicBlock {
    fn to_at_and_t_code(&self) -> String {
        let mut output = String::new();
        if self.label != "entry" {
            output += &(format!("{}:\n", self.label).as_str());
        }
        for ir in self.irs.iter() {
            output += &(format!("  {}\n", ir.to_at_and_t_code()).as_str());
        }
        output
    }
}

impl X64IR {
    fn to_at_and_t_code(&self) -> String {
        match &self.kind {
            // add
            X64IRKind::ADDREGTOREG(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                format!("addq %{}, %{}", src_reg.to_string(), dst_reg.to_string())
            }
            X64IRKind::ADDIMMTOREG(dst, immediate) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                format!("addq ${}, %{}", immediate.int_value(), dst_reg.to_string())
            }

            // mov
            X64IRKind::MOVREGTOREG(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                format!("movq %{}, %{}", src_reg.to_string(), dst_reg.to_string())
            }
            X64IRKind::MOVIMMTOREG(dst, immediate) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                format!("movq ${}, %{}", immediate.int_value(), dst_reg.to_string())
            }

            // sub
            X64IRKind::SUBREGTOREG(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                format!("subq %{}, %{}", src_reg.to_string(), dst_reg.to_string())
            }
            X64IRKind::SUBIMMTOREG(dst, immediate) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                format!("subq ${}, %{}", immediate.int_value(), dst_reg.to_string())
            }

            // mul
            X64IRKind::MULREGTOREG(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                format!("imulq %{}, %{}", src_reg.to_string(), dst_reg.to_string())
            }
            X64IRKind::MULIMMTOREG(dst, immediate) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                format!("imulq ${}, %{}", immediate.int_value(), dst_reg.to_string())
            }

            // div
            X64IRKind::DIVREGTOREG(dst, src) => {
                let mut output = String::new();
                let dst_reg = Registers::from_number_ir(dst.phys);
                let src_reg = Registers::from_number_ir(src.phys);
                output += &(format!("movq %{}, %rax\n", dst_reg.to_string()).as_str());
                output += "  cltd\n";
                output += &(format!("  idivq %{}\n", src_reg.to_string()).as_str());
                output += &(format!("  movq %rax, %{}", dst_reg.to_string()).as_str());
                output
            }
            X64IRKind::DIVIMMTOREG(dst, immediate) => {
                let mut output = String::new();
                let dst_reg = Registers::from_number_ir(dst.phys);
                output += &(format!("movq %{}, %rax\n", dst_reg.to_string()).as_str());
                output += &(format!("  movq ${}, %rcx\n", immediate.int_value()).as_str());
                output += "  cltd\n";
                output += "  idivq %rcx\n";
                output += &(format!("  movq %rax, %{}", dst_reg.to_string()).as_str());
                output
            }

            // ret
            X64IRKind::RETREG(return_op) => {
                let return_reg = Registers::from_number_ir(return_op.phys);
                format!("movq %{}, %rax\n  ret", return_reg.to_string())
            }
            X64IRKind::RETIMM(return_op) => {
                let return_value = return_op.int_value();
                format!("movq ${}, %rax\n  ret", return_value)
            }
            X64IRKind::JMP(label_name) => format!("jmp {}", label_name),
            _ => {
                eprintln!("can't emit with invalid ir -> {:?}", self.kind);
                String::new()
            }
        }
    }
}
