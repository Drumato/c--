use std::collections::{BTreeMap, BTreeSet};

use crate::compiler::backend::high_optimizer::HighOptimizer;
use crate::compiler::ir::three_address_code::{tac::ThreeAddressCode, tac_kind::TacKind};

type RegisterNumber = usize;
#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct ControlFlowGraphInBB {
    pub succ: Vec<BTreeSet<usize>>,
    pub prev: Vec<BTreeSet<usize>>,
    pub used: Vec<BTreeSet<RegisterNumber>>,
    pub def: Vec<BTreeSet<RegisterNumber>>,
}
impl ControlFlowGraphInBB {
    pub fn new(len: usize) -> Self {
        Self {
            succ: vec![BTreeSet::new(); len],
            prev: vec![BTreeSet::new(); len],
            used: vec![BTreeSet::new(); len],
            def: vec![BTreeSet::new(); len],
        }
    }
}

impl HighOptimizer {
    pub fn build_cfg(&mut self) {
        let mut functions = self.functions.clone();

        let function_number = functions.len();
        for func_idx in 0..function_number {
            let block_number = functions[func_idx].blocks.len();
            for blk_idx in 0..block_number {
                let cfg_inbb =
                    self.build_cfg_with_bb(functions[func_idx].blocks[blk_idx].tacs.clone());
                functions[func_idx].blocks[blk_idx].cfg_inbb = cfg_inbb;
            }
        }

        self.functions = functions;
    }
    pub fn build_cfg_with_bb(&mut self, tacs: Vec<ThreeAddressCode>) -> ControlFlowGraphInBB {
        // jump-statement系にエッジを追加するときのために利用
        let label_map: BTreeMap<String, usize> = self.build_labelmap(&tacs);

        // 各ベーシックブロックに対応したCFG
        let mut cfg_inbb = ControlFlowGraphInBB::new(tacs.len());

        // 一つ前の文がgotoであるとき,をチェックする
        let mut prev_inst_is_goto = false;
        for (i, t) in tacs.iter().enumerate() {
            match &t.kind {
                TacKind::UNARYEXPR(_var, _operator, _inner) => {
                    self.add_succ(&mut cfg_inbb, tacs.len(), i, i + 1);

                    if i != 0 && !prev_inst_is_goto {
                        self.add_prev(&mut cfg_inbb, i, i - 1);
                    }
                }
                TacKind::EXPR(_var, _operator, _left, _right) => {
                    self.add_succ(&mut cfg_inbb, tacs.len(), i, i + 1);

                    if i != 0 && !prev_inst_is_goto {
                        self.add_prev(&mut cfg_inbb, i, i - 1);
                    }
                }
                TacKind::ASSIGN(_lv, _rv) => {
                    self.add_succ(&mut cfg_inbb, tacs.len(), i, i + 1);

                    if i != 0 && !prev_inst_is_goto {
                        self.add_prev(&mut cfg_inbb, i, i - 1);
                    }
                }
                TacKind::IFF(_lv, label) => {
                    self.add_succ(&mut cfg_inbb, tacs.len(), i, i + 1);

                    if i != 0 && !prev_inst_is_goto {
                        self.add_prev(&mut cfg_inbb, i, i - 1);
                    }

                    if let Some(label_idx) = label_map.get(label) {
                        self.add_succ(&mut cfg_inbb, tacs.len(), i, *label_idx);

                        self.add_prev(&mut cfg_inbb, *label_idx, i);
                    }

                    prev_inst_is_goto = true;
                    continue;
                }
                TacKind::RET(_return_op) => {
                    self.add_succ(&mut cfg_inbb, tacs.len(), i, i + 1);

                    if i != 0 && !prev_inst_is_goto {
                        self.add_prev(&mut cfg_inbb, i, i - 1);
                    }
                }
                TacKind::LABEL(_label_name) => {
                    self.add_succ(&mut cfg_inbb, tacs.len(), i, i + 1);

                    if i != 0 && !prev_inst_is_goto {
                        self.add_prev(&mut cfg_inbb, i, i - 1);
                    }
                }
                TacKind::GOTO(label_name) => {
                    if i != 0 && !prev_inst_is_goto {
                        self.add_prev(&mut cfg_inbb, i, i - 1);
                    }

                    if let Some(label_idx) = label_map.get(label_name) {
                        // goto-statement -> labeled-statement
                        self.add_succ(&mut cfg_inbb, tacs.len(), i, *label_idx);

                        // labeled-statementから見たエッジ
                        self.add_prev(&mut cfg_inbb, *label_idx, i);
                    }

                    prev_inst_is_goto = true;
                    continue;
                }
            }

            // TacKind::GOTO 以外はここを経由
            prev_inst_is_goto = false;
        }
        cfg_inbb
    }
    fn build_labelmap(&mut self, tacs: &Vec<ThreeAddressCode>) -> BTreeMap<String, usize> {
        let mut label_map: BTreeMap<String, usize> = BTreeMap::new();
        for (i, t) in tacs.iter().enumerate() {
            if let TacKind::LABEL(label_name) = t.kind.clone() {
                label_map.insert(label_name.to_string(), i);
            }
        }

        label_map
    }
    fn add_succ(
        &mut self,
        cfg_inbb: &mut ControlFlowGraphInBB,
        tac_length: usize,
        n: usize,
        edge: usize,
    ) {
        if edge < tac_length {
            cfg_inbb.succ[n].insert(edge);
        } else {
            // eprintln!("edge out of bound");
        }
    }
    fn add_prev(&mut self, cfg_inbb: &mut ControlFlowGraphInBB, n: usize, edge: usize) {
        if n != 0 {
            cfg_inbb.prev[n].insert(edge);
        }
    }
}

#[cfg(test)]
mod build_cfg_tests {}
