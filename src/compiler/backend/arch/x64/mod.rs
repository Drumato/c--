extern crate clap;

pub mod generate;
pub mod optimizer;
pub mod translate;

use crate::compiler::backend::high_optimizer::HighOptimizer;

pub fn x64_process(matches: &clap::ArgMatches, high_opt: HighOptimizer) -> String {
    let x64_optimizer: optimizer::X64Optimizer = HighOptimizer::translate_tacs_to_x64(high_opt);

    // TODO: 命令選択実装する必要あり
    // x64_optimizer.select_best_instruction();
    // コード生成
    let assembly = if matches.is_present("atandt-syntax") {
        x64_optimizer.generate_assembly_with_at_and_t_syntax()
    } else {
        x64_optimizer.generate_assembly_with_intel_syntax()
    };

    assembly
}
