pub mod arch;
pub mod three_address_code;

use crate::compiler::frontend::manager::Manager;

impl Manager {
    pub fn dump_tacs_to_stderr(&self) {
        for func in self.ir_funcs.iter() {
            eprintln!("{}'s blocks", func.name);
            for bb in func.blocks.iter() {
                bb.dump_tacs_to_stderr();
            }
        }
    }
}
