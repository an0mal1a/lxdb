use std::{error::Error, fmt, path::PathBuf};

use lxdb_engine::EngineError;

#[derive(Debug)]
pub enum CliError {
    OpenDataset { path: PathBuf, source: Box<dyn Error + Send + Sync> },

    Query(EngineError),
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenDataset { path, source } => {
                write!(formatter, "failed to open dataset '{}': {source}", path.display(),)
            }

            Self::Query(error) => {
                write!(formatter, "dataset query failed: {error}",)
            }
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::OpenDataset { source, .. } => Some(source.as_ref()),

            Self::Query(error) => Some(error),
        }
    }
}

impl From<EngineError> for CliError {
    fn from(error: EngineError) -> Self {
        Self::Query(error)
    }
}
