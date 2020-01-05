pub mod translate;

// IR系定義
#[derive(Clone)]
pub struct X64Optimizer {
    // TODO: Vec<X64Function>,
    entry_bb: X64BasicBlock,
}
impl X64Optimizer {
    // TODO: Vec<X64Function> | Vec<X64Module>を受け取るように
    fn new(label: String, irs: Vec<X64IR>) -> Self {
        let entry_bb = X64BasicBlock::new(label, irs);
        Self { entry_bb: entry_bb }
    }
}
#[derive(Clone)]
pub struct X64BasicBlock {
    label: String,
    irs: Vec<X64IR>,
}
impl X64BasicBlock {
    fn new(label: String, irs: Vec<X64IR>) -> Self {
        Self {
            label: label,
            irs: irs,
        }
    }
}
#[derive(Clone)]
pub struct X64IR {
    kind: X64IRKind,
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
    pub fn new_ret(return_op: X64Operand) -> Self {
        Self {
            kind: X64IRKind::RET(return_op),
        }
    }
}

#[derive(Clone)]
pub enum X64IRKind {
    // 2つオペランドを持つ系
    MOV(X64Operand, X64Operand),
    ADD(X64Operand, X64Operand),

    // 1つオペランドを持つ系
    RET(X64Operand),
}

// アセンブリコード系定義
#[derive(Clone)]
pub struct X64Operand {
    pub kind: X64OpeKind,
    pub virt: usize,
    pub phys: usize,
}

impl X64Operand {
    fn new(kind: X64OpeKind, virt: usize, phys: usize) -> Self {
        Self {
            kind: kind,
            virt: virt,
            phys: phys,
        }
    }
    fn new_inv() -> Self {
        Self {
            kind: X64OpeKind::INVALID,
            virt: 0,
            phys: 0,
        }
    }
}

#[derive(Clone)]
pub enum X64OpeKind {
    INTLIT(i128),
    REG,
    INVALID,
}
