use crate::section::Section;

/// Header preceding every binary section.
#[derive(Debug, Clone, Copy)]
pub struct SectionHeader {
    pub section: Section,
    pub length: u64,
}

impl SectionHeader {
    pub const fn new(section: Section, length: u64) -> Self {
        Self { section, length }
    }
}