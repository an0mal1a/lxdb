/// Number of bytes occupied by an encoded adjacency record.
pub const ADJACENCY_RECORD_SIZE: usize = 16;

/// Binary representation of a token's outgoing relation range.
///
/// The token identifier is implicit from this record's position
/// inside the adjacency section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdjacencyRecord {
    offset: u64,
    count: u32,
}

impl AdjacencyRecord {
    pub const fn new(offset: u64, count: u32) -> Self {
        Self { offset, count }
    }

    pub const fn offset(self) -> u64 {
        self.offset
    }

    pub const fn count(self) -> u32 {
        self.count
    }

    pub const fn end(self) -> u64 {
        self.offset + self.count as u64
    }

    pub fn encode(self) -> [u8; ADJACENCY_RECORD_SIZE] {
        let mut bytes = [0_u8; ADJACENCY_RECORD_SIZE];

        bytes[0..8].copy_from_slice(&self.offset.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.count.to_le_bytes());

        // bytes[12..16] remain reserved and must be zero.

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{ADJACENCY_RECORD_SIZE, AdjacencyRecord};

    #[test]
    fn encodes_adjacency_record() {
        let record = AdjacencyRecord::new(1_024, 17);

        let bytes = record.encode();

        assert_eq!(bytes.len(), ADJACENCY_RECORD_SIZE);

        assert_eq!(
            u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]),
            1_024
        );

        assert_eq!(u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11],]), 17);

        assert_eq!(&bytes[12..16], &[0; 4]);
    }

    #[test]
    fn calculates_relation_range_end() {
        let record = AdjacencyRecord::new(25, 12);

        assert_eq!(record.end(), 37);
    }

    #[test]
    fn supports_tokens_without_relations() {
        let record = AdjacencyRecord::new(50, 0);

        let bytes = record.encode();

        assert_eq!(record.offset(), 50);
        assert_eq!(record.count(), 0);
        assert_eq!(record.end(), 50);
        assert_eq!(&bytes[8..12], &[0; 4]);
    }
}
