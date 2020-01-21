use crate::compiler::frontend::token::{Position, Token, TokenKind};
use crate::compiler::frontend::types::Type;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: String,

    pub def_position: Position,
    // args
    // pub args: BTreeMap<String, Node>,
    pub stmts: Vec<Node>,

    pub frame_size: usize,
}

impl Function {
    pub fn init(name: String, pos: Position) -> Self {
        Self {
            name: name,
            def_position: pos,
            stmts: Vec::new(),
            frame_size: 0,
        }
    }
}

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
    pub fn new_labeled(pos: Position, label_name: String, stmt: Node) -> Self {
        Self::new(pos, NodeKind::LABELEDSTMT(label_name, Box::new(stmt)))
    }
    pub fn new_goto(pos: Position, label_name: String) -> Self {
        Self::new(pos, NodeKind::GOTOSTMT(label_name))
    }
    pub fn new_exprstmt(pos: Position, expr: Node) -> Self {
        Self::new(pos, NodeKind::EXPRSTMT(Box::new(expr)))
    }
    pub fn new_declaration(pos: Position, name: String, ty: Type) -> Self {
        Self::new(pos, NodeKind::DECLARATION(name, ty))
    }
    pub fn new_assign(pos: Position, lvalue: Node, rvalue: Node) -> Self {
        Self::new(pos, NodeKind::ASSIGN(Box::new(lvalue), Box::new(rvalue)))
    }
    pub fn new_return(pos: Position, expr: Node) -> Self {
        Self::new(pos, NodeKind::RETURNSTMT(Box::new(expr)))
    }
    pub fn new_binary_node(tok: &Token, left: Node, right: Node) -> Self {
        let node_kind = match tok.kind {
            TokenKind::PLUS => NodeKind::ADD(Box::new(left), Box::new(right)),
            TokenKind::MINUS => NodeKind::SUB(Box::new(left), Box::new(right)),
            TokenKind::ASTERISK => NodeKind::MUL(Box::new(left), Box::new(right)),
            TokenKind::SLASH => NodeKind::DIV(Box::new(left), Box::new(right)),
            _ => panic!("not found such an operator"),
        };
        Self::new(tok.position, node_kind)
    }
}

type Expr = Box<Node>;
type Stmt = Box<Node>;
type Label = String;
#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    // statement
    RETURNSTMT(Expr),
    GOTOSTMT(Label),
    LABELEDSTMT(Label, Stmt),
    EXPRSTMT(Expr),
    DECLARATION(String, Type),

    // expression
    ASSIGN(Expr, Expr),
    ADD(Expr, Expr),
    SUB(Expr, Expr),
    MUL(Expr, Expr),
    DIV(Expr, Expr),
    INTEGER(i128),
    IDENTIFIER(String),
    INVALID,
}

// 演算の優先順位を定義
#[derive(Debug, PartialEq, Clone)]
pub enum Priority {
    ADDITIVE,
    MULTIPLICATIVE,
}
