#[derive(PartialEq, Debug, Clone)]
pub struct X64Instruction {
    name: X64InstName,
    kind: X64InstKind,
}

impl X64Instruction {
    pub fn new_add(src: X64Operand, dst: X64Operand) -> Self {
        Self {
            name: X64InstName::ADD,
            kind: X64InstKind::BINARY(src, dst),
        }
    }
    pub fn new_mov(src: X64Operand, dst: X64Operand) -> Self {
        Self {
            name: X64InstName::MOV,
            kind: X64InstKind::BINARY(src, dst),
        }
    }
    pub fn new_ret() -> Self {
        Self {
            name: X64InstName::RET,
            kind: X64InstKind::NOOPERAND,
        }
    }
    pub fn to_string(&self) -> String {
        match &self.kind {
            X64InstKind::NOOPERAND => format!("{}", self.name.to_string()),
            X64InstKind::UNARY(op) => format!("{} {}", self.name.to_string(), op.to_string()),
            X64InstKind::BINARY(src, dst) => format!(
                "{} {}, {}",
                self.name.to_string(),
                dst.to_string(),
                src.to_string()
            ),
            X64InstKind::LABEL(name) => format!("{}:", name),
        }
    }
}

type SrcOperand = X64Operand;
type DstOperand = X64Operand;
#[derive(PartialEq, Debug, Clone)]
pub enum X64InstKind {
    // オペランドを取らないもの
    NOOPERAND,
    // 1つオペランドを取るもの
    UNARY(X64Operand),
    // 2つオペランドを取るもの
    // AT&T記法の順番で格納.
    BINARY(SrcOperand, DstOperand),
    // ラベルを命令として持つと,後で処理しやすい.
    LABEL(String),
}

#[derive(PartialEq, Debug, Clone)]
pub enum X64InstName {
    ADD,
    MOV,
    RET,
}
impl X64InstName {
    fn to_string(&self) -> String {
        match self {
            Self::ADD => "add".to_string(),
            Self::MOV => "mov".to_string(),
            Self::RET => "ret".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct X64Operand {
    kind: X64OpeKind,
}

impl X64Operand {
    pub fn new_label(name: String) -> Self {
        Self {
            kind: X64OpeKind::LABEL(name),
        }
    }
    pub fn new_register(number: usize) -> Self {
        Self {
            kind: X64OpeKind::REG(number),
        }
    }
    pub fn new_integer(value: i128) -> Self {
        Self {
            kind: X64OpeKind::INTEGER(value),
        }
    }
    pub fn new_invalid() -> Self {
        Self {
            kind: X64OpeKind::INVALID,
        }
    }
    pub fn to_string(&self) -> String {
        match &self.kind {
            X64OpeKind::REG(number) => format!("r{}", number),
            X64OpeKind::INTEGER(val) => format!("{}", val),
            X64OpeKind::LABEL(name) => name.to_string(),
            _ => "invalid".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum X64OpeKind {
    // レジスタ
    REG(usize),
    // 即値
    INTEGER(i128),

    // jump命令とか,ラベルをオペランドに持つ場合も
    LABEL(String),

    INVALID,
}
