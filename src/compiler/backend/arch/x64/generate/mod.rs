pub mod at_and_t;
pub mod intel;

pub enum Registers {
    RAX,
    RCX,
    RDX,
    RBX,
    RSP,
    RBP,
    RSI,
    RDI,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[allow(dead_code)]
impl Registers {
    fn from_number_ir(num: usize) -> Self {
        match num {
            0 => Self::RDI,
            1 => Self::RSI,
            2 => Self::RDX,
            3 => Self::RCX,
            4 => Self::R8,
            5 => Self::R9,
            6 => Self::R10,
            7 => Self::R11,
            8 => Self::R12,
            9 => Self::R13,
            10 => Self::R14,
            11 => Self::R15,
            12 => Self::RAX,
            13 => Self::RBX,
            _ => {
                eprintln!("can't use rsp and rbp!");
                Self::RDI
            }
        }
    }
    // 本来の順番
    fn from_number(num: usize) -> Self {
        match num {
            0 => Self::RAX,
            1 => Self::RCX,
            2 => Self::RDX,
            3 => Self::RBX,
            4 => Self::RSP,
            5 => Self::RBP,
            6 => Self::RSI,
            7 => Self::RDI,
            8 => Self::R8,
            9 => Self::R9,
            10 => Self::R10,
            11 => Self::R11,
            12 => Self::R12,
            13 => Self::R13,
            14 => Self::R14,
            15 => Self::R15,
            _ => {
                eprintln!("can't over 16!");
                Self::RAX
            }
        }
    }
    fn to_string(&self) -> String {
        match self {
            Self::RAX => "rax",
            Self::RCX => "rcx",
            Self::RDX => "rdx",
            Self::RBX => "rbx",
            Self::RSP => "rsp",
            Self::RBP => "rbp",
            Self::RSI => "rsi",
            Self::RDI => "rdi",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
        }
        .to_string()
    }
}
