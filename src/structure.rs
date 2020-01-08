use crate::target::Target;

pub struct AssemblyFile {
    pub code: String,
    pub target: Target,
    pub syntax: Syntax,
}

impl AssemblyFile {
    pub fn new_intel_file(code: String, target: Target) -> Self {
        Self {
            code: code,
            target: target,
            syntax: Syntax::INTEL,
        }
    }
    pub fn new_atandt_file(code: String, target: Target) -> Self {
        Self {
            code: code,
            target: target,
            syntax: Syntax::ATANDT,
        }
    }
}

pub enum Syntax {
    ATANDT, // AT&T syntax
    INTEL,  // Intel syntax
}
