#[allow(dead_code)]
pub struct Target {
    pub os: OS,
    pub arch: Architecture,
}

impl Target {
    pub fn new() -> Self {
        Self {
            os: OS::new(),
            arch: Architecture::new(),
        }
    }
}

pub enum OS {
    Linux,
    INVALID,
}
impl OS {
    fn new() -> Self {
        if cfg!(target_os = "linux") {
            Self::Linux
        } else {
            Self::INVALID
        }
    }
}

pub enum Architecture {
    X86_64,
    INVALID,
}

impl Architecture {
    fn new() -> Self {
        if cfg!(target_arch = "x86_64") {
            Self::X86_64
        } else {
            Self::INVALID
        }
    }
}
