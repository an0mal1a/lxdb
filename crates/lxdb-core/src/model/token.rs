use crate::ids::TokenId;

/// Represents a semantic token stored inside LXDB.
///
/// A token is the smallest textual unit addressable by the engine.
///
/// Tokens are immutable once compiled.
#[derive(Debug, Clone)]
pub struct Token {
    id: TokenId,
    text: Box<str>,
}

impl Token {
    pub fn new(id: TokenId, text: Box<str>) -> Self {
        Self { id, text }
    }

    pub const fn id(&self) -> TokenId {
        self.id
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}