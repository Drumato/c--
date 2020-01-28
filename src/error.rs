use crate::compiler::frontend::token::Position;

extern crate colored;
use colored::*;

pub struct Error {
    kind: ErrorKind,
    message: ErrorMsg,
    position: Position,
}

impl Error {
    pub fn new(kind: ErrorKind, pos: Position, msg: ErrorMsg) -> Self {
        Self {
            kind: kind,
            message: msg,
            position: pos,
        }
    }

    pub fn found_cant_support_architecture() {
        let err = Error::new(
            ErrorKind::Compile,
            (0, 0),
            ErrorMsg::CantSupportSuchAnArchitecture,
        );
        err.compile_error();
    }
    pub fn found(&self) {
        eprintln!(
            "[{}] at {}:{}: {}",
            self.kind.string().bold().red(),
            self.position.0,
            self.position.1,
            self.message.string(),
        );
    }
    pub fn compile_error(&self) {
        eprintln!(
            "[{}] {}",
            self.kind.string().bold().red(),
            self.message.string(),
        );
    }
}
pub enum ErrorKind {
    // コンパイラのエラー
    Parse,
    Type,
    GenIR,
    RegAlloc,
    Compile,

    // アセンブラのエラー
    AsmParse,
}

impl ErrorKind {
    fn string(&self) -> &str {
        match self {
            Self::Parse => "ParseError",
            Self::Type => "TypeError",
            Self::GenIR => "GenerateIRError",
            Self::RegAlloc => "RegisterAllocationError",
            Self::Compile => "CompileError",
            Self::AsmParse => "AssemblyParseError",
        }
    }
}

pub enum ErrorMsg {
    // コンパイラのエラー
    MustBePrimary,           // パーサがPrimaryを期待する場所でPrimaryではなかった.後
    InvalidNodeCantHaveType, // 意味解析器がInvalidなASTノードを確認した
    MustBeSameTypeInBinaryOperation, // 二項演算時,暗黙の型変換が適用されない組み合わせだった
    CantSupportSuchAnArchitecture, // 意図しないアーキテクチャ上でコンパイラが実行された
    CantUseNoMoreRegisters,  // レジスタ割付時エラー
    InvalidCFileOrDirectory, // ファイルが見つからない or ディレクトリであった

    // アセンブラのエラー
    MustBeIntegerLiteral, // Lexerが整数を期待する場所で整数ではなかった.
    InvalidOperand,       // 意図しないオペランドを受け取った
    MustSpecifySymbolNameInGlobalDirective, // .global <name> においてnameが見つからない
}

impl ErrorMsg {
    fn string(&self) -> &str {
        match self {
            // コンパイラのエラー
            Self::MustBePrimary => {
                "must be (identifier | constant-expr | paren-expr | string_literal | `_Generic`)"
            }
            Self::InvalidNodeCantHaveType => "invalid node can't have any types",
            Self::MustBeSameTypeInBinaryOperation => {
                "two expression must be same type in binary operation"
            }
            Self::CantSupportSuchAnArchitecture => "not supporting such an architecture yet",
            Self::CantUseNoMoreRegisters => "can't use no more registers",
            Self::InvalidCFileOrDirectory => "invalid c file or directory given",

            // アセンブラのエラー
            Self::MustBeIntegerLiteral => "must be integer-literal",
            Self::InvalidOperand => "invalid operand",
            Self::MustSpecifySymbolNameInGlobalDirective => {
                "must specify symbol name in global directive"
            }
        }
    }
}
