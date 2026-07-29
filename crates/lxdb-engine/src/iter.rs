use lxdb_format::{AdjacencyRecord, FormatError, RelationRecord, TokenRecord};

use lxdb_storage::BinaryDataset;

#[derive(Debug)]
pub struct TokenRecordIter<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> TokenRecordIter<'a> {
    pub fn new(dataset: &'a BinaryDataset) -> Self {
        Self { bytes: dataset.token_records(), cursor: 0 }
    }
}

impl Iterator for TokenRecordIter<'_> {
    type Item = Result<TokenRecord, FormatError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.bytes.len() {
            return None;
        }

        let end = self.cursor + TokenRecord::SIZE;

        let record = TokenRecord::decode(&self.bytes[self.cursor..end]);

        self.cursor = end;

        Some(record)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.bytes.len() - self.cursor) / TokenRecord::SIZE;

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for TokenRecordIter<'_> {}

#[derive(Debug)]
pub struct RelationRecordIter<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> RelationRecordIter<'a> {
    pub fn new(dataset: &'a BinaryDataset) -> Self {
        Self { bytes: dataset.relation_records(), cursor: 0 }
    }
}

impl Iterator for RelationRecordIter<'_> {
    type Item = Result<RelationRecord, FormatError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.bytes.len() {
            return None;
        }

        let end = self.cursor + RelationRecord::SIZE;

        let record = RelationRecord::decode(&self.bytes[self.cursor..end]);

        self.cursor = end;

        Some(record)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.bytes.len() - self.cursor) / RelationRecord::SIZE;

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RelationRecordIter<'_> {}

#[derive(Debug)]
pub struct AdjacencyRecordIter<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> AdjacencyRecordIter<'a> {
    pub fn new(dataset: &'a BinaryDataset) -> Self {
        Self { bytes: dataset.adjacency_records(), cursor: 0 }
    }
}

impl Iterator for AdjacencyRecordIter<'_> {
    type Item = Result<AdjacencyRecord, FormatError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.bytes.len() {
            return None;
        }

        let end = self.cursor + AdjacencyRecord::SIZE;

        let record = AdjacencyRecord::decode(&self.bytes[self.cursor..end]);

        self.cursor = end;

        Some(record)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.bytes.len() - self.cursor) / AdjacencyRecord::SIZE;

        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for AdjacencyRecordIter<'_> {}
