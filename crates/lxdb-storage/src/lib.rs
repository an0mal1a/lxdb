mod dataset;
mod error;
mod reader;

pub use dataset::BinaryDataset;
pub use error::StorageError;
pub use reader::DatasetReader;

pub(crate) use dataset::SectionRange;
