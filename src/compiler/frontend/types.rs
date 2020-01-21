use crate::compiler::frontend::token::{Token, TokenKind};

#[derive(PartialEq, Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub byte_size: usize, // メモリ上のサイズ
}

impl Type {
    pub fn from_token(tk: Token) -> Self {
        match tk.kind {
            TokenKind::INT => Self::new_integer(),
            TokenKind::VOID => Self::new_void(),
            _ => panic!("can't translate {:?} to type", tk),
        }
    }
    pub fn new_void() -> Self {
        Self {
            kind: TypeKind::VOID,
            byte_size: 0,
        }
    }
    pub fn new_integer() -> Self {
        Self {
            kind: TypeKind::INTEGER,
            byte_size: 4,
        }
    }
    pub fn pointer_to(base: Self) -> Self {
        Self {
            kind: TypeKind::POINTER(Box::new(base)),
            byte_size: 8,
        }
    }
    pub fn new_unknown() -> Self {
        Self {
            kind: TypeKind::UNKNOWN,
            byte_size: 4,
        }
    }
}

type Base = Box<Type>;
#[derive(PartialEq, Debug, Clone)]
pub enum TypeKind {
    INTEGER,
    VOID,
    POINTER(Base),
    UNKNOWN,
}
