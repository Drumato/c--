#[derive(PartialEq, Debug, Clone)]
pub enum X64InstName {
    // 抽象的なオペコード
    ADD,
    SUB,
    IMUL,
    IDIV,
    CQO,
    CALL,
    JMP,
    MOV,
    RET,
    SYSCALL,

    // 具体的なオペコード
    ADDRM64IMM32,
    ADDRM64R64,
    SUBRM64IMM32,
    SUBRM64R64,
    IMULR64RM64IMM32,
    IMULR64RM64,
    CALLRM64,
    IDIVRM64,
    MOVRM64IMM32,
    MOVRM64R64,
    JMPREL32,

    // その他
    LABEL,
}
impl X64InstName {
    pub fn to_string(&self) -> String {
        match self {
            Self::ADD => "add".to_string(),
            Self::SUB => "sub".to_string(),
            Self::IMUL => "imul".to_string(),
            Self::IDIV => "idiv".to_string(),
            Self::CALL => "call".to_string(),
            Self::CQO => "cqo".to_string(),
            Self::JMP => "jmp".to_string(),
            Self::MOV => "mov".to_string(),
            Self::RET => "ret".to_string(),
            Self::SYSCALL => "syscall".to_string(),
            Self::ADDRM64IMM32 => "add(r/m64 imm32)".to_string(),
            Self::ADDRM64R64 => "add(r/m64 r64)".to_string(),
            Self::SUBRM64IMM32 => "sub(r/m64 imm32)".to_string(),
            Self::SUBRM64R64 => "sub(r/m64 r64)".to_string(),
            Self::IMULR64RM64IMM32 => "imul(r64 r/m64 imm32)".to_string(),
            Self::IMULR64RM64 => "imul(r64 r/m64)".to_string(),
            Self::CALLRM64 => "call(r/m64)".to_string(),
            Self::IDIVRM64 => "idiv(r/m64)".to_string(),
            Self::MOVRM64IMM32 => "mov(r/m64 imm32)".to_string(),
            Self::MOVRM64R64 => "mov(r/m64 r64)".to_string(),
            Self::JMPREL32 => "jmp (rel32)".to_string(),
            Self::LABEL => "label".to_string(),
        }
    }
}
