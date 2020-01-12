use std::collections::BTreeMap;

type RegisterNumber = usize;
type LiveIn = usize;
type LiveOut = usize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BasicBlock {
    pub label: String,
    pub tacs: Vec<ThreeAddressCode>,

    pub living: BTreeMap<RegisterNumber, (LiveIn, LiveOut)>,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label: label,
            tacs: Vec::new(),
            living: BTreeMap::new(),
        }
    }
    pub fn dump_tacs_to_stderr(&self) {
        eprintln!("{}'s IR:", self.label);
        for t in self.tacs.iter() {
            eprintln!("\t{}", t.to_string());
        }
    }
    pub fn dump_tacs_to_stderr_with_physical(&self) {
        eprintln!("{}'s IR:", self.label);
        for t in self.tacs.iter() {
            eprintln!("\t{}", t.to_string_physical());
        }
    }
    pub fn dump_liveness(&self) {
        for (reg_number, range) in self.living.iter() {
            eprintln!("t{} --> {}...{}", reg_number, range.0, range.1);
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ThreeAddressCode {
    pub kind: TacKind,
}

impl ThreeAddressCode {
    pub fn new(kind: TacKind) -> Self {
        Self { kind: kind }
    }
    pub fn new_return_code(return_op: Operand) -> Self {
        Self::new(TacKind::RET(return_op))
    }
    pub fn new_binop_code(
        variable_op: Operand,
        operator: Operator,
        left: Operand,
        right: Operand,
    ) -> Self {
        Self::new(TacKind::EXPR(variable_op, operator, left, right))
    }

    pub fn to_string(&self) -> String {
        match &self.kind {
            TacKind::EXPR(var, op, left, right) => format!(
                "{} <- {} {} {}",
                var.to_string(),
                left.to_string(),
                op.to_string(),
                right.to_string()
            ),
            TacKind::RET(return_op) => format!("return {}", return_op.to_string()),
        }
    }
    pub fn to_string_physical(&self) -> String {
        match &self.kind {
            TacKind::EXPR(var, op, left, right) => format!(
                "{} <- {} {} {}",
                var.to_string_physical(),
                left.to_string_physical(),
                op.to_string(),
                right.to_string_physical()
            ),
            TacKind::RET(return_op) => format!("return {}", return_op.to_string_physical()),
        }
    }
}
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
    fn to_string(&self) -> &str {
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
