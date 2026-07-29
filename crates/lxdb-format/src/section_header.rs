use crate::section::Section;

/// Number of bytes occupied by a section header.
pub const SECTION_HEADER_SIZE: usize = 12;

/// Header stored before every section payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionHeader {
    section: Section,
    flags: u8,
    length: u64,
}

impl SectionHeader {
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

        // bytes[2..4] remain reserved and must be zero.
        bytes[4..12].copy_from_slice(&self.length.to_le_bytes());

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::{SECTION_HEADER_SIZE, SectionHeader};
    use crate::{flags, section::Section};

    #[test]
    fn encodes_section_header() {
        let header =
            SectionHeader::new(Section::Relations, flags::COMPRESSED | flags::OPTIONAL, 4_096);

        let bytes = header.encode();

        assert_eq!(bytes.len(), SECTION_HEADER_SIZE);
        assert_eq!(bytes[0], Section::Relations.as_u8());
        assert_eq!(bytes[1], flags::COMPRESSED | flags::OPTIONAL);
        assert_eq!(&bytes[2..4], &[0, 0]);

        assert_eq!(
            u64::from_le_bytes([
                bytes[4], bytes[5], bytes[6], bytes[7], bytes[8], bytes[9], bytes[10], bytes[11],
            ]),
            4_096
        );
    }

    #[test]
    fn encodes_empty_section() {
        let header = SectionHeader::new(Section::Metadata, flags::NONE, 0);

        let bytes = header.encode();

        assert_eq!(bytes[0], Section::Metadata.as_u8());
        assert_eq!(bytes[1], flags::NONE);
        assert_eq!(&bytes[4..12], &[0; 8]);
    }
}
