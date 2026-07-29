/// Binary representation of a token.
///
/// The token text itself is stored in the token string table.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TokenRecord {
    pub id: u32,
    pub offset: u32,
    pub length: u32,
}
