//! Reproducible, local-first linguistic dictionary builds for LXDB.
//!
//! The crate deliberately keeps linguistic acquisition separate from the LXDB
//! binary format.  Sources are parsed into [`LexicalEntry`] values, merged and
//! then rendered to the compiler's small `.lx` interchange format.

mod cache;
mod config;
mod error;
mod language;
mod manifest;
mod model;
mod pipeline;
mod report;
mod source;

pub use config::{BuildOptions, DictionaryProfile, SourceSelection};
pub use error::DictionaryError;
pub use language::{LANGUAGES, LanguageProfile, find_language};
pub use model::{
    LexicalEntry, LexicalRelation, LexicalRelationKind, SemanticQuality, SourceKind,
    SourceReference,
};
pub use pipeline::{build, inspect_manifest, update};
pub use report::BuildReport;
