use crate::compiler::backend::arch::x64::generate::Registers;
use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::ir::arch::x64::{
    basicblock::X64BasicBlock, function::X64Function, ir::X64IR, ir_kind::X64IRKind,
};

impl X64Optimizer {
    pub fn generate_assembly_with_intel_syntax(&self) -> String {
        let mut output = self.generate_intel_prefix_and_directive();

        // Function本体
        for func in self.functions.iter() {
            output += &func.to_intel_code();
        }
        output
    }

    pub fn generate_intel_prefix_and_directive(&self) -> String {
        let mut output = String::new();
        // intel記法のprefix
        output += ".intel_syntax noprefix\n";
        for func in self.functions.iter() {
            output += &(format!(".global {}\n", func.func_name).as_str());
        }
        output
    }
}

impl X64Function {
    fn to_intel_code(&self) -> String {
        let mut output = String::new();
        output += &(format!("{}:\n", self.func_name).as_str());

        // 関数プロローグ
        output += &(format!("  push rbp\n").as_str());
        output += &(format!("  mov rbp, rsp\n").as_str());
        if self.frame_size != 0 {
            output += &(format!("  sub rsp, {}\n", !7 & self.frame_size + 7).as_str());
        }

        // 関数本体
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
            X64IRKind::MOVMEMTOREG(dst, var) => {
                let src_name = var.var_name();
                let src_off = var.var_offset();
                let dst_reg = Registers::from_number_ir(dst.phys);

                format!(
                    "mov {}, -{}[rbp] # {}",
                    dst_reg.to_string(),
                    src_off,
                    src_name
                )
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

            // store
            X64IRKind::STOREREG(dst, src) => {
                let src_reg = Registers::from_number_ir(src.phys);
                let dst_name = dst.var_name();
                let dst_offset = dst.var_offset();
                format!(
                    "mov -{}[rbp], {} # {}",
                    dst_offset,
                    src_reg.to_string(),
                    dst_name
                )
            }
            X64IRKind::STOREIMM(dst, src) => {
                let src_value = src.int_value();
                let dst_name = dst.var_name();
                let dst_offset = dst.var_offset();
                format!(
                    "mov QWORD PTR -{}[rbp], {} # {}",
                    dst_offset, src_value, dst_name
                )
            }
            X64IRKind::STOREMEM(dst, src) => {
                let mut output = String::new();

                // メモリからメモリに直接movする命令はない.
                // ここではraxにロードし,そこから対象アドレスにロードする.
                let src_name = src.var_name();
                let src_offset = src.var_offset();
                output += &(format!("mov rax, -{}[rbp] # {}\n", src_offset, src_name).as_str());

                let dst_name = dst.var_name();
                let dst_offset = dst.var_offset();
                output += &(format!("  mov -{}[rbp], rax # {}", dst_offset, dst_name).as_str());

                output
            }
            // negative
            X64IRKind::NEGREG(inner_op) => {
                let negative_reg = Registers::from_number_ir(inner_op.phys);
                format!("neg {}", negative_reg.to_string())
            }
            // ret
            X64IRKind::RETREG(return_op) => {
                let mut output = String::new();
                let return_reg = Registers::from_number_ir(return_op.phys);
                output += &(format!("mov rax, {}\n", return_reg.to_string()).as_str());

                // 関数エピローグ
                output += &(format!("  mov rsp, rbp\n").as_str());
                output += &(format!("  pop rbp\n").as_str());
                output += &(format!("  ret").as_str());
                output
            }
            X64IRKind::RETIMM(return_op) => {
                let mut output = String::new();
                let return_value = return_op.int_value();
                output += &(format!("mov rax, {}\n", return_value).as_str());

                // 関数エピローグ
                output += &(format!("  mov rsp, rbp\n").as_str());
                output += &(format!("  pop rbp\n").as_str());
                output += &(format!("  ret").as_str());
                output
            }
            X64IRKind::RETMEM(return_op) => {
                let mut output = String::new();
                let return_name = return_op.var_name();
                let return_off = return_op.var_offset();
                output += &(format!("mov rax, -{}[rbp] # {}\n", return_off, return_name).as_str());

                // 関数エピローグ
                output += &(format!("  mov rsp, rbp\n").as_str());
                output += &(format!("  pop rbp\n").as_str());
                output += &(format!("  ret").as_str());
                output
            }
            X64IRKind::RETCALL(return_op) => {
                let mut output = String::new();
                let return_name = return_op.var_name();
                output += &(format!("call {}\n", return_name).as_str());

                // 関数エピローグ
                output += &(format!("  mov rsp, rbp\n").as_str());
                output += &(format!("  pop rbp\n").as_str());
                output += &(format!("  ret").as_str());
                output
            }
            // cmpzero
            X64IRKind::CMPZEROREG(cmp_op) => {
                let cmp_reg = Registers::from_number_ir(cmp_op.phys);
                format!("cmp {}, 0", cmp_reg.to_string())
            }
            X64IRKind::CMPZEROMEM(cmp_op) => {
                let cmp_name = cmp_op.var_name();
                let cmp_off = cmp_op.var_offset();
                format!("cmp QWORD PTR -{}[rbp], 0 # {}", cmp_off, cmp_name)
            }
            X64IRKind::JMP(label_name) => format!("jmp {}", label_name),
            X64IRKind::JZ(label_name) => format!("jz {}", label_name),
            // genparam
            X64IRKind::GENPARAMIMM(reg_num, gen_op) => {
                let dst_reg = Registers::from_arg_number(*reg_num);
                let gen_value = gen_op.int_value();
                format!("mov {}, {}", dst_reg.to_string(), gen_value)
            }
            X64IRKind::PUSHPARAM(reg_num, offset) => {
                let src_reg = Registers::from_arg_number(*reg_num);
                format!("mov QWORD PTR -{}[rbp], {}", offset, src_reg.to_string())
            }
            _ => {
                eprintln!("can't emit with invalid ir -> {:?}", self.kind);
                String::new()
            }
        }
    }
}
