use crate::compiler::ir::arch::x64::ir_kind;
use ir_kind::{X64IRKind, X64Operand};

#[derive(Debug, Clone)]
pub struct X64IR {
    pub kind: X64IRKind,
}

impl X64IR {
    pub fn new_jump(label_name: String) -> Self {
        Self {
            kind: X64IRKind::JMP(label_name),
        }
    }
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
    pub fn new_mul(dst: X64Operand, src: X64Operand) -> Self {
        Self {
            kind: X64IRKind::MUL(dst, src),
        }
    }
    pub fn new_store(dst: X64Operand, src: X64Operand) -> Self {
        Self {
            kind: X64IRKind::STORE(dst, src),
        }
    }
    pub fn new_div(dst: X64Operand, src: X64Operand) -> Self {
        Self {
            kind: X64IRKind::DIV(dst, src),
        }
    }
    pub fn new_neg(inner: X64Operand) -> Self {
        Self {
            kind: X64IRKind::NEGATIVE(inner),
        }
    }
    pub fn new_ret(return_op: X64Operand) -> Self {
        Self {
            kind: X64IRKind::RET(return_op),
        }
    }
    pub fn new_cmpzero(cmp_op: X64Operand) -> Self {
        Self {
            kind: X64IRKind::CMPZERO(cmp_op),
        }
    }
    pub fn new_jumpzero(label_name: String) -> Self {
        Self {
            kind: X64IRKind::JZ(label_name),
        }
    }
    pub fn new_genparam(reg_num: usize, op: X64Operand) -> Self {
        Self {
            kind: X64IRKind::GENPARAM(reg_num, op),
        }
    }
    pub fn new_pushparam(reg_num: usize, offset: usize) -> Self {
        Self {
            kind: X64IRKind::PUSHPARAM(reg_num, offset),
        }
    }
}
