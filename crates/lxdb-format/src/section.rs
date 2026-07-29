/// Identifies a section inside an LXDB binary file.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Metadata = 1,
    Tokens = 2,
    TokenStringTable = 3,
    Relations = 4,
    Adjacency = 5,
}

impl Section {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}
