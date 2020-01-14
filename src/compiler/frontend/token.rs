type Column = usize;
type Row = usize;
pub static GLOBAL_EOF_TOKEN: Token = Token {
    position: (0, 0),
    kind: TokenKind::EOF,
};
pub type Position = (Row, Column);
#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub position: Position,
    pub kind: TokenKind,
}

impl Token {
    pub fn should_ignore(&self) -> bool {
        match self.kind {
            TokenKind::BLANK | TokenKind::NEWLINE => true,
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

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    INTEGER(i128), // 整数
    PLUS,          // +記号
    MINUS,         // -記号
    SEMICOLON,     // ;記号
    BLANK,         // 空白類文字
    NEWLINE,       // 改行
    EOF,

    // 予約語
    RETURN, // return
}
