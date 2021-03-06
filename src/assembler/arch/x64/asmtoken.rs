use crate::assembler::arch::x64::inst::inst_name::X64InstName;
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
            AsmTokenKind::BLANK
            | AsmTokenKind::NEWLINE
            | AsmTokenKind::QWORD
            | AsmTokenKind::COMMA
            | AsmTokenKind::COMMENT
            | AsmTokenKind::PTR => true,
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
    MOVQ,  // movq命令
    ADDQ,  // addq命令
    CLTD,  // cltd命令
    SUBQ,  // subq命令
    IMULQ, // imulq命令
    IDIVQ, // idivq命令
    NEGQ,  // negq命令

    // intel記法
    NEG,  // neg命令
    MOV,  // mov命令
    ADD,  // add命令
    CMP,  // cmp命令
    CQO,  // cqo命令
    SUB,  // sub命令
    IMUL, // imul命令
    IDIV, // idiv命令

    // 汎用記法
    JMP,     // jmp命令
    JZ,      // jz命令
    CALL,    // call命令
    RET,     // ret命令
    SYSCALL, // syscall命令
    PUSH,    // push命令
    POP,     // pop命令

    // その他
    QWORD,
    PTR,
    COMMA,             // , 記号
    LBRACKET,          // [ 記号
    RBRACKET,          // [ 記号
    MINUS,             // - 記号
    LABEL(String),     // ラベル
    INTEGER(i128),     // 整数
    DIRECTIVE(String), // ディレクティブ
    BLANK,             // 空白類文字
    NEWLINE,           // 改行
    COMMENT,           // コメント
    EOF,
}

impl AsmTokenKind {
    pub fn to_inst_name(&self) -> X64InstName {
        match self {
            Self::ADD | Self::ADDQ => X64InstName::ADD,
            Self::SUB | Self::SUBQ => X64InstName::SUB,
            Self::JMP => X64InstName::JMP,
            Self::JZ => X64InstName::JZ,
            Self::NEG | Self::NEGQ => X64InstName::NEG,
            Self::IMUL | Self::IMULQ => X64InstName::IMUL,
            Self::IDIV | Self::IDIVQ => X64InstName::IDIV,
            Self::CALL => X64InstName::CALL,
            Self::SYSCALL => X64InstName::SYSCALL,
            Self::RET => X64InstName::RET,
            Self::CMP => X64InstName::CMP,
            Self::CQO | Self::CLTD => X64InstName::CQO,
            Self::MOV | Self::MOVQ => X64InstName::MOV,
            Self::PUSH => X64InstName::PUSH,
            Self::POP => X64InstName::POP,
            _ => panic!("can't translate to X64InstName"),
        }
    }
}
