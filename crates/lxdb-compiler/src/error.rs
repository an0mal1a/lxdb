use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io,
};

/// Errors produced while compiling an LXDB dataset.
#[derive(Debug)]
pub enum CompilerError {
    InvalidWeight,
    EmptyToken,
    SelfReference,
    DuplicateToken(String),
    UnknownToken(String),
    MissingInput,
    Io(io::Error),
    InvalidSyntax { line: usize, content: String },
}

impl Display for CompilerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidWeight => {
                write!(formatter, "relation weight must be between 0.0 and 1.0")
            }
            Self::EmptyToken => {
                write!(formatter, "token text cannot be empty")
            }
            Self::SelfReference => {
                write!(formatter, "a token cannot reference itself")
            }
            Self::DuplicateToken(token) => {
                write!(formatter, "duplicate token: {token}")
            }
            Self::UnknownToken(token) => {
                write!(formatter, "unknown token: {token}")
            }
            Self::MissingInput => {
                write!(formatter, "no input file was provided")
            }
            Self::Io(error) => {
                write!(formatter, "input/output error: {error}")
            }
            Self::InvalidSyntax { line, content } => {
                write!(formatter, "invalid syntax at line {line}: {content}")
            }
        }
    }
}

impl Error for CompilerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for CompilerError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
