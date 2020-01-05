extern crate colored;
use colored::*;
pub fn colored_prefix_to_stderr(msg: &str) {
    eprintln!("++++++++ {} ++++++++", msg.bold().green());
}

pub fn colored_message_to_stderr(msg: &str) {
    eprintln!("{}", msg.bold().green());
}
