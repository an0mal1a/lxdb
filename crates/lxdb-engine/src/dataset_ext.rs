use lxdb_format::{AdjacencyRecord, RelationRecord, TokenRecord};

use lxdb_storage::BinaryDataset;

use crate::{AdjacencyRecordIter, RecordIter, RelationRecordIter, TokenRecordIter};

pub trait BinaryDatasetExt {
    fn tokens(&self) -> TokenRecordIter<'_>;

    fn relations(&self) -> RelationRecordIter<'_>;

    fn adjacency(&self) -> AdjacencyRecordIter<'_>;
}

impl BinaryDatasetExt for BinaryDataset {
    fn tokens(&self) -> TokenRecordIter<'_> {
        RecordIter::<TokenRecord>::new(self.token_records())
    }

    fn relations(&self) -> RelationRecordIter<'_> {
        RecordIter::<RelationRecord>::new(self.relation_records())
    }

    fn adjacency(&self) -> AdjacencyRecordIter<'_> {
        RecordIter::<AdjacencyRecord>::new(self.adjacency_records())
    }
}
