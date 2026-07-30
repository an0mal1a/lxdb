use crate::StorageError;
use lxdb_format::{ADJACENCY_RECORD_SIZE, RELATION_RECORD_SIZE, TOKEN_RECORD_SIZE, Version};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SectionRange {
    start: usize,
    end: usize,
}

impl SectionRange {
    pub(crate) const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub(crate) const fn len(&self) -> usize {
        self.end - self.start
    }

    pub(crate) fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }
}

/// An opened LXDB binary dataset.
///
/// The dataset owns the underlying bytes while exposing borrowed views
/// over each section. It does not decode tokens or relations.
#[derive(Debug)]
pub struct BinaryDataset {
    bytes: Box<[u8]>,
    version: Version,

    token_records: SectionRange,
    token_string_table: SectionRange,
    relation_records: SectionRange,
    adjacency_records: SectionRange,

    metadata: Option<SectionRange>,
}

impl BinaryDataset {
    pub(crate) fn new(
        bytes: Box<[u8]>,
        version: Version,
        token_records: SectionRange,
        token_string_table: SectionRange,
        relation_records: SectionRange,
        adjacency_records: SectionRange,
        metadata: Option<SectionRange>,
    ) -> Result<Self, StorageError> {
        Self::validate_record_section(
            lxdb_format::Section::Tokens,
            token_records.len(),
            TOKEN_RECORD_SIZE,
        )?;

        Self::validate_record_section(
            lxdb_format::Section::Relations,
            relation_records.len(),
            RELATION_RECORD_SIZE,
        )?;

        Self::validate_record_section(
            lxdb_format::Section::Adjacency,
            adjacency_records.len(),
            ADJACENCY_RECORD_SIZE,
        )?;

        Ok(Self {
            bytes,
            version,
            token_records,
            token_string_table,
            relation_records,
            adjacency_records,
            metadata,
        })
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Version of the binary format used by this dataset.
    pub const fn version(&self) -> Version {
        self.version
    }

    /// Total number of bytes in the encoded dataset.
    pub fn file_size(&self) -> usize {
        self.bytes.len()
    }

    pub fn token_records(&self) -> &[u8] {
        &self.bytes[self.token_records.as_range()]
    }

    pub fn token_string_table(&self) -> &[u8] {
        &self.bytes[self.token_string_table.as_range()]
    }

    pub fn relation_records(&self) -> &[u8] {
        &self.bytes[self.relation_records.as_range()]
    }

    pub fn adjacency_records(&self) -> &[u8] {
        &self.bytes[self.adjacency_records.as_range()]
    }

    pub fn metadata(&self) -> Option<&[u8]> {
        self.metadata.as_ref().map(|range| &self.bytes[range.as_range()])
    }

    pub fn token_count(&self) -> usize {
        self.token_records.len() / TOKEN_RECORD_SIZE
    }

    pub fn relation_count(&self) -> usize {
        self.relation_records.len() / RELATION_RECORD_SIZE
    }

    pub fn adjacency_count(&self) -> usize {
        self.adjacency_records.len() / ADJACENCY_RECORD_SIZE
    }

    fn validate_record_section(
        section: lxdb_format::Section,
        length: usize,
        record_size: usize,
    ) -> Result<(), StorageError> {
        if length % record_size != 0 {
            return Err(StorageError::InvalidSectionLength {
                section_type: section.as_u8(),
                length,
                record_size,
            });
        }

        Ok(())
    }
}
