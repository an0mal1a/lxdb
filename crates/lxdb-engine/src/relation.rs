use crate::{EngineError, RecordIter, RelationRecordIter};
use lxdb_core::ids::TokenId;
use lxdb_format::{AdjacencyRecord, RelationRecord};
use lxdb_storage::BinaryDataset;

pub(crate) fn outgoing_relations(
    dataset: &BinaryDataset,
    token_id: TokenId,
) -> Result<RelationRecordIter<'_>, EngineError> {
    let token_index = usize::try_from(token_id.value()).expect("u32 token id must fit in usize");

    let adjacency_bytes = dataset.adjacency_records();

    let adjacency_start =
        token_index.checked_mul(AdjacencyRecord::SIZE).ok_or(EngineError::MissingAdjacency {
            token_id: token_id.value(),
            adjacency_count: dataset.adjacency_count(),
        })?;

    let adjacency_end = adjacency_start.checked_add(AdjacencyRecord::SIZE).ok_or(
        EngineError::MissingAdjacency {
            token_id: token_id.value(),
            adjacency_count: dataset.adjacency_count(),
        },
    )?;

    let adjacency_slice = adjacency_bytes.get(adjacency_start..adjacency_end).ok_or(
        EngineError::MissingAdjacency {
            token_id: token_id.value(),
            adjacency_count: dataset.adjacency_count(),
        },
    )?;

    let adjacency = AdjacencyRecord::decode(adjacency_slice)?;

    relation_range(dataset, token_id, adjacency)
}

fn relation_range(
    dataset: &BinaryDataset,
    token_id: TokenId,
    adjacency: AdjacencyRecord,
) -> Result<RelationRecordIter<'_>, EngineError> {
    let relation_offset = usize::try_from(adjacency.offset())
        .map_err(|_| relation_range_error(dataset, token_id, adjacency))?;

    let relation_count =
        usize::try_from(adjacency.count()).expect("u32 relation count must fit in usize");

    let relation_end = relation_offset
        .checked_add(relation_count)
        .ok_or_else(|| relation_range_error(dataset, token_id, adjacency))?;

    if relation_end > dataset.relation_count() {
        return Err(relation_range_error(dataset, token_id, adjacency));
    }

    let byte_start = relation_offset
        .checked_mul(RelationRecord::SIZE)
        .ok_or_else(|| relation_range_error(dataset, token_id, adjacency))?;

    let byte_end = relation_end
        .checked_mul(RelationRecord::SIZE)
        .ok_or_else(|| relation_range_error(dataset, token_id, adjacency))?;

    let bytes = dataset
        .relation_records()
        .get(byte_start..byte_end)
        .ok_or_else(|| relation_range_error(dataset, token_id, adjacency))?;

    Ok(RecordIter::<RelationRecord>::new(bytes))
}

fn relation_range_error(
    dataset: &BinaryDataset,
    token_id: TokenId,
    adjacency: AdjacencyRecord,
) -> EngineError {
    EngineError::RelationRangeOutOfBounds {
        token_id: token_id.value(),
        offset: adjacency.offset(),
        count: adjacency.count(),
        relation_count: dataset.relation_count(),
    }
}

#[cfg(test)]
mod tests {
    use lxdb_core::ids::TokenId;

    use lxdb_format::{AdjacencyRecord, Header, RelationRecord, Section, SectionHeader, flags};

    use lxdb_storage::DatasetReader;

    use crate::{BinaryDatasetExt, EngineError};

    #[test]
    fn resolves_only_outgoing_relations_for_token() {
        let first_relation = RelationRecord::new(0, 0, 1, 0.75);

        let second_relation = RelationRecord::new(1, 0, 2, 0.50);

        let dataset = dataset_with_relations(
            &[first_relation, second_relation],
            &[AdjacencyRecord::new(0, 2), AdjacencyRecord::new(2, 0), AdjacencyRecord::new(2, 0)],
        );

        let mut outgoing =
            dataset.outgoing(TokenId::new(0)).expect("token adjacency should resolve");

        assert_eq!(outgoing.len(), 2);

        let first = outgoing
            .next()
            .expect("first relation should exist")
            .expect("first relation should decode");

        assert_eq!(first.id(), 0);
        assert_eq!(first.source(), 0);
        assert_eq!(first.target(), 1);

        let second = outgoing
            .next()
            .expect("second relation should exist")
            .expect("second relation should decode");

        assert_eq!(second.id(), 1);
        assert_eq!(second.source(), 0);
        assert_eq!(second.target(), 2);

        assert!(outgoing.next().is_none());
    }

    #[test]
    fn supports_tokens_without_outgoing_relations() {
        let relation = RelationRecord::new(0, 0, 1, 1.0);

        let dataset = dataset_with_relations(
            &[relation],
            &[AdjacencyRecord::new(0, 1), AdjacencyRecord::new(1, 0)],
        );

        let outgoing = dataset.outgoing(TokenId::new(1)).expect("empty adjacency should be valid");

        assert_eq!(outgoing.len(), 0);
    }

    #[test]
    fn rejects_tokens_without_adjacency_records() {
        let dataset = dataset_with_relations(&[], &[AdjacencyRecord::new(0, 0)]);

        let error =
            dataset.outgoing(TokenId::new(5)).expect_err("missing adjacency should be rejected");

        assert!(matches!(error, EngineError::MissingAdjacency { token_id: 5, adjacency_count: 1 }));
    }

    #[test]
    fn rejects_relation_ranges_outside_table() {
        let dataset = dataset_with_relations(&[], &[AdjacencyRecord::new(4, 2)]);

        let error =
            dataset.outgoing(TokenId::new(0)).expect_err("invalid relation range should fail");

        assert!(matches!(
            error,
            EngineError::RelationRangeOutOfBounds {
                token_id: 0,
                offset: 4,
                count: 2,
                relation_count: 0,
            }
        ));
    }

    fn dataset_with_relations(
        relations: &[RelationRecord],
        adjacency: &[AdjacencyRecord],
    ) -> lxdb_storage::BinaryDataset {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&Header::current().encode());

        append_section(&mut bytes, Section::Tokens, &[]);

        append_section(&mut bytes, Section::TokenStringTable, &[]);

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
}
