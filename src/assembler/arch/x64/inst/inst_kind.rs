type SrcOperand = X64Operand;
type DstOperand = X64Operand;
#[derive(PartialEq, Debug, Clone)]
pub enum X64InstKind {
    // オペランドを取らないもの
    NOOPERAND,
    // 1つオペランドを取るもの
    UNARY(X64Operand),
    // 2つオペランドを取るもの
    // AT&T記法の順番で格納.
    BINARY(SrcOperand, DstOperand),
    // ラベルを命令として持つと,後で処理しやすい.
    LABEL(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct X64Operand {
    pub kind: X64OpeKind,
}

impl X64Operand {
    pub fn new_label(name: String) -> Self {
        Self {
            kind: X64OpeKind::LABEL(name),
        }
    }
    pub fn new_register(name: String) -> Self {
        Self {
            kind: X64OpeKind::REG(name),
        }
    }
    pub fn new_integer(value: i128) -> Self {
        Self {
            kind: X64OpeKind::INTEGER(value),
        }
    }
    pub fn new_addressing(offset: i128, name: String) -> Self {
        Self {
            kind: X64OpeKind::ADDRESSING(offset, name),
        }
    }
    pub fn to_string(&self) -> String {
        match &self.kind {
            X64OpeKind::REG(name) => name.to_string(),
            X64OpeKind::INTEGER(val) => format!("{}", val),
            X64OpeKind::LABEL(name) => name.to_string(),
            X64OpeKind::ADDRESSING(offset, name) => format!("-{}[{}]", offset, name),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum X64OpeKind {
    // レジスタ
    REG(String),

    // 即値
    INTEGER(i128),

    // jump命令とか,ラベルをオペランドに持つ場合も
    LABEL(String),

    // メモリアドレッシング
    // 簡易実装なので,後々良くする.
    ADDRESSING(i128, String), // offset, RegisterName
}
