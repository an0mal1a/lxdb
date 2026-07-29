use crate::{
    ids::TokenId,
    model::{Relation, Token},
    storage::AdjacencyList,
    traversal::GraphTraversal,
};

/// Immutable semantic graph.
///
/// The graph owns every token and every relation.
///
/// Algorithms are implemented by higher level crates.
#[derive(Debug, Default)]
pub struct SemanticGraph {
    pub tokens: Vec<Token>,
    pub relations: Vec<Relation>,
    pub adjacency: AdjacencyList,
}

impl GraphTraversal for SemanticGraph {
    fn contains(&self, token_id: TokenId) -> bool {
        self.adjacency.contains(token_id)
    }

    fn outgoing(&self, token_id: TokenId) -> &[Relation] {
        let Some(entry) = self.adjacency.get(token_id) else {
            return &[];
        };

        &self.relations[entry.offset()..entry.end()]
    }
}
