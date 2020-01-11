use crate::compiler::backend::arch::x64::generate::Registers;
use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::ir::arch::x64::*;

impl X64Optimizer {
    pub fn generate_assembly_with_at_and_t_syntax(&self) -> String {
        let mut output = String::new();
        output += &(format!(".global {}\n", self.entry_bb.label).as_str());

        // BasicBlock本体
        output += &self.entry_bb.to_at_and_t_code();
        output
    }
}

impl X64BasicBlock {
    fn to_at_and_t_code(&self) -> String {
        let mut output = String::new();
        output += &(format!("{}:\n", self.label).as_str());
        for ir in self.irs.iter() {
            output += &(format!("  {}\n", ir.to_at_and_t_code()).as_str());
        }
        output
    }
}

impl X64IR {
    fn to_at_and_t_code(&self) -> String {
        match &self.kind {
            X64IRKind::MOV(dst, src) => {
                let dst_reg = Registers::from_number_ir(dst.phys);
                match src.kind {
                    X64OpeKind::REG => {
                        let src_reg = Registers::from_number_ir(src.phys);
                        format!("movq %{}, %{}", src_reg.to_string(), dst_reg.to_string())
                    }
                    X64OpeKind::INTLIT(src_value) => {
                        format!("movq ${}, %{}", src_value, dst_reg.to_string())
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
                        format!("addq %{}, %{}", src_reg.to_string(), dst_reg.to_string())
                    }
                    X64OpeKind::INTLIT(src_value) => {
                        format!("addq ${}, %{}", src_value, dst_reg.to_string())
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
                    format!("movq %{}, %rax\n  ret", return_reg.to_string())
                }
                X64OpeKind::INTLIT(return_value) => format!("movl ${}, %rax\n  ret", return_value),
                _ => {
                    eprintln!("can't emit with INVALID operand!");
                    String::new()
                }
            },
        }
    }
}
