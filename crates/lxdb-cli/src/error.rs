use std::{error::Error, fmt, path::PathBuf};

use lxdb_engine::EngineError;

#[derive(Debug)]
pub enum CliError {
    Message(String),

    CompileDataset {
        source_path: PathBuf,
        output_path: PathBuf,
        source: Box<dyn Error + Send + Sync>,
    },

    OpenDataset {
        path: PathBuf,
        source: Box<dyn Error + Send + Sync>,
    },

    Query(EngineError),
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message(message) => write!(formatter, "{message}"),

            Self::CompileDataset { source_path, output_path, source } => {
                write!(
                    formatter,
                    "failed to compile '{}' into '{}': {source}",
                    source_path.display(),
                    output_path.display(),
                )
            }

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
            Self::CompileDataset { source, .. } | Self::OpenDataset { source, .. } => {
                Some(source.as_ref())
            }

            Self::Query(error) => Some(error),

            Self::Message(_) => None,
        }
    }
}

impl From<EngineError> for CliError {
    fn from(error: EngineError) -> Self {
        Self::Query(error)
    }
}
