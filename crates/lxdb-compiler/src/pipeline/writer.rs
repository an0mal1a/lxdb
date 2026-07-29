use std::io;

use lxdb_core::graph::SemanticGraph;
use lxdb_format::{
    AdjacencyRecord, Header, RelationRecord, Section, SectionHeader, TokenRecord, flags,
};

/// Encodes a semantic graph into the LXDB binary format.
///
/// The writer is storage-agnostic: it produces an in-memory byte buffer
/// and leaves persistence to the caller.
#[derive(Debug, Default)]
pub struct Writer;

impl Writer {
    pub const fn new() -> Self {
        Self
    }

    pub fn encode(&self, graph: &SemanticGraph) -> Result<Vec<u8>, io::Error> {
        let token_records = self.encode_token_records(graph)?;
        let string_table = self.encode_string_table(graph);
        let relation_records = self.encode_relation_records(graph)?;
        let adjacency_records = self.encode_adjacency_records(graph)?;

        let capacity = Header::SIZE
            + SectionHeader::SIZE * 4
            + token_records.len()
            + string_table.len()
            + relation_records.len()
            + adjacency_records.len();

        let mut bytes = Vec::with_capacity(capacity);

        bytes.extend_from_slice(&Header::current().encode());

        Self::append_section(&mut bytes, Section::Tokens, &token_records)?;
        Self::append_section(&mut bytes, Section::TokenStringTable, &string_table)?;
        Self::append_section(&mut bytes, Section::Relations, &relation_records)?;
        Self::append_section(&mut bytes, Section::Adjacency, &adjacency_records)?;

        Ok(bytes)
    }

    fn append_section(
        output: &mut Vec<u8>,
        section: Section,
        payload: &[u8],
    ) -> Result<(), io::Error> {
        let payload_length = u64::try_from(payload.len()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "section payload is too large to encode")
        })?;

        let header = SectionHeader::new(section, flags::NONE, payload_length);

        output.extend_from_slice(&header.encode());
        output.extend_from_slice(payload);

        Ok(())
    }

    fn encode_token_records(&self, graph: &SemanticGraph) -> Result<Vec<u8>, io::Error> {
        let mut records =
            Vec::with_capacity(graph.tokens().len().saturating_mul(TokenRecord::SIZE));

        let mut string_offset = 0_u64;

        for token in graph.tokens() {
            let text_length = u32::try_from(token.text().len()).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidData, "token text is too large to encode")
            })?;

            let record = TokenRecord::new(token.id().value(), string_offset, text_length);

            records.extend_from_slice(&record.encode());

            string_offset = string_offset.checked_add(u64::from(text_length)).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "token string table offset overflow")
            })?;
        }

        Ok(records)
    }

    fn encode_string_table(&self, graph: &SemanticGraph) -> Vec<u8> {
        let capacity = graph.tokens().iter().map(|token| token.text().len()).sum();

        let mut table = Vec::with_capacity(capacity);

        for token in graph.tokens() {
            table.extend_from_slice(token.text().as_bytes());
        }

        table
    }

    fn encode_relation_records(&self, graph: &SemanticGraph) -> Result<Vec<u8>, io::Error> {
        let mut records =
            Vec::with_capacity(graph.relations().len().saturating_mul(RelationRecord::SIZE));

        for relation in graph.relations() {
            let record = RelationRecord::new(
                relation.id().value(),
                relation.source().value(),
                relation.target().value(),
                relation.weight().value(),
            );

            records.extend_from_slice(&record.encode());
        }

        Ok(records)
    }

    fn encode_adjacency_records(&self, graph: &SemanticGraph) -> Result<Vec<u8>, io::Error> {
        let mut records =
            Vec::with_capacity(graph.tokens().len().saturating_mul(AdjacencyRecord::SIZE));

        for token in graph.tokens() {
            let Some(entry) = graph.adjacency().get(token.id()) else {
                records.extend_from_slice(&AdjacencyRecord::new(0, 0).encode());
                continue;
            };

            let offset = u64::try_from(entry.offset()).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "adjacency offset is too large to encode",
                )
            })?;

            let count = u32::try_from(entry.count()).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "adjacency relation count is too large to encode",
                )
            })?;

            records.extend_from_slice(&AdjacencyRecord::new(offset, count).encode());
        }

        Ok(records)
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
