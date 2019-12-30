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

    pub fn found(&self) {
        eprintln!(
            "[{}] at {}:{}: {}",
            self.kind.string().bold().red(),
            self.position.0,
            self.position.1,
            self.message.string(),
        );
    }
}
pub enum ErrorKind {
    Parse,
}

impl ErrorKind {
    fn string(&self) -> &str {
        match self {
            Self::Parse => "ParseError",
        }
    }
}

pub enum ErrorMsg {
    MustBeInteger,
}

impl ErrorMsg {
    fn string(&self) -> &str {
        match self {
            Self::MustBeInteger => "must be integer",
        }
    }
}
