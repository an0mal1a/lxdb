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

    pub fn contains(&self, _token: TokenId) -> bool {
        todo!()
    }
}