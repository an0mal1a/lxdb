/// Number of bytes occupied by an encoded relation record.
pub const RELATION_RECORD_SIZE: usize = 16;

/// Binary representation of a semantic relation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelationRecord {
    id: u32,
    source: u32,
    target: u32,
    weight: f32,
}

impl RelationRecord {
    pub const fn new(id: u32, source: u32, target: u32, weight: f32) -> Self {
        Self { id, source, target, weight }
    }

    pub const fn id(self) -> u32 {
        self.id
    }

    pub const fn source(self) -> u32 {
        self.source
    }

    pub const fn target(self) -> u32 {
        self.target
    }

    pub const fn weight(self) -> f32 {
        self.weight
    }

    pub fn encode(self) -> [u8; RELATION_RECORD_SIZE] {
        let mut bytes = [0_u8; RELATION_RECORD_SIZE];

        bytes[0..4].copy_from_slice(&self.id.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.source.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.target.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.weight.to_le_bytes());

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{RELATION_RECORD_SIZE, RelationRecord};

    #[test]
    fn encodes_relation_record() {
        let record = RelationRecord::new(12, 3, 9, 0.85);

        let bytes = record.encode();

        assert_eq!(bytes.len(), RELATION_RECORD_SIZE);

        assert_eq!(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3],]), 12);

        assert_eq!(u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7],]), 3);

        assert_eq!(u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11],]), 9);

        assert_eq!(f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15],]), 0.85);
    }

    #[test]
    fn preserves_weight_bits() {
        let weight = 0.333_333_34_f32;
        let record = RelationRecord::new(0, 1, 2, weight);

        let bytes = record.encode();

        let decoded = f32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);

        assert_eq!(decoded.to_bits(), weight.to_bits());
    }

    #[test]
    fn exposes_relation_fields() {
        let record = RelationRecord::new(7, 11, 13, 1.0);

        assert_eq!(record.id(), 7);
        assert_eq!(record.source(), 11);
        assert_eq!(record.target(), 13);
        assert_eq!(record.weight(), 1.0);
    }
}
