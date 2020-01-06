use crate::target::Target;

pub struct AssemblyFile {
    pub code: String,
    pub target: Target,
}

impl AssemblyFile {
    pub fn new(code: String, target: Target) -> Self {
        Self {
            code: code,
            target: target,
        }
    }
}
