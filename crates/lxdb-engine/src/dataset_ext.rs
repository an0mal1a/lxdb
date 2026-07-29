use lxdb_storage::BinaryDataset;

use crate::{AdjacencyRecordIter, RelationRecordIter, TokenRecordIter};

pub trait BinaryDatasetExt {
    fn tokens(&self) -> TokenRecordIter<'_>;

    fn relations(&self) -> RelationRecordIter<'_>;

    fn adjacency(&self) -> AdjacencyRecordIter<'_>;
}

impl BinaryDatasetExt for BinaryDataset {
    fn tokens(&self) -> TokenRecordIter<'_> {
        TokenRecordIter::new(self)
    }

    fn relations(&self) -> RelationRecordIter<'_> {
        RelationRecordIter::new(self)
    }

    fn adjacency(&self) -> AdjacencyRecordIter<'_> {
        AdjacencyRecordIter::new(self)
    }
}
