use lxdb_format::{AdjacencyRecord, RelationRecord, TokenRecord};

use lxdb_storage::BinaryDataset;

use crate::{
    AdjacencyRecordIter, BinaryTokenIter, RecordIter, RelationRecordIter, TokenRecordIter,
};

pub trait BinaryDatasetExt {
    fn tokens(&self) -> TokenRecordIter<'_>;

    fn relations(&self) -> RelationRecordIter<'_>;

    fn adjacency(&self) -> AdjacencyRecordIter<'_>;

    fn resolved_tokens(&self) -> BinaryTokenIter<'_>;
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

    fn resolved_tokens(&self) -> BinaryTokenIter<'_> {
        BinaryTokenIter::new(self.tokens(), self.token_string_table())
    }
}
