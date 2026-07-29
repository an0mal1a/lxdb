pub mod flags;
pub mod header;
pub mod magic;
pub mod section;
pub mod section_header;
pub mod version;

pub mod adjacency_record;
pub mod relation_record;
pub mod token_record;

pub use header::{HEADER_SIZE, Header};
pub use magic::MAGIC;
pub use version::Version;
