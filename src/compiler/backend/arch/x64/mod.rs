extern crate clap;

pub mod generate;
pub mod translate;

use crate::compiler::backend::HighOptimizer;
use crate::compiler::ir::arch::x64::*;

#[derive(Clone)]
pub struct X64Optimizer {
    entry_bb: X64BasicBlock,
}
impl X64Optimizer {
    pub fn new(label: String, irs: Vec<X64IR>) -> Self {
        let entry_bb = X64BasicBlock::new(label, irs);
        Self { entry_bb: entry_bb }
    }
}

pub fn x64_process(matches: &clap::ArgMatches, high_optimizer: HighOptimizer) -> String {
    let x64_optimizer: X64Optimizer = HighOptimizer::translate_tacs_to_x64(high_optimizer);

    // コード生成
    let assembly = if matches.is_present("at-and-t-syntax") {
        x64_optimizer.generate_assembly_with_at_and_t_syntax()
    } else {
        x64_optimizer.generate_assembly_with_intel_syntax()
    };

    assembly
}
