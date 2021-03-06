type LabelName = String;
type Offset = usize;
type RegNumber = usize;
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
    JZ(LabelName),
    CMPZERO(X64Operand),
    STORE(X64Operand, X64Operand),

    // 1つオペランドを持つ系
    NEGATIVE(X64Operand),
    RET(X64Operand),

    // 具体的なIR
    // 2つオペランドを持つ系
    ADDIMMTOREG(X64Operand, X64Operand),
    ADDREGTOREG(X64Operand, X64Operand),
    ADDIMMTOVAR(X64Operand, X64Operand),
    MOVIMMTOREG(X64Operand, X64Operand),
    MOVREGTOREG(X64Operand, X64Operand),
    MOVMEMTOREG(X64Operand, X64Operand),
    SUBIMMTOREG(X64Operand, X64Operand),
    SUBREGTOREG(X64Operand, X64Operand),
    SUBIMMTOVAR(X64Operand, X64Operand),
    MULIMMTOREG(X64Operand, X64Operand),
    MULREGTOREG(X64Operand, X64Operand),
    DIVIMMTOREG(X64Operand, X64Operand),
    DIVREGTOREG(X64Operand, X64Operand),

    // 1つオペランドを持つ系
    NEGREG(X64Operand),
    RETREG(X64Operand),
    RETIMM(X64Operand),
    RETMEM(X64Operand),
    RETCALL(X64Operand),
    STOREREG(X64Operand, X64Operand),
    STOREIMM(X64Operand, X64Operand),
    STOREMEM(X64Operand, X64Operand),
    CMPZEROREG(X64Operand),
    CMPZEROIMM(X64Operand),
    CMPZEROMEM(X64Operand),

    // その他
    GENPARAM(RegNumber, X64Operand),
    GENPARAMIMM(RegNumber, X64Operand),
    PUSHPARAM(RegNumber, Offset),
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
    pub fn var_name(&self) -> &String {
        match &self.kind {
            X64OpeKind::AUTOVAR(name, _offset) => &name,
            X64OpeKind::CALL(name) => &name,
            _ => panic!("can't get variable-name without autovar"),
        }
    }
    pub fn var_offset(&self) -> usize {
        match &self.kind {
            X64OpeKind::AUTOVAR(_name, offset) => *offset,
            _ => panic!("can't get variable-offset without autovar"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum X64OpeKind {
    INTLIT(i128),
    CALL(String),
    AUTOVAR(String, usize),
    REG,
    INVALID,
}
