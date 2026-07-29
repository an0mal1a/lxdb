use std::{iter::FusedIterator, marker::PhantomData};

use lxdb_format::{AdjacencyRecord, BinaryRecord, FormatError, RelationRecord, TokenRecord};

/// Iterates over fixed-size binary records without allocating a collection.
#[derive(Debug, Clone)]
pub struct RecordIter<'a, T> {
    bytes: &'a [u8],
    cursor: usize,
    marker: PhantomData<T>,
}

impl<'a, T> RecordIter<'a, T>
where
    T: BinaryRecord,
{
    pub(crate) const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0, marker: PhantomData }
    }

    fn remaining(&self) -> usize {
        (self.bytes.len() - self.cursor) / T::SIZE
    }
}

impl<T> Iterator for RecordIter<'_, T>
where
    T: BinaryRecord,
{
    type Item = Result<T, FormatError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.bytes.len() {
            return None;
        }

        let end = self.cursor + T::SIZE;

        let record = T::decode(&self.bytes[self.cursor..end]);

        self.cursor = end;

        Some(record)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();

        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for RecordIter<'_, T>
where
    T: BinaryRecord,
{
    fn len(&self) -> usize {
        self.remaining()
    }
}

impl<T> FusedIterator for RecordIter<'_, T> where T: BinaryRecord {}

pub type TokenRecordIter<'a> = RecordIter<'a, TokenRecord>;

pub type RelationRecordIter<'a> = RecordIter<'a, RelationRecord>;

pub type AdjacencyRecordIter<'a> = RecordIter<'a, AdjacencyRecord>;

#[cfg(test)]
mod tests {
    use lxdb_format::{RelationRecord, TokenRecord};

    use super::RecordIter;

    #[test]
    fn iterates_over_token_records() {
        let first = TokenRecord::new(0, 0, 4);

        let second = TokenRecord::new(1, 4, 8);

        let mut bytes = Vec::new();

        bytes.extend_from_slice(&first.encode());
        bytes.extend_from_slice(&second.encode());

        let mut records = RecordIter::<TokenRecord>::new(&bytes);

        assert_eq!(records.len(), 2);

        let decoded_first =
            records.next().expect("first record should exist").expect("first record should decode");

        assert_eq!(decoded_first, first);
        assert_eq!(records.len(), 1);

        let decoded_second = records
            .next()
            .expect("second record should exist")
            .expect("second record should decode");

        assert_eq!(decoded_second, second);
        assert_eq!(records.len(), 0);
        assert!(records.next().is_none());
        assert!(records.next().is_none());
    }

    #[test]
    fn uses_the_same_iterator_for_relation_records() {
        let relation = RelationRecord::new(7, 2, 5, 0.75);

        let bytes = relation.encode();

        let mut records = RecordIter::<RelationRecord>::new(&bytes);

        let decoded = records
            .next()
            .expect("relation record should exist")
            .expect("relation record should decode");

        assert_eq!(decoded.id(), relation.id());
        assert_eq!(decoded.source(), relation.source());
        assert_eq!(decoded.target(), relation.target());

        assert_eq!(decoded.weight().to_bits(), relation.weight().to_bits(),);

        assert!(records.next().is_none());
    }

    #[test]
    fn empty_record_iterator_is_fused() {
        let mut records = RecordIter::<TokenRecord>::new(&[]);

        assert_eq!(records.len(), 0);
        assert!(records.next().is_none());
        assert!(records.next().is_none());
    }
}
