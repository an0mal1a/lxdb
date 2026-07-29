use crate::ids::TokenId;

/// Stores graph connectivity.
///
/// Given a TokenId, it provides access to its outgoing relations.
///
/// Internal representation is intentionally hidden.
#[derive(Debug, Default)]
pub struct AdjacencyList;