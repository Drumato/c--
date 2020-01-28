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
    PUSH,
    POP,
    NEG,

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
    MOVR64RM64,
    MOVRM64R64,
    JMPREL32,
    PUSHR64,
    POPR64,
    NEGRM64,

    // その他
    LABEL,
}
impl X64InstName {
    pub fn to_string(&self) -> String {
        match self {
            // add
            Self::ADD => "add".to_string(),
            Self::ADDRM64IMM32 => "add(r/m64 imm32)".to_string(),
            Self::ADDRM64R64 => "add(r/m64 r64)".to_string(),
            // sub
            Self::SUB => "sub".to_string(),
            Self::SUBRM64IMM32 => "sub(r/m64 imm32)".to_string(),
            Self::SUBRM64R64 => "sub(r/m64 r64)".to_string(),
            // imul
            Self::IMUL => "imul".to_string(),
            Self::IMULR64RM64IMM32 => "imul(r64 r/m64 imm32)".to_string(),
            Self::IMULR64RM64 => "imul(r64 r/m64)".to_string(),
            // idiv
            Self::IDIV => "idiv".to_string(),
            Self::IDIVRM64 => "idiv(r/m64)".to_string(),
            // call
            Self::CALL => "call".to_string(),
            Self::CALLRM64 => "call(r/m64)".to_string(),
            // cqo
            Self::CQO => "cqo".to_string(),
            // jmp
            Self::JMP => "jmp".to_string(),
            Self::JMPREL32 => "jmp (rel32)".to_string(),
            // neg
            Self::NEG => "neg".to_string(),
            Self::NEGRM64 => "neg (r/m64)".to_string(),
            // push
            Self::PUSH => "push".to_string(),
            Self::PUSHR64 => "push (r64)".to_string(),
            // pop
            Self::POP => "pop".to_string(),
            Self::POPR64 => "pop (r64)".to_string(),
            // mov
            Self::MOV => "mov".to_string(),
            Self::MOVR64RM64 => "mov(r64 r/m64)".to_string(),
            Self::MOVRM64IMM32 => "mov(r/m64 imm32)".to_string(),
            Self::MOVRM64R64 => "mov(r/m64 r64)".to_string(),

            // ret
            Self::RET => "ret".to_string(),
            // syscall
            Self::SYSCALL => "syscall".to_string(),
            // label
            Self::LABEL => "label".to_string(),
        }
    }
}
