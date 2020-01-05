pub mod three_address_code;
pub mod x64;

use crate::compiler::frontend::Manager;

impl Manager {
    pub fn dump_tacs_to_stderr(&self) {
        self.entry_block.dump_tacs_to_stderr();
    }
}
