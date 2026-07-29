/// Binary sections contained inside an LXDB file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Section {
    Metadata = 1,
    Tokens = 2,
    Relations = 3,
    Adjacency = 4,
}
