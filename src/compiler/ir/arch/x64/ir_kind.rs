type LabelName = String;
#[derive(Debug, Clone)]
pub enum X64IRKind {
    // 抽象的なIR
    // 2つオペランドを持つ系
    MOV(X64Operand, X64Operand),
    ADD(X64Operand, X64Operand),
    SUB(X64Operand, X64Operand),
    MUL(X64Operand, X64Operand),
    DIV(X64Operand, X64Operand),
    JMP(LabelName),

    // 1つオペランドを持つ系
    RET(X64Operand),

    // 具体的的なIR
    ADDIMMTOREG(X64Operand, X64Operand),
    ADDREGTOREG(X64Operand, X64Operand),
    MOVIMMTOREG(X64Operand, X64Operand),
    MOVREGTOREG(X64Operand, X64Operand),
    SUBIMMTOREG(X64Operand, X64Operand),
    SUBREGTOREG(X64Operand, X64Operand),
    MULIMMTOREG(X64Operand, X64Operand),
    MULREGTOREG(X64Operand, X64Operand),
    DIVIMMTOREG(X64Operand, X64Operand),
    DIVREGTOREG(X64Operand, X64Operand),
    RETREG(X64Operand),
    RETIMM(X64Operand),
}
#[derive(Debug, Clone)]
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
    pub fn new_rax() -> Self {
        Self {
            kind: X64OpeKind::REG,
            virt: 0,
            phys: 6,
        }
    }
    pub fn new_inv() -> Self {
        Self {
            kind: X64OpeKind::INVALID,
            virt: 0,
            phys: 0,
        }
    }
    pub fn int_value(&self) -> i128 {
        match self.kind {
            X64OpeKind::INTLIT(value) => value,
            _ => panic!("can't get immediate-value without intlit-kind"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum X64OpeKind {
    INTLIT(i128),
    REG,
    INVALID,
}
