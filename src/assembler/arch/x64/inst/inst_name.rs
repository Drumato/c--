#[derive(PartialEq, Debug, Clone)]
pub enum X64InstName {
    // 抽象的なオペコード
    ADD,
    SUB,
    CALL,
    MOV,
    RET,
    SYSCALL,

    // 具体的なオペコード
    ADDRM64IMM32,
    ADDRM64R64,
    SUBRM64IMM32,
    SUBRM64R64,
    CALLRM64,
    MOVRM64IMM32,
    MOVRM64R64,
}
impl X64InstName {
    pub fn to_string(&self) -> String {
        match self {
            Self::ADD => "add".to_string(),
            Self::SUB => "sub".to_string(),
            Self::CALL => "call".to_string(),
            Self::MOV => "mov".to_string(),
            Self::RET => "ret".to_string(),
            Self::SYSCALL => "syscall".to_string(),
            Self::ADDRM64IMM32 => "add(r/m64 imm32)".to_string(),
            Self::ADDRM64R64 => "add(r/m64 r64)".to_string(),
            Self::SUBRM64IMM32 => "sub(r/m64 imm32)".to_string(),
            Self::SUBRM64R64 => "sub(r/m64 r64)".to_string(),
            Self::CALLRM64 => "call(r/m64)".to_string(),
            Self::MOVRM64IMM32 => "mov(r/m64 imm32)".to_string(),
            Self::MOVRM64R64 => "mov(r/m64 r64)".to_string(),
        }
    }
}
