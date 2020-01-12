#[derive(Clone)]
pub enum X64IRKind {
    // 2つオペランドを持つ系
    MOV(X64Operand, X64Operand),
    ADD(X64Operand, X64Operand),
    SUB(X64Operand, X64Operand),

    // 1つオペランドを持つ系
    RET(X64Operand),
}
#[derive(Clone)]
pub struct X64Operand {
    pub kind: X64OpeKind,
    pub virt: usize,
    pub phys: usize,
}

impl X64Operand {
    pub fn new(kind: X64OpeKind, virt: usize, phys: usize) -> Self {
        Self {
            kind: kind,
            virt: virt,
            phys: phys,
        }
    }
    pub fn new_inv() -> Self {
        Self {
            kind: X64OpeKind::INVALID,
            virt: 0,
            phys: 0,
        }
    }
}

#[derive(Clone)]
pub enum X64OpeKind {
    INTLIT(i128),
    REG,
    INVALID,
}
