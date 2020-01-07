use crate::target::Target;

pub struct AssemblyFile {
    pub code: String,
    pub target: Target,
    pub syntax: Syntax,
}

impl AssemblyFile {
    pub fn new(code: String, target: Target) -> Self {
        Self {
            code: code,
            target: target,
            syntax: Syntax::INTEL,
        }
    }
}

pub enum Syntax {
    ATANDT, // AT&T syntax
    INTEL,  // Intel syntax
}
