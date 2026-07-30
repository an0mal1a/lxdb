//! Public facade for the LXDB semantic dataset ecosystem.
//!
//! Most applications only need this crate. Lower-level crates remain public for
//! specialized integrations, while the facade provides a stable import path.

pub use lxdb_compiler as compiler;
pub use lxdb_core as core;
pub use lxdb_dictionary as dictionary;
pub use lxdb_engine as engine;
pub use lxdb_format as format;
pub use lxdb_storage as storage;

pub use lxdb_engine::{BinaryDatasetExt, DatasetQuery, EngineError};
pub use lxdb_storage::{BinaryDataset, DatasetReader, StorageError};
