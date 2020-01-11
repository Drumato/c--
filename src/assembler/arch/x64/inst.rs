use crate::assembler::arch::x64::analyze::OperandSize;

#[derive(PartialEq, Debug, Clone)]
pub struct X64Instruction {
    pub name: X64InstName,
    pub kind: X64InstKind,

    pub operand_size: OperandSize,
    // 拡張レジスタを用いているか
    pub src_expanded: bool,

    // 一つオペランドを取る命令も利用
    pub dst_expanded: bool,

    // オペランドのレジスタ番号
    pub src_regnumber: usize,
    pub dst_regnumber: usize,

    // 即値も取ってしまう
    pub immediate_value: i128,
}

impl X64Instruction {
    pub fn new_add(src: X64Operand, dst: X64Operand) -> Self {
        Self {
            name: X64InstName::ADD,
            kind: X64InstKind::BINARY(src, dst),
            operand_size: OperandSize::UNKNOWN,
            src_expanded: false,
            dst_expanded: false,
            src_regnumber: 0,
            dst_regnumber: 0,
            immediate_value: 0,
        }
    }
    pub fn new_call(call_op: X64Operand) -> Self {
        Self {
            name: X64InstName::CALL,
            kind: X64InstKind::UNARY(call_op),
            operand_size: OperandSize::UNKNOWN,
            src_expanded: false,
            dst_expanded: false,
            src_regnumber: 0,
            dst_regnumber: 0,
            immediate_value: 0,
        }
    }
    pub fn new_mov(src: X64Operand, dst: X64Operand) -> Self {
        Self {
            name: X64InstName::MOV,
            kind: X64InstKind::BINARY(src, dst),
            operand_size: OperandSize::UNKNOWN,
            src_expanded: false,
            dst_expanded: false,
            src_regnumber: 0,
            dst_regnumber: 0,
            immediate_value: 0,
        }
    }
    pub fn new_ret() -> Self {
        Self {
            name: X64InstName::RET,
            kind: X64InstKind::NOOPERAND,
            operand_size: OperandSize::UNKNOWN,
            src_expanded: false,
            dst_expanded: false,
            src_regnumber: 0,
            dst_regnumber: 0,
            immediate_value: 0,
        }
    }
    pub fn new_syscall() -> Self {
        Self {
            name: X64InstName::SYSCALL,
            kind: X64InstKind::NOOPERAND,
            operand_size: OperandSize::UNKNOWN,
            src_expanded: false,
            dst_expanded: false,
            src_regnumber: 0,
            dst_regnumber: 0,
            immediate_value: 0,
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
#[allow(dead_code)]
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
    // 抽象的なオペコード
    ADD,
    CALL,
    MOV,
    RET,
    SYSCALL,

    // 具体的なオペコード
    ADDRM64IMM32,
    ADDRM64R64,
    CALLRM64,
    MOVRM64IMM32,
    MOVRM64R64,
}
impl X64InstName {
    fn to_string(&self) -> String {
        match self {
            Self::ADD => "add".to_string(),
            Self::CALL => "call".to_string(),
            Self::MOV => "mov".to_string(),
            Self::RET => "ret".to_string(),
            Self::SYSCALL => "syscall".to_string(),
            Self::ADDRM64IMM32 => "add(r/m64 imm32)".to_string(),
            Self::ADDRM64R64 => "add(r/m64 r64)".to_string(),
            Self::CALLRM64 => "call(r/m64)".to_string(),
            Self::MOVRM64IMM32 => "mov(r/m64 imm32)".to_string(),
            Self::MOVRM64R64 => "mov(r/m64 r64)".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct X64Operand {
    pub kind: X64OpeKind,
}

impl X64Operand {
    pub fn new_label(name: String) -> Self {
        Self {
            kind: X64OpeKind::LABEL(name),
        }
    }
    pub fn new_register(name: String) -> Self {
        Self {
            kind: X64OpeKind::REG(name),
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
            X64OpeKind::REG(name) => name.to_string(),
            X64OpeKind::INTEGER(val) => format!("{}", val),
            X64OpeKind::LABEL(name) => name.to_string(),
            _ => "invalid".to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum X64OpeKind {
    // レジスタ
    REG(String),

    // 即値
    INTEGER(i128),

    // jump命令とか,ラベルをオペランドに持つ場合も
    LABEL(String),

    INVALID,
}
