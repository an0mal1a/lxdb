use crate::graph::SemanticGraph;

use super::Metadata;

/// Immutable LXDB dataset.
#[derive(Debug)]
pub struct Dataset {
    pub metadata: Metadata,
    pub graph: SemanticGraph,
}