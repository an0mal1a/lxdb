use crate::ids::{RelationId, TokenId};

use super::Weight;

/// Directed semantic relation between two tokens.
#[derive(Debug, Clone)]
pub struct Relation {
    pub id: RelationId,
    pub source: TokenId,
    pub target: TokenId,
    pub weight: Weight,
}