pub mod generate;

#[derive(Debug, PartialEq, Clone)]
pub struct BasicBlock {
    // TODO: CASE文やif-elseに対応するbranchを生やす
    pub label: String,
    pub tacs: Vec<ThreeAddressCode>,
}

impl BasicBlock {
    pub fn new(label: String) -> Self {
        Self {
            label: label,
            tacs: Vec::new(),
        }
    }
    pub fn dump_tacs_to_stderr(&self) {
        eprintln!("{}'s IR:", self.label);
        for t in self.tacs.iter() {
            eprintln!("\t{}", t.to_string());
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreeAddressCode {
    kind: TacKind,
}

impl ThreeAddressCode {
    fn new(kind: TacKind) -> Self {
        Self { kind: kind }
    }
    fn new_return_code(return_op: Operand) -> Self {
        Self::new(TacKind::RET(return_op))
    }
    fn new_binop_code(
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
}
#[derive(Debug, PartialEq, Clone)]
pub enum TacKind {
    EXPR(Operand, Operator, Operand, Operand),
    RET(Operand),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    PLUS,
}

impl Operator {
    fn to_string(&self) -> &str {
        match self {
            Self::PLUS => "+",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Operand {
    kind: OpeKind,
    virt: usize,
    phys: usize,
}
impl Operand {
    fn new(kind: OpeKind) -> Self {
        Self {
            kind: kind,
            virt: 0,
            phys: 0,
        }
    }
    fn new_int_literal(val: i128) -> Self {
        Self::new(OpeKind::INTLIT(val))
    }
    fn new_virtreg(virt: usize) -> Self {
        let mut base_reg = Self::new(OpeKind::REG);
        base_reg.virt = virt;
        base_reg
    }
    fn new_invalid() -> Self {
        Self::new(OpeKind::INVALID)
    }
    fn to_string(&self) -> String {
        match self.kind {
            OpeKind::INTLIT(val) => format!("{}", val),
            OpeKind::REG => format!("t{}", self.virt),
            OpeKind::INVALID => "invalid".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpeKind {
    INTLIT(i128),
    REG,
    INVALID,
}
