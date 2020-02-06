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
                // mov
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
                // add
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

                // sub
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
                // mul
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
                // div
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
                // TODO: 今はレジスタに対するnegしかしない
                X64IRKind::NEGATIVE(inner_op) => {
                    match &inner_op.kind {
                        // negative reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::NEGREG(inner_op.clone());
                        }
                        _ => panic!("not implemented in negative selection"),
                    }
                }
                // ret
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

                // store
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

                        // store var
                        X64OpeKind::AUTOVAR(_name, _offset) => {
                            ir.kind = X64IRKind::STOREMEM(dst_op.clone(), src_op.clone());
                        }

                        _ => panic!("not implemented in store selection"),
                    }
                }
                // cmpzero
                X64IRKind::CMPZERO(cmp_op) => {
                    match &cmp_op.kind {
                        // cmpzero reg
                        X64OpeKind::REG => {
                            ir.kind = X64IRKind::CMPZEROREG(cmp_op.clone());
                        }

                        // cmpzero imm
                        X64OpeKind::INTLIT(_value) => {
                            ir.kind = X64IRKind::CMPZEROIMM(cmp_op.clone());
                        }

                        // cmpzero var
                        X64OpeKind::AUTOVAR(_name, _offset) => {
                            ir.kind = X64IRKind::CMPZEROMEM(cmp_op.clone());
                        }

                        _ => panic!("not implemented in cmpzero selection"),
                    }
                }
                _ => (),
            }
        }
        block
    }
}
