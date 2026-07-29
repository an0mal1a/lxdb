use super::Metadata;
use crate::graph::SemanticGraph;

/// Immutable LXDB dataset.
#[derive(Debug)]
pub struct Dataset {
    metadata: Metadata,
    graph: SemanticGraph,
}

impl Dataset {
    pub fn new(metadata: Metadata, graph: SemanticGraph) -> Self {
        Self { metadata, graph }
    }

    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub const fn graph(&self) -> &SemanticGraph {
        &self.graph
    }
}
