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
    pub fn dump_ast(&self) {
        eprintln!("function {}() {{ ", self.name);
        for st in self.stmts.iter() {
            eprintln!("  {}", st.to_string());
        }
        eprintln!("}}");
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
    pub fn new_nop() -> Self {
        Self::new((0, 0), NodeKind::NOP)
    }
    pub fn new_labeled(pos: Position, label_name: String, stmt: Node) -> Self {
        Self::new(pos, NodeKind::LABELEDSTMT(label_name, Box::new(stmt)))
    }
    pub fn new_if(pos: Position, cond_expr: Node, stmt: Node) -> Self {
        Self::new(pos, NodeKind::IFSTMT(Box::new(cond_expr), Box::new(stmt)))
    }
    pub fn new_if_else(pos: Position, cond_expr: Node, stmt: Node, alt: Node) -> Self {
        Self::new(
            pos,
            NodeKind::IFELSESTMT(Box::new(cond_expr), Box::new(stmt), Box::new(alt)),
        )
    }
    pub fn new_for(pos: Position, clause: Node, expr_2: Node, expr_3: Node, stmt: Node) -> Self {
        Self::new(
            pos,
            NodeKind::FORSTMT(
                Box::new(clause),
                Box::new(expr_2),
                Box::new(expr_3),
                Box::new(stmt),
            ),
        )
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
    pub fn new_unary_node(tok: &Token, inner: Node) -> Self {
        let node_kind = match tok.kind {
            TokenKind::MINUS => NodeKind::NEGATIVE(Box::new(inner)),
            _ => panic!("not found such an operator"),
        };
        Self::new(tok.position, node_kind)
    }

    pub fn to_string(&self) -> String {
        match &self.kind {
            // statement
            NodeKind::RETURNSTMT(expr) => format!("return {};", expr.to_string()),
            NodeKind::GOTOSTMT(label) => format!("goto {};", label),
            NodeKind::LABELEDSTMT(label, st) => format!("{}: {}", label, st.to_string()),
            NodeKind::EXPRSTMT(expr) => format!("{};", expr.to_string()),
            NodeKind::FORSTMT(clause, expr_2, expr_3, stmt) => format!(
                "for ( {}; {}; {} )\n    {}",
                clause.to_string(),
                expr_2.to_string(),
                expr_3.to_string(),
                stmt.to_string()
            ),
            NodeKind::IFSTMT(expr, stmt) => {
                format!("if ( {} ) {}", expr.to_string(), stmt.to_string())
            }
            NodeKind::IFELSESTMT(expr, stmt, alt) => format!(
                "if ( {} ) {}\n  else {}",
                expr.to_string(),
                stmt.to_string(),
                alt.to_string()
            ),
            NodeKind::DECLARATION(name, ty) => format!("{} {};", ty.to_string(), name),

            // expression
            NodeKind::ASSIGN(lv, rv) => format!("{} = {}", lv.to_string(), rv.to_string()),
            NodeKind::ADD(left, right) => format!("{} + {}", left.to_string(), right.to_string()),
            NodeKind::SUB(left, right) => format!("{} - {}", left.to_string(), right.to_string()),
            NodeKind::MUL(left, right) => format!("{} * {}", left.to_string(), right.to_string()),
            NodeKind::DIV(left, right) => format!("{} / {}", left.to_string(), right.to_string()),
            NodeKind::NEGATIVE(ex) => format!("- {}", ex.to_string()),
            NodeKind::INTEGER(v) => format!("{}", v),
            NodeKind::IDENTIFIER(name) => name.to_string(),
            NodeKind::INVALID => "invalid".to_string(),
            NodeKind::NOP => "nop".to_string(),
        }
    }
}

type Clause = Box<Node>;
type Expr = Box<Node>;
type Stmt = Box<Node>;
type Label = String;
#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    // statement
    RETURNSTMT(Expr),
    GOTOSTMT(Label),
    IFSTMT(Expr, Stmt),
    IFELSESTMT(Expr, Stmt, Stmt),
    FORSTMT(Clause, Expr, Expr, Stmt),
    LABELEDSTMT(Label, Stmt),
    EXPRSTMT(Expr),
    DECLARATION(String, Type),

    // expression
    ASSIGN(Expr, Expr),
    ADD(Expr, Expr),
    SUB(Expr, Expr),
    MUL(Expr, Expr),
    DIV(Expr, Expr),
    NEGATIVE(Expr),
    INTEGER(i128),
    IDENTIFIER(String),
    INVALID,
    NOP,
}

// 演算の優先順位を定義
#[derive(Debug, PartialEq, Clone)]
pub enum Priority {
    ADDITIVE,
    MULTIPLICATIVE,
}
