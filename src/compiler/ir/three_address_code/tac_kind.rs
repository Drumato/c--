type Label = String;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum TacKind {
    EXPR(Operand, Operator, Operand, Operand),
    UNARYEXPR(Operand, Operator, Operand),
    RET(Operand),
    GOTO(Label),
    ASSIGN(Operand, Operand),
    IFF(Operand, Label),

    // ラベルを必要とするのは,CFG構築などで存在すると便利だから.
    // BasicBlockがこの情報を保持しているので,Low-IRに変換したときに捨てる.
    LABEL(Label),
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum Operator {
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
}

impl Operator {
    pub fn to_string(&self) -> &str {
        match self {
            Self::PLUS => "+",
            Self::MINUS => "-",
            Self::ASTERISK => "*",
            Self::SLASH => "/",
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
    pub fn new_auto_var(name: String, offset: usize) -> Self {
        Self::new(OpeKind::AUTOVARIABLE(name, offset))
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
        match &self.kind {
            OpeKind::INTLIT(val) => format!("{}", val),
            OpeKind::AUTOVARIABLE(name, _offset) => format!("{}", name),
            OpeKind::REG => format!("t{}", self.virt),
            OpeKind::INVALID => "invalid".to_string(),
        }
    }
    pub fn to_string_physical(&self) -> String {
        match &self.kind {
            OpeKind::INTLIT(val) => format!("{}", val),
            OpeKind::AUTOVARIABLE(name, offset) => format!("{}[sp-{}]", name, offset),
            OpeKind::REG => format!("t{}", self.phys),
            OpeKind::INVALID => "invalid".to_string(),
        }
    }
}

type Offset = usize;
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone)]
pub enum OpeKind {
    INTLIT(i128),
    REG,
    AUTOVARIABLE(String, Offset),
    INVALID,
}
