/// Points to the range of outgoing relations of a token.
///
/// Relations are stored contiguously inside the relation section.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AdjacencyRecord {
    /// First relation index.
    pub offset: u32,

    /// Number of outgoing relations.
    pub count: u32,
}