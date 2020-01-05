pub mod generate;
pub mod translate;

use crate::compiler::backend::HighOptimizer;
use crate::compiler::ir::arch::x64::*;

#[derive(Clone)]
pub struct X64Optimizer {
    // TODO: Vec<X64Function>,
    entry_bb: X64BasicBlock,
}
impl X64Optimizer {
    // TODO: Vec<X64Function> | Vec<X64Module>を受け取るように
    pub fn new(label: String, irs: Vec<X64IR>) -> Self {
        let entry_bb = X64BasicBlock::new(label, irs);
        Self { entry_bb: entry_bb }
    }
}

pub fn x64_process(high_optimizer: HighOptimizer) -> String {
    // TODO: 低レベルなIRFunctionのVectorが返る方が自然
    let x64_optimizer: X64Optimizer = HighOptimizer::translate_tacs_to_x64(high_optimizer);

    // コード生成
    let assembly = x64_optimizer.generate_assembly();

    assembly
}
