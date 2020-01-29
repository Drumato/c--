use crate::compiler::frontend::manager::Manager;
use crate::compiler::frontend::node::{Function, NodeKind};
use crate::compiler::frontend::variable::VarKind;

impl Manager {
    pub fn alloc_frame(&mut self) {
        // 複数関数を実装した時用に関数自体を渡す.

        let mut func = self.entry_func.clone();
        self.alloc_frame_for_function(&mut func);
        self.entry_func = func;
    }
    #[allow(irrefutable_let_patterns)]
    pub fn alloc_frame_for_function(&mut self, func: &mut Function) {
        // 簡易実装として,DECLARATIONノードを見たら割り当てるように
        let mut stack_offset: usize = 0;
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
