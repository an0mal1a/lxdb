use lxdb_core::ids::TokenId;
use lxdb_storage::BinaryDataset;

use crate::{BinaryDatasetExt, BinaryToken, EngineError, RelationRecordIter};

/// Read-only query interface over a binary LXDB dataset.
///
/// The query borrows the dataset and does not allocate an intermediate graph.
#[derive(Debug, Clone, Copy)]
pub struct DatasetQuery<'a> {
    dataset: &'a BinaryDataset,
}

impl<'a> DatasetQuery<'a> {
    pub const fn new(dataset: &'a BinaryDataset) -> Self {
        Self { dataset }
    }

    pub const fn dataset(&self) -> &'a BinaryDataset {
        self.dataset
    }

    /// Finds the first token whose text exactly matches `text`.
    ///
    /// This performs a linear scan over the token records.
    pub fn token_by_text(&self, text: &str) -> Result<Option<BinaryToken<'a>>, EngineError> {
        for token in self.dataset.resolved_tokens() {
            let token = token?;

            if token.text() == text {
                return Ok(Some(token));
            }
        }

        Ok(None)
    }

    pub fn token_by_id(&self, token_id: TokenId) -> Result<Option<BinaryToken<'a>>, EngineError> {
        for token in self.dataset.resolved_tokens() {
            let token = token?;

            if token.id() == token_id {
                return Ok(Some(token));
            }
        }

        Ok(None)
    }

    pub fn outgoing(&self, token_id: TokenId) -> Result<RelationRecordIter<'a>, EngineError> {
        self.dataset.outgoing(token_id)
    }
}

#[cfg(test)]
mod tests {
    use lxdb_core::ids::TokenId;
    use lxdb_format::{AdjacencyRecord, RelationRecord};

    use crate::{BinaryDatasetExt, DatasetQuery, test_support};

    #[test]
    fn finds_token_by_exact_text() {
        let dataset = test_support::dataset(
            &["rust", "language", "compiler"],
            &[],
            &[AdjacencyRecord::new(0, 0), AdjacencyRecord::new(0, 0), AdjacencyRecord::new(0, 0)],
        );

        let query = DatasetQuery::new(&dataset);

        let token = query
            .token_by_text("language")
            .expect("token lookup should succeed")
            .expect("token should exist");

        assert_eq!(token.id().value(), 1);
        assert_eq!(token.text(), "language");
    }

    #[test]
    fn returns_none_for_unknown_token_text() {
        let dataset = test_support::dataset(&["rust"], &[], &[AdjacencyRecord::new(0, 0)]);

        let token = dataset.query().token_by_text("python").expect("token lookup should succeed");

        assert!(token.is_none());
    }

    #[test]
    fn finds_token_by_id() {
        let dataset = test_support::dataset(
            &["rust", "memory"],
            &[],
            &[AdjacencyRecord::new(0, 0), AdjacencyRecord::new(0, 0)],
        );

        let token = dataset
            .query()
            .token_by_id(TokenId::new(1))
            .expect("token lookup should succeed")
            .expect("token should exist");

        assert_eq!(token.text(), "memory");
    }

    #[test]
    fn exposes_outgoing_relations() {
        let relation = RelationRecord::new(0, 0, 1, 0.9);

        let dataset = test_support::dataset(
            &["rust", "language"],
            &[relation],
            &[AdjacencyRecord::new(0, 1), AdjacencyRecord::new(1, 0)],
        );

        let mut outgoing =
            dataset.query().outgoing(TokenId::new(0)).expect("outgoing lookup should succeed");

        let relation =
            outgoing.next().expect("relation should exist").expect("relation should decode");

        assert_eq!(relation.source(), 0);
        assert_eq!(relation.target(), 1);
        assert_eq!(outgoing.len(), 0);
    }
}
