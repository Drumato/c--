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

    // 即値,オフセッも取ってしまう
    pub immediate_value: i128,
    pub load_offset: i128,
    pub store_offset: i128,
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
            load_offset: 0,
            store_offset: 0,
        }
    }
    pub fn new_noop_inst(name: inst_name::X64InstName) -> Self {
        match name {
            inst_name::X64InstName::SYSCALL => Self::new_syscall(),
            inst_name::X64InstName::CQO => Self::new_cqo(),
            inst_name::X64InstName::RET => Self::new_ret(),
            _ => panic!("no such a no-operand instruction"),
        }
    }
    pub fn new_unary_inst(name: inst_name::X64InstName, unop: inst_kind::X64Operand) -> Self {
        match name {
            inst_name::X64InstName::CALL => Self::new_call(unop),
            inst_name::X64InstName::NEG => Self::new_neg(unop),
            inst_name::X64InstName::JMP => Self::new_jmp(unop),
            inst_name::X64InstName::JZ => Self::new_jz(unop),
            inst_name::X64InstName::IDIV => Self::new_idiv(unop),
            inst_name::X64InstName::PUSH => Self::new_push(unop),
            inst_name::X64InstName::POP => Self::new_pop(unop),
            _ => panic!("no such an unary instruction"),
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
            inst_name::X64InstName::IMUL => Self::new_imul(src, dst),
            inst_name::X64InstName::MOV => Self::new_mov(src, dst),
            inst_name::X64InstName::CMP => Self::new_cmp(src, dst),
            _ => panic!("no such a binary instruction"),
        }
    }
    pub fn new_label(label_name: String) -> Self {
        Self {
            name: inst_name::X64InstName::LABEL,
            kind: inst_kind::X64InstKind::LABEL(label_name),
            operand_size: OperandSize::UNKNOWN,
            src_expanded: false,
            dst_expanded: false,
            src_regnumber: 0,
            dst_regnumber: 0,
            immediate_value: 0,
            load_offset: 0,
            store_offset: 0,
        }
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
