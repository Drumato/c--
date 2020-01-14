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
    pub fn new_return(pos: Position, child: Node) -> Self {
        Self::new(pos, NodeKind::RETURNSTMT(Box::new(child)))
    }
    pub fn new_binary_node(tok: &Token, left: Node, right: Node) -> Self {
        let node_kind = match tok.kind {
            TokenKind::PLUS => NodeKind::ADD(Box::new(left), Box::new(right)),
            TokenKind::MINUS => NodeKind::SUB(Box::new(left), Box::new(right)),
            _ => panic!("not found such an operator"),
        };
        Self::new(tok.position, node_kind)
    }
}

type Child = Box<Node>;
#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    // statement
    RETURNSTMT(Child),

    // expression
    ADD(Child, Child),
    SUB(Child, Child),
    INTEGER(i128),
    INVALID,
}

// 演算の優先順位を定義
#[derive(Debug, PartialEq, Clone)]
pub enum Priority {
    ADDSUB,
}
