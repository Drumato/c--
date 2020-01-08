type Column = usize;
type Row = usize;

pub type Position = (Column, Row);

#[derive(PartialEq, Debug, Clone)]
pub struct AsmToken {
    pub position: Position,
    pub kind: AsmTokenKind,
}

impl AsmToken {
    pub fn should_ignore(&self) -> bool {
        match self.kind {
            AsmTokenKind::BLANK | AsmTokenKind::NEWLINE => true,
            _ => false,
        }
    }
    pub fn new(position: Position, kind: AsmTokenKind) -> Self {
        Self {
            position: position,
            kind: kind,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum AsmTokenKind {
    // レジスタ
    REG(String),

    // 命令
    // AT&T記法
    MOVQ, // movq命令
    ADDQ, // movq命令

    // intel記法
    MOV, // mov命令
    ADD, // add命令

    // 汎用記法
    RET, // ret命令

    // その他
    LABEL(String),     // ラベル
    INTEGER(i128),     // 整数
    DIRECTIVE(String), // ディレクティブ
    BLANK,             // 空白類文字
    NEWLINE,           // 改行
    EOF,
}
