pub mod arch;
pub mod three_address_code;

use crate::compiler::frontend::manager::Manager;

impl Manager {
    pub fn dump_tacs_to_stderr(&self) {
        eprintln!("{}'s blocks", self.ir_func.name);
        for bb in self.ir_func.blocks.iter() {
            bb.dump_tacs_to_stderr();
        }
    }
}
