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
    pub fn is_x86_64(&self) -> bool {
        if let Architecture::X86_64 = self.arch {
            return true;
        }
        false
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
