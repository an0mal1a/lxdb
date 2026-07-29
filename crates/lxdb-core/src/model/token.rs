use crate::ids::TokenId;

/// Represents a semantic token stored inside LXDB.
///
/// A token is the smallest textual unit addressable by the engine.
///
/// Tokens are immutable once compiled.
#[derive(Debug, Clone)]
pub struct Token {
    pub id: TokenId,
    pub text: String,
}