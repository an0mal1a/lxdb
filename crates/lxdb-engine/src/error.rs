use std::{error::Error, fmt, str::Utf8Error};

use lxdb_format::FormatError;

#[derive(Debug)]
pub enum EngineError {
    Format(FormatError),

    TokenStringOutOfBounds { token_id: u32, offset: u64, length: u32, table_length: usize },

    InvalidTokenUtf8 { token_id: u32, source: Utf8Error },
}

impl fmt::Display for EngineError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Format(error) => {
                write!(formatter, "failed to decode binary record: {error}")
            }

            Self::TokenStringOutOfBounds { token_id, offset, length, table_length } => {
                write!(
                    formatter,
                    "token {token_id} references string range \
                     {offset}..{} outside a table of {table_length} bytes",
                    offset.saturating_add(u64::from(*length)),
                )
            }

            Self::InvalidTokenUtf8 { token_id, source } => {
                write!(formatter, "token {token_id} contains invalid UTF-8: {source}",)
            }
        }
    }
}

impl Error for EngineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Format(error) => Some(error),

            Self::InvalidTokenUtf8 { source, .. } => Some(source),

            Self::TokenStringOutOfBounds { .. } => None,
        }
    }
}

impl From<FormatError> for EngineError {
    fn from(error: FormatError) -> Self {
        Self::Format(error)
    }
}
