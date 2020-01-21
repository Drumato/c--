use crate::compiler::frontend::types;

#[derive(Clone)]
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
}

type StackOffset = usize;
#[derive(Clone)]
pub enum VarKind {
    LOCAL(StackOffset),
}
