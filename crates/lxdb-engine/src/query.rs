use lxdb_core::ids::TokenId;
use lxdb_storage::BinaryDataset;

use crate::{BinaryDatasetExt, BinaryRelationIter, BinaryToken, EngineError, RelationRecordIter};

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

    /// Resolves all outgoing relations for a token.
    pub fn resolved_outgoing(
        &self,
        token_id: TokenId,
    ) -> Result<BinaryRelationIter<'a>, EngineError> {
        let records = self.dataset.outgoing(token_id)?;

        Ok(BinaryRelationIter::new(self.dataset, records))
    }

    /// Finds a token by text and resolves its outgoing relations.
    ///
    /// `Ok(None)` means that no token exists with the provided text.
    pub fn related_to(&self, text: &str) -> Result<Option<BinaryRelationIter<'a>>, EngineError> {
        let Some(token) = self.token_by_text(text)? else {
            return Ok(None);
        };

        let relations = self.resolved_outgoing(token.id())?;

        Ok(Some(relations))
    }
}

#[cfg(test)]
mod tests {
    use lxdb_core::ids::TokenId;
    use lxdb_format::{AdjacencyRecord, RelationRecord};

    use crate::{BinaryDatasetExt, DatasetQuery, EngineError, test_support};

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

    #[test]
    fn rejects_relations_with_missing_target_tokens() {
        let relation = RelationRecord::new(7, 0, 99, 1.0);

        let dataset = test_support::dataset(&["rust"], &[relation], &[AdjacencyRecord::new(0, 1)]);

        let mut relations = dataset
            .query()
            .related_to("rust")
            .expect("related token query should initialize")
            .expect("source token should exist");

        let error = relations
            .next()
            .expect("relation should exist")
            .expect_err("missing target should fail");

        assert!(matches!(
            error,
            EngineError::MissingRelationTarget { relation_id: 7, token_id: 99 }
        ));
    }

    #[test]
    fn resolves_empty_relation_set() {
        let dataset = test_support::dataset(&["rust"], &[], &[AdjacencyRecord::new(0, 0)]);

        let relations = dataset
            .query()
            .related_to("rust")
            .expect("related token query should succeed")
            .expect("source token should exist");

        assert_eq!(relations.len(), 0);
    }

    #[test]
    fn returns_none_when_related_source_does_not_exist() {
        let dataset = test_support::dataset(&["rust"], &[], &[AdjacencyRecord::new(0, 0)]);

        let relations =
            dataset.query().related_to("python").expect("related token query should succeed");

        assert!(relations.is_none());
    }

    #[test]
    fn resolves_related_tokens_by_text() {
        let relation_to_language = RelationRecord::new(0, 0, 1, 0.9);

        let relation_to_compiler = RelationRecord::new(1, 0, 2, 0.7);

        let dataset = test_support::dataset(
            &["rust", "language", "compiler"],
            &[relation_to_language, relation_to_compiler],
            &[AdjacencyRecord::new(0, 2), AdjacencyRecord::new(2, 0), AdjacencyRecord::new(2, 0)],
        );

        let mut relations = dataset
            .query()
            .related_to("rust")
            .expect("related token query should succeed")
            .expect("source token should exist");

        assert_eq!(relations.len(), 2);

        let first = relations
            .next()
            .expect("first relation should exist")
            .expect("first relation should resolve");

        assert_eq!(first.id(), 0);
        assert_eq!(first.source().text(), "rust");
        assert_eq!(first.target().text(), "language");

        assert_eq!(first.weight().to_bits(), 0.9_f32.to_bits(),);

        let second = relations
            .next()
            .expect("second relation should exist")
            .expect("second relation should resolve");

        assert_eq!(second.id(), 1);
        assert_eq!(second.source().text(), "rust");
        assert_eq!(second.target().text(), "compiler");

        assert_eq!(second.weight().to_bits(), 0.7_f32.to_bits(),);

        assert!(relations.next().is_none());
    }
}
