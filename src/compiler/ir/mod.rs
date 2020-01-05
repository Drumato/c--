pub mod arch;
pub mod three_address_code;

use crate::compiler::frontend::Manager;

impl Manager {
    pub fn dump_tacs_to_stderr(&self) {
        self.entry_block.dump_tacs_to_stderr();
    }
}
