use crate::compiler::backend::arch::x64::optimizer::X64Optimizer;
use crate::compiler::ir::arch::x64::{
    basicblock::X64BasicBlock,
    ir_kind::{X64IRKind, X64OpeKind},
};

impl X64Optimizer {
    pub fn select_best_instruction(&mut self) {
        let mut allocated_blocks = Vec::new();
        let blocks = self.entry_func.blocks.clone();
        for block in blocks {
            let allocated_block = self.change_ir_with_bb(block);

            allocated_blocks.push(allocated_block);
        }

        self.entry_func.blocks = allocated_blocks;
    }

    fn change_ir_with_bb(&mut self, mut block: X64BasicBlock) -> X64BasicBlock {
        for ir in block.irs.iter_mut() {
            match &ir.kind {
                // 今はレジスタに対するmovしかない
                X64IRKind::MOV(dst, src) => {
                    match &src.kind {
                        // mov reg, reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::MOVREGTOREG(dst.clone(), src.clone());
                        }

                        // mov reg, imm
                        X64OpeKind::INTLIT(_value) => {
                            ir.kind = X64IRKind::MOVIMMTOREG(dst.clone(), src.clone());
                        }
                        _ => panic!("not implemented in mov selection"),
                    }
                }
                // 今はレジスタに対するaddしかない
                X64IRKind::ADD(dst, src) => {
                    match &src.kind {
                        // add reg, reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::ADDREGTOREG(dst.clone(), src.clone());
                        }

                        // add reg, imm
                        X64OpeKind::INTLIT(_value) => {
                            ir.kind = X64IRKind::ADDIMMTOREG(dst.clone(), src.clone());
                        }
                        _ => panic!("not implemented in add selection"),
                    }
                }

                // 今はレジスタに対するsubしかない
                X64IRKind::SUB(dst, src) => {
                    match &src.kind {
                        // sub reg, reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::SUBREGTOREG(dst.clone(), src.clone());
                        }

                        // sub reg, imm
                        X64OpeKind::INTLIT(_value) => {
                            ir.kind = X64IRKind::SUBIMMTOREG(dst.clone(), src.clone());
                        }
                        _ => panic!("not implemented in sub selection"),
                    }
                }
                // 今はレジスタに対するmulしかない
                X64IRKind::MUL(dst, src) => {
                    match &src.kind {
                        // mul reg, reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::MULREGTOREG(dst.clone(), src.clone());
                        }

                        // mul reg, imm
                        X64OpeKind::INTLIT(_value) => {
                            ir.kind = X64IRKind::MULIMMTOREG(dst.clone(), src.clone());
                        }
                        _ => panic!("not implemented in mul selection"),
                    }
                }
                // 今はレジスタに対するdivしかない
                X64IRKind::DIV(dst, src) => {
                    match &src.kind {
                        // div reg, reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::DIVREGTOREG(dst.clone(), src.clone());
                        }

                        // div reg, imm
                        X64OpeKind::INTLIT(_value) => {
                            ir.kind = X64IRKind::DIVIMMTOREG(dst.clone(), src.clone());
                        }
                        _ => panic!("not implemented in div selection"),
                    }
                }
                X64IRKind::RET(return_op) => {
                    match &return_op.kind {
                        // return reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::RETREG(return_op.clone());
                        }

                        // return imm
                        X64OpeKind::INTLIT(_value) => {
                            ir.kind = X64IRKind::RETIMM(return_op.clone());
                        }

                        // return var
                        X64OpeKind::AUTOVAR(_name, _offset) => {
                            ir.kind = X64IRKind::RETMEM(return_op.clone());
                        }
                        _ => panic!("not implemented in ret selection"),
                    }
                }
                X64IRKind::STORE(dst_op, src_op) => {
                    match &src_op.kind {
                        // store reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::STOREREG(dst_op.clone(), src_op.clone());
                        }

                        // store imm
                        X64OpeKind::INTLIT(_value) => {
                            ir.kind = X64IRKind::STOREIMM(dst_op.clone(), src_op.clone());
                        }

                        _ => panic!("not implemented in store selection"),
                    }
                }
                _ => (),
            }
        }
        block
    }
}
