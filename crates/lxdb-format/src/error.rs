use std::{error::Error, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormatError {
    UnexpectedRecordSize { record: &'static str, expected: usize, found: usize },
    InvalidHeader,
}

impl fmt::Display for FormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedRecordSize { record, expected, found } => {
                write!(formatter, "invalid {record} size: expected {expected} bytes, found {found}",)
            }
            Self::InvalidHeader => {
                write!(formatter, "invalid or unsupported LXDB header")
            }
        }
    }
}

impl Error for FormatError {}
