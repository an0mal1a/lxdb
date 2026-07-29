use crate::Section;

/// Number of bytes occupied by an encoded section header.
pub const SECTION_HEADER_SIZE: usize = 12;

/// Header preceding every binary LXDB section.
///
/// Binary layout:
///
/// - section type: 1 byte
/// - flags: 1 byte
/// - reserved: 2 bytes
/// - payload length: 8 bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionHeader {
    section: Section,
    flags: u8,
    length: u64,
}

impl SectionHeader {
    pub const SIZE: usize = SECTION_HEADER_SIZE;

    pub const fn new(section: Section, flags: u8, length: u64) -> Self {
        Self { section, flags, length }
    }

    pub const fn section(self) -> Section {
        self.section
    }

    pub const fn flags(self) -> u8 {
        self.flags
    }

    pub const fn length(self) -> u64 {
        self.length
    }

    pub fn encode(self) -> [u8; SECTION_HEADER_SIZE] {
        let mut bytes = [0_u8; SECTION_HEADER_SIZE];

        bytes[0] = self.section.as_u8();
        bytes[1] = self.flags;

        // bytes[2..4] are reserved and remain zero.

        bytes[4..12].copy_from_slice(&self.length.to_le_bytes());

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{SECTION_HEADER_SIZE, SectionHeader};
    use crate::{Section, flags};

    #[test]
    fn encodes_section_header() {
        let header =
            SectionHeader::new(Section::Tokens, flags::COMPRESSED | flags::OPTIONAL, 1_024);

        let bytes = header.encode();

        assert_eq!(bytes.len(), SECTION_HEADER_SIZE);
        assert_eq!(bytes[0], Section::Tokens.as_u8());
        assert_eq!(bytes[1], flags::COMPRESSED | flags::OPTIONAL);
        assert_eq!(&bytes[2..4], &[0, 0]);

        assert_eq!(
            u64::from_le_bytes([
                bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
            ]),
            1_024,
        );
    }

    #[test]
    fn encodes_section_without_flags() {
        let header = SectionHeader::new(Section::Relations, flags::NONE, 500);

        let bytes = header.encode();

        assert_eq!(bytes[1], 0);
    }
}
