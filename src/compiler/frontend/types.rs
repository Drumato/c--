#[derive(PartialEq, Debug, Clone)]
pub struct Type {
    kind: TypeKind,
    byte_size: usize, // メモリ上のサイズ
}

impl Type {
    pub fn new_integer() -> Self {
        Self {
            kind: TypeKind::INTEGER,
            byte_size: 4,
        }
    }
    pub fn new_unknown() -> Self {
        Self {
            kind: TypeKind::UNKNOWN,
            byte_size: 4,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeKind {
    INTEGER,
    UNKNOWN,
}
