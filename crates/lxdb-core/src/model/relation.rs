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

impl Relation {
    pub fn new(
        id: RelationId,
        source: TokenId,
        target: TokenId,
        weight: Weight,
    ) -> Self {
        Self {
            id,
            source,
            target,
            weight,
        }
    }

    pub const fn id(&self) -> RelationId {
        self.id
    }

    pub const fn source(&self) -> TokenId {
        self.source
    }

    pub const fn target(&self) -> TokenId {
        self.target
    }

    pub const fn weight(&self) -> Weight {
        self.weight
    }
}