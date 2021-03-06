use crate::compiler::ir::three_address_code::tac_kind::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ThreeAddressCode {
    pub kind: TacKind,
}

impl ThreeAddressCode {
    pub fn new(kind: TacKind) -> Self {
        Self { kind: kind }
    }
    pub fn new_goto(label_name: String) -> Self {
        Self::new(TacKind::GOTO(label_name))
    }
    pub fn new_label(label_name: String) -> Self {
        Self::new(TacKind::LABEL(label_name))
    }
    pub fn new_return(return_op: Operand) -> Self {
        Self::new(TacKind::RET(return_op))
    }
    pub fn new_assign_code(lv: Operand, rv: Operand) -> Self {
        Self::new(TacKind::ASSIGN(lv, rv))
    }
    pub fn new_iff(lv: Operand, label_name: String) -> Self {
        Self::new(TacKind::IFF(lv, label_name))
    }
    pub fn new_genparam(reg_num: usize, op: Operand) -> Self {
        Self::new(TacKind::GENPARAM(reg_num, op))
    }
    pub fn new_pushparam(reg_num: usize, offset: usize) -> Self {
        Self::new(TacKind::PUSHPARAM(reg_num, offset))
    }
    pub fn new_binop_code(
        variable_op: Operand,
        operator: Operator,
        left: Operand,
        right: Operand,
    ) -> Self {
        Self::new(TacKind::EXPR(variable_op, operator, left, right))
    }
    pub fn new_unop_code(variable_op: Operand, operator: Operator, inner: Operand) -> Self {
        Self::new(TacKind::UNARYEXPR(variable_op, operator, inner))
    }

    pub fn to_string(&self) -> String {
        match &self.kind {
            TacKind::LABEL(label_name) => format!("{}:", label_name),
            TacKind::GOTO(label_name) => format!("goto {}", label_name),
            TacKind::ASSIGN(lv, rv) => format!("{} <- {}", lv.to_string(), rv.to_string()),
            TacKind::IFF(lv, label) => format!("if false {} goto {}", lv.to_string(), label),
            TacKind::EXPR(var, op, left, right) => format!(
                "{} <- {} {} {}",
                var.to_string(),
                left.to_string(),
                op.to_string(),
                right.to_string()
            ),
            TacKind::UNARYEXPR(var, op, inner) => format!(
                "{} <- {} {}",
                var.to_string(),
                op.to_string(),
                inner.to_string(),
            ),
            TacKind::PUSHPARAM(number, offset) => format!("pushparam {}, {}", number, offset),
            TacKind::GENPARAM(reg_num, op) => format!("genparam {}, {}", reg_num, op.to_string()),
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
            _ => self.to_string(),
        }
    }
}
