use std::str;

use lxdb_core::ids::TokenId;
use lxdb_format::TokenRecord;

use crate::{EngineError, TokenRecordIter};

/// A token resolved directly against the binary string table.
///
/// The text borrows from the underlying `BinaryDataset`, so no `String`
/// allocation is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BinaryToken<'a> {
    id: TokenId,
    text: &'a str,
}

impl<'a> BinaryToken<'a> {
    pub const fn new(id: TokenId, text: &'a str) -> Self {
        Self { id, text }
    }

    pub const fn id(&self) -> TokenId {
        self.id
    }

    pub const fn text(&self) -> &'a str {
        self.text
    }
}

/// Resolves token records lazily against the dataset string table.
#[derive(Debug, Clone)]
pub struct BinaryTokenIter<'a> {
    records: TokenRecordIter<'a>,
    string_table: &'a [u8],
}

impl<'a> BinaryTokenIter<'a> {
    pub(crate) const fn new(records: TokenRecordIter<'a>, string_table: &'a [u8]) -> Self {
        Self { records, string_table }
    }

    fn resolve(&self, record: TokenRecord) -> Result<BinaryToken<'a>, EngineError> {
        let offset =
            usize::try_from(record.offset()).map_err(|_| EngineError::TokenStringOutOfBounds {
                token_id: record.id(),
                offset: record.offset(),
                length: record.length(),
                table_length: self.string_table.len(),
            })?;

        let length = usize::try_from(record.length()).expect("u32 token length must fit in usize");

        let end = offset.checked_add(length).ok_or(EngineError::TokenStringOutOfBounds {
            token_id: record.id(),
            offset: record.offset(),
            length: record.length(),
            table_length: self.string_table.len(),
        })?;

        let bytes =
            self.string_table.get(offset..end).ok_or(EngineError::TokenStringOutOfBounds {
                token_id: record.id(),
                offset: record.offset(),
                length: record.length(),
                table_length: self.string_table.len(),
            })?;

        let text = str::from_utf8(bytes)
            .map_err(|source| EngineError::InvalidTokenUtf8 { token_id: record.id(), source })?;

        Ok(BinaryToken::new(TokenId::new(record.id()), text))
    }
}

impl<'a> Iterator for BinaryTokenIter<'a> {
    type Item = Result<BinaryToken<'a>, EngineError>;

    fn next(&mut self) -> Option<Self::Item> {
        let record = match self.records.next()? {
            Ok(record) => record,
            Err(error) => return Some(Err(error.into())),
        };

        Some(self.resolve(record))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.records.size_hint()
    }
}

impl ExactSizeIterator for BinaryTokenIter<'_> {
    fn len(&self) -> usize {
        self.records.len()
    }
}

impl std::iter::FusedIterator for BinaryTokenIter<'_> {}

#[cfg(test)]
mod tests {
    use lxdb_format::TokenRecord;

    use crate::{BinaryTokenIter, EngineError, RecordIter};

    #[test]
    fn resolves_tokens_without_allocating_strings() {
        let first = TokenRecord::new(0, 0, 4);

        let second = TokenRecord::new(1, 4, 8);

        let mut records = Vec::new();

        records.extend_from_slice(&first.encode());
        records.extend_from_slice(&second.encode());

        let string_table = b"rustlanguage";

        let record_iter = RecordIter::<TokenRecord>::new(&records);

        let mut tokens = BinaryTokenIter::new(record_iter, string_table);

        assert_eq!(tokens.len(), 2);

        let first =
            tokens.next().expect("first token should exist").expect("first token should resolve");

        assert_eq!(first.id().value(), 0);
        assert_eq!(first.text(), "rust");

        let second =
            tokens.next().expect("second token should exist").expect("second token should resolve");

        assert_eq!(second.id().value(), 1);
        assert_eq!(second.text(), "language");

        assert!(tokens.next().is_none());
    }

    #[test]
    fn rejects_out_of_bounds_string_ranges() {
        let record = TokenRecord::new(7, 10, 5);

        let encoded = record.encode();

        let record_iter = RecordIter::<TokenRecord>::new(&encoded);

        let mut tokens = BinaryTokenIter::new(record_iter, b"short");

        let error = tokens
            .next()
            .expect("token record should exist")
            .expect_err("invalid string range should fail");

        assert!(matches!(
            error,
            EngineError::TokenStringOutOfBounds {
                token_id: 7,
                offset: 10,
                length: 5,
                table_length: 5,
            }
        ));
    }

    #[test]
    fn rejects_invalid_utf8_token_strings() {
        let record = TokenRecord::new(3, 0, 2);

        let encoded = record.encode();

        let record_iter = RecordIter::<TokenRecord>::new(&encoded);

        let invalid_utf8 = [0xFF, 0xFF];

        let mut tokens = BinaryTokenIter::new(record_iter, &invalid_utf8);

        let error = tokens
            .next()
            .expect("token record should exist")
            .expect_err("invalid UTF-8 should fail");

        assert!(matches!(error, EngineError::InvalidTokenUtf8 { token_id: 3, .. }));
    }
}
