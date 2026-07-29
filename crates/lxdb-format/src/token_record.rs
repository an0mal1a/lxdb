use crate::{BinaryRecord, FormatError};
/// Number of bytes occupied by an encoded token record.
pub const TOKEN_RECORD_SIZE: usize = 24;

/// Binary representation of a token.
///
/// The text itself is stored separately inside the token string table.
/// `offset` is relative to the beginning of that section.
///
/// Binary layout:
///
/// - token id: 4 bytes
/// - reserved: 4 bytes
/// - string offset: 8 bytes
/// - string length: 4 bytes
/// - reserved: 4 bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenRecord {
    id: u32,
    offset: u64,
    length: u32,
}

impl TokenRecord {
    pub const SIZE: usize = TOKEN_RECORD_SIZE;

    pub const fn new(id: u32, offset: u64, length: u32) -> Self {
        Self { id, offset, length }
    }

    pub const fn id(self) -> u32 {
        self.id
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn length(self) -> u32 {
        self.length
    }

    pub const fn end(self) -> u64 {
        self.offset + self.length as u64
    }

    pub fn encode(self) -> [u8; TOKEN_RECORD_SIZE] {
        let mut bytes = [0_u8; TOKEN_RECORD_SIZE];

        bytes[0..4].copy_from_slice(&self.id.to_le_bytes());

        // bytes[4..8] are reserved and remain zero.

        bytes[8..16].copy_from_slice(&self.offset.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.length.to_le_bytes());

        // bytes[20..24] are reserved and remain zero.

        bytes
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, FormatError> {
        if bytes.len() != Self::SIZE {
            return Err(FormatError::UnexpectedRecordSize {
                record: "token record",
                expected: Self::SIZE,
                found: bytes.len(),
            });
        }

        let id =
            u32::from_le_bytes(bytes[0..4].try_into().expect("token id must occupy four bytes"));

        let offset = u64::from_le_bytes(
            bytes[8..16].try_into().expect("token string offset must occupy eight bytes"),
        );

        let length = u32::from_le_bytes(
            bytes[16..20].try_into().expect("token string length must occupy four bytes"),
        );

        Ok(Self::new(id, offset, length))
    }
}

impl BinaryRecord for TokenRecord {
    const SIZE: usize = TokenRecord::SIZE;

    fn decode(bytes: &[u8]) -> Result<Self, FormatError> {
        TokenRecord::decode(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::{TOKEN_RECORD_SIZE, TokenRecord};

    #[test]
    fn encodes_token_record() {
        let record = TokenRecord::new(42, 4_294_967_500, 128);

        let bytes = record.encode();

        assert_eq!(bytes.len(), TOKEN_RECORD_SIZE);

        assert_eq!(
            u32::from_le_bytes(bytes[0..4].try_into().expect("token id must occupy four bytes"),),
            42,
        );

        assert_eq!(&bytes[4..8], &[0; 4]);

        assert_eq!(
            u64::from_le_bytes(
                bytes[8..16].try_into().expect("string offset must occupy eight bytes"),
            ),
            4_294_967_500,
        );

        assert_eq!(
            u32::from_le_bytes(
                bytes[16..20].try_into().expect("string length must occupy four bytes"),
            ),
            128,
        );

        assert_eq!(&bytes[20..24], &[0; 4]);
    }

    #[test]
    fn calculates_string_range_end() {
        let record = TokenRecord::new(0, 500, 25);

        assert_eq!(record.end(), 525);
    }

    #[test]
    fn exposes_record_fields() {
        let record = TokenRecord::new(7, 100, 12);

        assert_eq!(record.id(), 7);
        assert_eq!(record.offset(), 100);
        assert_eq!(record.length(), 12);
    }

    #[test]
    fn decodes_token_record() {
        let original = TokenRecord::new(42, 8_589_934_592, 128);

        let encoded = original.encode();

        let decoded = TokenRecord::decode(&encoded).expect("encoded token record should decode");

        assert_eq!(decoded, original);
    }

    #[test]
    fn rejects_invalid_token_record_size() {
        let bytes = [0_u8; TokenRecord::SIZE - 1];

        let error = TokenRecord::decode(&bytes).expect_err("truncated token record should fail");

        assert!(matches!(
            error,
            crate::FormatError::UnexpectedRecordSize {
                record: "token record",
                expected: TokenRecord::SIZE,
                found,
            } if found == TokenRecord::SIZE - 1
        ));
    }
}
