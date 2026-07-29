use lxdb_core::{graph::SemanticGraph, traversal::GraphTraversal};

use lxdb_format::{AdjacencyRecord, Header, RelationRecord, TokenRecord};

/// Encodes a semantic graph into the LXDB binary format.
#[derive(Debug, Default)]
pub struct Writer;

impl Writer {
    pub const fn new() -> Self {
        Self
    }

    pub fn encode(&self, graph: &SemanticGraph) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();

        //----------------------------------
        // Header
        //----------------------------------

        bytes.extend_from_slice(&Header::current().encode());

        //----------------------------------
        // Token Records + String Table
        //----------------------------------

        let mut string_table = Vec::<u8>::new();

        for token in graph.tokens() {
            let offset = string_table.len() as u32;

            string_table.extend_from_slice(token.text().as_bytes());

            let record = TokenRecord::new(token.id().value(), offset, token.text().len() as u32);

            bytes.extend_from_slice(&record.encode());
        }

        bytes.extend_from_slice(&string_table);

        //----------------------------------
        // Relations
        //----------------------------------

        for relation in graph.relations() {
            let record = RelationRecord::new(
                relation.id().value(),
                relation.source().value(),
                relation.target().value(),
                relation.weight().value(),
            );

            bytes.extend_from_slice(&record.encode());
        }

        //----------------------------------
        // Adjacency
        //----------------------------------

        for token in graph.tokens() {
            let outgoing = graph.outgoing(token.id());

            let offset = outgoing.first().map(|r| r.id().value() as u64).unwrap_or(0);

            let record = AdjacencyRecord::new(offset, outgoing.len() as u32);

            bytes.extend_from_slice(&record.encode());
        }

        Ok(bytes)
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

        assert!(!bytes.is_empty());
    }
}
