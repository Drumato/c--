use crate::compiler::frontend::token::Position;
#[derive(Debug, PartialEq)]
pub struct Node {
    position: Position,
    kind: NodeKind,
    // TODO: ASTにつける型として type_kind: Type メンバを追加
}

impl Node {
    pub fn new(pos: Position, kind: NodeKind) -> Self {
        Self {
            position: pos,
            kind: kind,
        }
    }
}

type Operand = Box<Node>;
#[derive(Debug, PartialEq)]
pub enum NodeKind {
    ADD(Operand, Operand),
    INTEGER(i128),
    INVALID,
}
