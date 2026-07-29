use crate::{graph::SemanticGraph, ids::TokenId};

/// Read-only view over a semantic graph.
///
/// Queries never modify the graph.
pub struct Query<'a> {
    pub graph: &'a SemanticGraph,
}

impl<'a> Query<'a> {
    pub fn new(graph: &'a SemanticGraph) -> Self {
        Self { graph }
    }

    pub fn contains(&self, token: TokenId) -> bool {
        self.graph.adjacency().contains(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        graph::SemanticGraph,
        ids::TokenId,
        storage::{AdjacencyEntry, AdjacencyList},
    };

    use super::Query;

    #[test]
    fn checks_token_membership_from_the_graph_adjacency() {
        let graph = SemanticGraph {
            tokens: Vec::new(),
            relations: Vec::new(),
            adjacency: AdjacencyList::new(vec![AdjacencyEntry::new(0, 0)]),
        };

        let query = Query::new(&graph);

        assert!(query.contains(TokenId::new(0)));
        assert!(!query.contains(TokenId::new(1)));
    }
}
