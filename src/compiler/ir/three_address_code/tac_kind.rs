#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum TacKind {
    EXPR(Operand, Operator, Operand, Operand),
    RET(Operand),
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum Operator {
    PLUS,
    MINUS,
}

impl Operator {
    pub fn to_string(&self) -> &str {
        match self {
            Self::PLUS => "+",
            Self::MINUS => "-",
        }
    }
}

// ローカル変数等にも仮想/物理レジスタ番号を持たせているのは,
// 最適化によってローカル変数をレジスタに割り付ける可能性があるため.
//
// OpeKind::REG(Virtual,Physical) のようにしてしまうとやりづらい.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub struct Operand {
    pub kind: OpeKind,
    pub virt: usize,
    pub phys: usize,
}
impl Operand {
    pub fn new(kind: OpeKind) -> Self {
        Self {
            kind: kind,
            virt: 0,
            phys: 0,
        }
    }
    pub fn new_int_literal(val: i128) -> Self {
        Self::new(OpeKind::INTLIT(val))
    }
    pub fn new_virtreg(virt: usize) -> Self {
        let mut base_reg = Self::new(OpeKind::REG);
        base_reg.virt = virt;
        base_reg
    }
    pub fn new_invalid() -> Self {
        Self::new(OpeKind::INVALID)
    }
    pub fn is_register(&self) -> bool {
        match self.kind {
            OpeKind::REG => true,
            _ => false,
        }
    }
    pub fn to_string(&self) -> String {
        match self.kind {
            OpeKind::INTLIT(val) => format!("{}", val),
            OpeKind::REG => format!("t{}", self.virt),
            OpeKind::INVALID => "invalid".to_string(),
        }
    }
    pub fn to_string_physical(&self) -> String {
        match self.kind {
            OpeKind::INTLIT(val) => format!("{}", val),
            OpeKind::REG => format!("t{}", self.phys),
            OpeKind::INVALID => "invalid".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum OpeKind {
    INTLIT(i128),
    REG,
    INVALID,
}
