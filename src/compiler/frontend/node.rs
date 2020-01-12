use crate::compiler::frontend::token::{Position, Token, TokenKind};
use crate::compiler::frontend::types::Type;
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub position: Position,
    pub kind: NodeKind,
    pub ctype: Type,
}

impl Node {
    pub fn new(pos: Position, kind: NodeKind) -> Self {
        Self {
            position: pos,
            kind: kind,
            ctype: Type::new_unknown(),
        }
    }
    pub fn new_binary_node(tok: &Token, left: Node, right: Node) -> Self {
        let node_kind = match tok.kind {
            TokenKind::PLUS => NodeKind::ADD(Box::new(left), Box::new(right)),
            TokenKind::MINUS => NodeKind::SUB(Box::new(left), Box::new(right)),
            _ => panic!("not found such an operator"),
        };
        Node::new(tok.position, node_kind)
    }
}

type Operand = Box<Node>;
#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    ADD(Operand, Operand),
    SUB(Operand, Operand),
    INTEGER(i128),
    INVALID,
}

// 演算の優先順位を定義
#[derive(Debug, PartialEq, Clone)]
pub enum Priority {
    ADDSUB,
}
