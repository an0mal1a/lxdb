use std::{fmt, io, path::PathBuf};

#[derive(Debug)]
pub enum DictionaryError {
    UnsupportedLanguage(String),
    InvalidProfile(String),
    InvalidConfiguration { path: PathBuf, message: String },
    MissingSource { source: &'static str, path: PathBuf },
    OfflineCacheMiss { source: &'static str, path: PathBuf },
    InvalidSource { source: &'static str, path: PathBuf, line: usize, message: String },
    Io(io::Error),
    Compile(lxdb_compiler::error::CompilerError),
    Validate(lxdb_storage::StorageError),
    Manifest(PathBuf),
}

impl fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLanguage(language) => write!(f, "unsupported language: {language}"),
            Self::InvalidProfile(profile) => write!(f, "unsupported dictionary profile: {profile}"),
            Self::InvalidConfiguration { path, message } => {
                write!(f, "invalid dictionary configuration '{}': {message}", path.display())
            }
            Self::MissingSource { source, path } => {
                write!(f, "{source} source is unavailable: {}", path.display())
            }
            Self::OfflineCacheMiss { source, path } => {
                write!(f, "offline cache miss for {source}: {}", path.display())
            }
            Self::InvalidSource { source, path, line, message } => {
                write!(f, "invalid {source} source at {}:{}: {message}", path.display(), line)
            }
            Self::Io(error) => write!(f, "I/O error: {error}"),
            Self::Compile(error) => write!(f, "LXDB compilation failed: {error}"),
            Self::Validate(error) => write!(f, "generated dataset is invalid: {error}"),
            Self::Manifest(path) => write!(f, "invalid dictionary manifest: {}", path.display()),
        }
    }
}

impl std::error::Error for DictionaryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Compile(error) => Some(error),
            Self::Validate(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for DictionaryError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
