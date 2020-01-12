use crate::compiler::ir::arch::x64::ir_kind;
use ir_kind::{X64IRKind, X64Operand};

#[derive(Clone)]
pub struct X64IR {
    pub kind: X64IRKind,
}

impl X64IR {
    pub fn new_mov(dst: X64Operand, src: X64Operand) -> Self {
        Self {
            kind: X64IRKind::MOV(dst, src),
        }
    }
    pub fn new_add(dst: X64Operand, src: X64Operand) -> Self {
        Self {
            kind: X64IRKind::ADD(dst, src),
        }
    }
    pub fn new_sub(dst: X64Operand, src: X64Operand) -> Self {
        Self {
            kind: X64IRKind::SUB(dst, src),
        }
    }
    pub fn new_ret(return_op: X64Operand) -> Self {
        Self {
            kind: X64IRKind::RET(return_op),
        }
    }
}
