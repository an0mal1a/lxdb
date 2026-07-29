use crate::{
    ids::TokenId,
    model::Relation,
};

/// Read-only graph traversal interface.
pub trait GraphTraversal {
    fn contains(&self, token: TokenId) -> bool;

    fn outgoing(&self, token: TokenId) -> &[Relation];
}