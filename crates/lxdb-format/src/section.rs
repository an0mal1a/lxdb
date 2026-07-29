/// Identifies a section inside an LXDB binary file.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Metadata = 1,
    Tokens = 2,
    TokenStringTable = 3,
    Relations = 4,
    Adjacency = 5,
}

impl Section {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Metadata),
            2 => Some(Self::Tokens),
            3 => Some(Self::TokenStringTable),
            4 => Some(Self::Relations),
            5 => Some(Self::Adjacency),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Section;

    #[test]
    fn converts_known_section_from_byte() {
        assert_eq!(Section::from_u8(Section::Tokens.as_u8()), Some(Section::Tokens),);

        assert_eq!(Section::from_u8(Section::Adjacency.as_u8()), Some(Section::Adjacency),);
    }

    #[test]
    fn rejects_unknown_section_byte() {
        assert_eq!(Section::from_u8(255), None);
    }
}
