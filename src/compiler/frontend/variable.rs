use crate::compiler::frontend::types;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Variable {
    pub kind: VarKind,
    pub ctype: types::Type,
}
impl Variable {
    pub fn init_local(ty: types::Type) -> Self {
        Self {
            kind: VarKind::LOCAL(0),
            ctype: ty,
        }
    }
    pub fn get_local_offset(&self) -> usize {
        match &self.kind {
            VarKind::LOCAL(offset) => *offset,
        }
    }
}

type StackOffset = usize;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VarKind {
    LOCAL(StackOffset),
}
