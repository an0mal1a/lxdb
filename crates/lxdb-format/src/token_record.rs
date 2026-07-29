/// Number of bytes occupied by an encoded token record.
pub const TOKEN_RECORD_SIZE: usize = 16;

/// Binary representation of a token.
///
/// The token text itself is stored separately in the token string table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenRecord {
    id: u32,
    offset: u32,
    length: u32,
}

impl TokenRecord {
    pub const fn new(id: u32, offset: u32, length: u32) -> Self {
        Self { id, offset, length }
    }

    pub const fn id(self) -> u32 {
        self.id
    }

    pub const fn offset(self) -> u32 {
        self.offset
    }

    pub const fn length(self) -> u32 {
        self.length
    }

    pub const fn end(self) -> u32 {
        self.offset + self.length
    }

    pub fn encode(self) -> [u8; TOKEN_RECORD_SIZE] {
        let mut bytes = [0_u8; TOKEN_RECORD_SIZE];

        bytes[0..4].copy_from_slice(&self.id.to_le_bytes());

        // bytes[4..8] remain reserved and must be zero.
        bytes[8..12].copy_from_slice(&self.offset.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.length.to_le_bytes());

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{TOKEN_RECORD_SIZE, TokenRecord};

    #[test]
    fn encodes_token_record() {
        let record = TokenRecord::new(42, 1_024, 7);

        let bytes = record.encode();

        assert_eq!(bytes.len(), TOKEN_RECORD_SIZE);

        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3],]), 42);

        assert_eq!(&bytes[4..8], &[0; 4]);

        assert_eq!(u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11],]), 1_024);

        assert_eq!(u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15],]), 7);
    }

    #[test]
    fn calculates_string_range_end() {
        let record = TokenRecord::new(0, 25, 12);

        assert_eq!(record.end(), 37);
    }

    #[test]
    fn encodes_empty_token_text() {
        let record = TokenRecord::new(3, 50, 0);

        let bytes = record.encode();

        assert_eq!(&bytes[12..16], &[0; 4]);
        assert_eq!(record.end(), 50);
    }
}
