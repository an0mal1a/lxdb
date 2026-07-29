use lxdb_core::graph::SemanticGraph;

/// Encodes a semantic graph into the LXDB binary format.
#[derive(Debug, Default)]
pub struct Writer;

impl Writer {
    pub const fn new() -> Self {
        Self
    }

    pub fn encode(&self, graph: &SemanticGraph) -> Result<Vec<u8>, std::io::Error> {
        let _ = graph;

        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Writer;

    use lxdb_core::{graph::SemanticGraph, storage::AdjacencyList};

    #[test]
    fn encodes_empty_graph() {
        let graph = SemanticGraph {
            tokens: Vec::new(),
            relations: Vec::new(),
            adjacency: AdjacencyList::default(),
        };

        let bytes = Writer::new().encode(&graph).expect("encoding should succeed");

        assert!(bytes.is_empty());
    }
}
