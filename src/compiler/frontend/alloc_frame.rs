use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Function, NodeKind};
use crate::compiler::frontend::variable::VarKind;

impl Manager {
    pub fn alloc_frame(&mut self) {
        let mut functions = self.functions.clone();
        let functions_number = functions.len();
        for func_idx in 0..functions_number {
            self.var_map = functions[func_idx].local_map.clone();
            self.params = functions[func_idx].params.clone();
            self.alloc_frame_for_function(&mut functions[func_idx]);
            functions[func_idx].local_map = self.var_map.clone();
            functions[func_idx].params = self.params.clone();
            self.var_map.clear();
            self.params.clear();
        }
        self.functions = functions;
    }
    #[allow(irrefutable_let_patterns)]
    pub fn alloc_frame_for_function(&mut self, func: &mut Function) {
        // 簡易実装として,DECLARATIONノードを見たら割り当てるように
        let mut stack_offset: usize = 0;
        for (_name, param) in self.params.iter_mut() {
            if let VarKind::LOCAL(ref mut offset) = param.kind {
                stack_offset += param.ctype.byte_size;
                *offset = stack_offset;
            }
        }
        for stmt in func.stmts.iter() {
            match &stmt.kind {
                NodeKind::DECLARATION(var_name, var_type) => {
                    if let Some(local_symbol) = self.var_map.get_mut(var_name) {
                        if let VarKind::LOCAL(ref mut offset) = local_symbol.kind {
                            stack_offset += var_type.byte_size;
                            *offset = stack_offset;
                        }
                    }
                }
                NodeKind::LABELEDSTMT(_label, inner_st) => {
                    if let NodeKind::DECLARATION(var_name, var_type) = &inner_st.kind {
                        if let Some(local_symbol) = self.var_map.get_mut(var_name) {
                            if let VarKind::LOCAL(ref mut offset) = local_symbol.kind {
                                stack_offset += var_type.byte_size;
                                *offset = stack_offset;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        func.frame_size = stack_offset;
    }
}
