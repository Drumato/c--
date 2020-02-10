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
    INTEGER(i128),      // 整数
    IDENTIFIER(String), // 識別子

    // 記号
    PLUS,      // +記号
    MINUS,     // -記号
    ASTERISK,  // *記号
    SLASH,     // /記号
    LPAREN,    // (記号
    RPAREN,    // )記号
    LBRACKET,  // {記号
    RBRACKET,  // }記号
    COLON,     // :記号
    SEMICOLON, // ;記号
    ASSIGN,    // =記号
    BLANK,     // 空白類文字
    NEWLINE,   // 改行
    EOF,

    // 予約語
    IF,     // if
    ELSE,   // else
    FOR,    // for
    INT,    // int
    GOTO,   // goto
    VOID,   // void
    RETURN, // return
}
