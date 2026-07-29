use std::{error::Error, fmt, io};

/// Errors produced while opening or parsing an LXDB dataset.
#[derive(Debug)]
pub enum StorageError {
    Io(io::Error),

    InvalidHeader,

    TruncatedSectionHeader { offset: usize },

    UnknownSection { section_type: u8, offset: usize },

    UnsupportedSectionFlags { section_type: u8, flags: u8 },

    SectionOutOfBounds { section_type: u8, offset: usize, length: u64 },

    DuplicateSection { section_type: u8 },

    MissingSection { section_type: u8 },

    InvalidSectionLength { section_type: u8, length: usize, record_size: usize },
}

impl fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => {
                write!(formatter, "failed to access LXDB dataset: {error}")
            }

            Self::InvalidHeader => {
                write!(formatter, "invalid or unsupported LXDB header")
            }

            Self::TruncatedSectionHeader { offset } => {
                write!(formatter, "truncated section header at byte offset {offset}",)
            }

            Self::UnknownSection { section_type, offset } => {
                write!(formatter, "unknown section type {section_type} at byte offset {offset}",)
            }

            Self::UnsupportedSectionFlags { section_type, flags } => {
                write!(
                    formatter,
                    "section type {section_type} uses unsupported flags {flags:#010b}",
                )
            }

            Self::SectionOutOfBounds { section_type, offset, length } => {
                write!(
                    formatter,
                    "section type {section_type} at offset {offset} declares \
                     an out-of-bounds payload of {length} bytes",
                )
            }

            Self::DuplicateSection { section_type } => {
                write!(formatter, "dataset contains duplicate section type {section_type}",)
            }

            Self::MissingSection { section_type } => {
                write!(formatter, "dataset is missing required section type {section_type}",)
            }

            Self::InvalidSectionLength { section_type, length, record_size } => {
                write!(
                    formatter,
                    "section type {section_type} has length {length}, \
                     which is not divisible by record size {record_size}",
                )
            }
        }
    }
}

impl Error for StorageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for StorageError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
