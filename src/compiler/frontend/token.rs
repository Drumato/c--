type Column = usize;
type Row = usize;
pub type Position = (Column, Row);
#[derive(PartialEq, Debug)]
pub struct Token {
    pub position: Position,
    pub kind: TokenKind,
}

impl Token {
    pub fn should_ignore(&self) -> bool {
        match self.kind {
            _ => false,
        }
    }
    pub fn new(position: Position, kind: TokenKind) -> Self {
        Self {
            position: position,
            kind: kind,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    INTEGER(i128), // 整数
    EOF,
}
