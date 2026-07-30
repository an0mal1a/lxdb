mod compile;
mod dictionary;
mod inspect;
mod query;

use std::path::Path;

use lxdb_storage::{BinaryDataset, DatasetReader};

use crate::error::CliError;

pub use compile::execute_compile;
pub use dictionary::{
    execute_dictionary_build, execute_dictionary_languages, execute_dictionary_update,
};
pub use inspect::execute_inspect;
pub use query::execute_query;

pub(super) fn open_dataset(path: &Path) -> Result<BinaryDataset, CliError> {
    DatasetReader::new().open(path).map_err(|source| CliError::OpenDataset {
        path: path.to_path_buf(),
        source: Box::new(source),
    })
}
