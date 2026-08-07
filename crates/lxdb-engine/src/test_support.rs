use lxdb_format::{
    AdjacencyRecord, Header, RelationRecord, Section, SectionHeader, TokenRecord, flags,
};

use lxdb_storage::{BinaryDataset, DatasetReader};

pub(crate) fn dataset(
    tokens: &[&str],
    relations: &[RelationRecord],
    adjacency: &[AdjacencyRecord],
) -> BinaryDataset {
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&Header::current().encode());

    let mut token_records = Vec::new();
    let mut string_table = Vec::new();

    for (index, text) in tokens.iter().enumerate() {
        let id = u32::try_from(index).expect("test token index should fit in u32");

        let offset =
            u64::try_from(string_table.len()).expect("test string offset should fit in u64");

        let length = u32::try_from(text.len()).expect("test token length should fit in u32");

        let record = TokenRecord::new(id, offset, length);

        token_records.extend_from_slice(&record.encode());

        string_table.extend_from_slice(text.as_bytes());
    }

    append_section(&mut bytes, Section::Tokens, &token_records);

    append_section(&mut bytes, Section::TokenStringTable, &string_table);

    let mut relation_bytes = Vec::new();

    for relation in relations {
        relation_bytes.extend_from_slice(&relation.encode());
    }

    append_section(&mut bytes, Section::Relations, &relation_bytes);

    let mut adjacency_bytes = Vec::new();

    for entry in adjacency {
        adjacency_bytes.extend_from_slice(&entry.encode());
    }

    append_section(&mut bytes, Section::Adjacency, &adjacency_bytes);

    DatasetReader::new().read(bytes).expect("test dataset should be valid")
}

fn append_section(output: &mut Vec<u8>, section: Section, payload: &[u8]) {
    let length = u64::try_from(payload.len()).expect("test payload length should fit in u64");

    let header = SectionHeader::new(section, flags::NONE, length);

    output.extend_from_slice(&header.encode());
    output.extend_from_slice(payload);
}
