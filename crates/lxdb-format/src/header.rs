use crate::{magic::MAGIC, version::VERSION};

/// Header present at the beginning of every LXDB file.
#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub magic: [u8; 4],
    pub version: u16,
}

impl Header {
    pub const fn new() -> Self {
        Self { magic: MAGIC, version: VERSION }
    }
}
