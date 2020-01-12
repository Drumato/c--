pub mod inst_kind;
pub mod inst_name;

use crate::assembler::arch::x64::analyze::OperandSize;

#[derive(PartialEq, Debug, Clone)]
pub struct X64Instruction {
    pub name: inst_name::X64InstName,
    pub kind: inst_kind::X64InstKind,

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
    pub fn new(inst_name: inst_name::X64InstName, kind: inst_kind::X64InstKind) -> Self {
        Self {
            name: inst_name,
            kind: kind,
            operand_size: OperandSize::UNKNOWN,
            src_expanded: false,
            dst_expanded: false,
            src_regnumber: 0,
            dst_regnumber: 0,
            immediate_value: 0,
        }
    }
    pub fn new_binary_inst(
        name: inst_name::X64InstName,
        src: inst_kind::X64Operand,
        dst: inst_kind::X64Operand,
    ) -> Self {
        match name {
            inst_name::X64InstName::ADD => Self::new_add(src, dst),
            inst_name::X64InstName::SUB => Self::new_sub(src, dst),
            inst_name::X64InstName::MOV => Self::new_mov(src, dst),
            _ => panic!("no such a binary instruction"),
        }
    }
    fn new_add(src: inst_kind::X64Operand, dst: inst_kind::X64Operand) -> Self {
        Self::new(
            inst_name::X64InstName::ADD,
            inst_kind::X64InstKind::BINARY(src, dst),
        )
    }
    fn new_sub(src: inst_kind::X64Operand, dst: inst_kind::X64Operand) -> Self {
        Self::new(
            inst_name::X64InstName::SUB,
            inst_kind::X64InstKind::BINARY(src, dst),
        )
    }
    pub fn new_call(call_op: inst_kind::X64Operand) -> Self {
        Self::new(
            inst_name::X64InstName::CALL,
            inst_kind::X64InstKind::UNARY(call_op),
        )
    }
    pub fn new_mov(src: inst_kind::X64Operand, dst: inst_kind::X64Operand) -> Self {
        Self::new(
            inst_name::X64InstName::MOV,
            inst_kind::X64InstKind::BINARY(src, dst),
        )
    }
    pub fn new_ret() -> Self {
        Self::new(
            inst_name::X64InstName::RET,
            inst_kind::X64InstKind::NOOPERAND,
        )
    }
    pub fn new_syscall() -> Self {
        Self::new(
            inst_name::X64InstName::SYSCALL,
            inst_kind::X64InstKind::NOOPERAND,
        )
    }
    pub fn to_string(&self) -> String {
        match &self.kind {
            inst_kind::X64InstKind::NOOPERAND => format!("{}", self.name.to_string()),
            inst_kind::X64InstKind::UNARY(op) => {
                format!("{} {}", self.name.to_string(), op.to_string())
            }
            inst_kind::X64InstKind::BINARY(src, dst) => format!(
                "{} {}, {}",
                self.name.to_string(),
                dst.to_string(),
                src.to_string()
            ),
            inst_kind::X64InstKind::LABEL(name) => format!("{}:", name),
        }
    }
}
