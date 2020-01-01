pub mod three_address_code;

extern crate colored;
use colored::*;

use crate::compiler::frontend::Manager;

impl Manager {
    pub fn dump_tacs_to_stderr(&self) {
        eprintln!(
            "++++++++ {} ++++++++",
            "dump three address code".bold().green()
        );
        for t in self.tacs.iter() {
            eprintln!("\t{}", t.to_string());
        }
    }
}
