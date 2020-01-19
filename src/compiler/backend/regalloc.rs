use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::three_address_code::{
    basicblock::BasicBlock,
    tac_kind::{OpeKind, TacKind},
};
use crate::error::*;

use std::collections::BTreeMap;

impl HighOptimizer {
    // 簡易的な実装
    pub fn register_allocation_for_virtual_registers(&mut self, available_registers: usize) {
        let mut allocated_blocks = Vec::new();
        let blocks = self.entry_func.blocks.clone();
        for block in blocks {
            let allocated_block = self.register_allocation_for_bb(block, available_registers);

            allocated_blocks.push(allocated_block);
        }

        self.entry_func.blocks = allocated_blocks;
    }
    pub fn register_allocation_for_bb(
        &mut self,
        mut block: BasicBlock,
        available_registers: usize,
    ) -> BasicBlock {
        // レジスタの使用を管理するマップ
        // virtual_register_number -> physical_register_number
        let mut register_map: BTreeMap<usize, usize> = BTreeMap::new();

        let living = block.living.clone();

        // 各IRのレジスタに物理レジスタを割り当てる
        for (now_looking, t) in block.tacs.iter_mut().enumerate() {
            match t.kind {
                TacKind::EXPR(ref mut var_op, ref mut _operator, ref mut left, ref mut right) => {
                    // 左オペランド
                    if let OpeKind::REG = left.kind {
                        if let Some(allocated_number) = register_map.get(&left.virt) {
                            left.phys = *allocated_number;
                        } else {
                            eprintln!("spill occured!(not implemented)");
                        }
                    }

                    // レジスタ数の削減
                    Self::reduce_register_number(&living, &mut register_map, now_looking);

                    // 右オペランド
                    if let OpeKind::REG = right.kind {
                        if let Some(allocated_number) = register_map.get(&right.virt) {
                            right.phys = *allocated_number;
                        } else {
                            eprintln!("spill occued!(not implemented)");
                        }
                    }

                    // レジスタ数の削減
                    Self::reduce_register_number(&living, &mut register_map, now_looking);

                    // 実際の割付
                    var_op.phys = register_map.len();
                    register_map.insert(var_op.virt, var_op.phys);
                }
                TacKind::RET(ref mut return_op) => {
                    if let OpeKind::REG = return_op.kind {
                        if let Some(allocated_number) = register_map.get(&return_op.virt) {
                            return_op.phys = *allocated_number;
                        } else {
                            eprintln!("spill occued!(not implemented)");
                        }
                    }
                }
                _ => (),
            }

            // レジスタ数の削減
            Self::reduce_register_number(&living, &mut register_map, now_looking);

            // もし使用可能なレジスタ数を超えていたら
            // (仮想レジスタに関しては起きないと思うけど)
            if available_registers <= register_map.len() {
                let err = Error::new(
                    ErrorKind::RegAlloc,
                    (0, 0),
                    ErrorMsg::CantUseNoMoreRegisters,
                );
                err.compile_error();
            }
        }

        block
    }
    fn reduce_register_number(
        living: &BTreeMap<usize, (usize, usize)>,
        register_map: &mut BTreeMap<usize, usize>,
        now_looking: usize,
    ) {
        // 生存情報をループ
        // すでに死んでるレジスタがいればマップから取り除く
        for (virt_num, (_live_in, live_out)) in living.iter() {
            if register_map.contains_key(virt_num) {
                if *live_out < now_looking {
                    register_map.remove(virt_num);
                }
            }
        }
    }
}
