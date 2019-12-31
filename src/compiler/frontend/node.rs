use crate::compiler::frontend::token::Position;
use crate::compiler::frontend::types::Type;
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub position: Position,
    pub kind: NodeKind,
    pub ctype: Type,
    // TODO: ASTにつける型として type_kind: Type メンバを追加
}

impl Node {
    pub fn new(pos: Position, kind: NodeKind) -> Self {
        Self {
            position: pos,
            kind: kind,
            ctype: Type::new_unknown(),
        }
    }
}

type Operand = Box<Node>;
#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    ADD(Operand, Operand),
    INTEGER(i128),
    INVALID,
}
