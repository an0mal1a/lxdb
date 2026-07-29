use std::{fs, path::Path};

use lxdb_format::{Header, Section, SectionHeader, flags};

use crate::{BinaryDataset, SectionRange, StorageError};

/// Reads and validates sectioned LXDB binary datasets.
#[derive(Debug, Default)]
pub struct DatasetReader;

#[derive(Debug, Default)]
struct LocatedSections {
    metadata: Option<SectionRange>,
    tokens: Option<SectionRange>,
    token_string_table: Option<SectionRange>,
    relations: Option<SectionRange>,
    adjacency: Option<SectionRange>,
}

impl DatasetReader {
    pub const fn new() -> Self {
        Self
    }

    pub fn open(&self, path: impl AsRef<Path>) -> Result<BinaryDataset, StorageError> {
        let bytes = fs::read(path)?;

        self.read(bytes)
    }

    pub fn read(&self, bytes: Vec<u8>) -> Result<BinaryDataset, StorageError> {
        self.read_boxed(bytes.into_boxed_slice())
    }

    pub fn read_boxed(&self, bytes: Box<[u8]>) -> Result<BinaryDataset, StorageError> {
        Self::validate_header(&bytes)?;

        let sections = Self::locate_sections(&bytes)?;

        let token_records = Self::required_section(sections.tokens, Section::Tokens)?;

        let token_string_table =
            Self::required_section(sections.token_string_table, Section::TokenStringTable)?;

        let relation_records = Self::required_section(sections.relations, Section::Relations)?;

        let adjacency_records = Self::required_section(sections.adjacency, Section::Adjacency)?;

        BinaryDataset::new(
            bytes,
            token_records,
            token_string_table,
            relation_records,
            adjacency_records,
            sections.metadata,
        )
    }

    fn validate_header(bytes: &[u8]) -> Result<(), StorageError> {
        if bytes.len() < Header::SIZE {
            return Err(StorageError::InvalidHeader);
        }

        let expected = Header::current().encode();

        if bytes[..Header::SIZE] != expected {
            return Err(StorageError::InvalidHeader);
        }

        Ok(())
    }

    fn locate_sections(bytes: &[u8]) -> Result<LocatedSections, StorageError> {
        let mut sections = LocatedSections::default();
        let mut cursor = Header::SIZE;

        while cursor < bytes.len() {
            let remaining = bytes.len() - cursor;

            if remaining < SectionHeader::SIZE {
                return Err(StorageError::TruncatedSectionHeader { offset: cursor });
            }

            let header_start = cursor;
            let header_end = cursor + SectionHeader::SIZE;
            let header_bytes = &bytes[header_start..header_end];

            let section_type = header_bytes[0];
            let section_flags = header_bytes[1];

            let payload_length = u64::from_le_bytes(
                header_bytes[4..12].try_into().expect("section length must occupy eight bytes"),
            );

            let payload_start = header_end;

            let payload_length_usize =
                usize::try_from(payload_length).map_err(|_| StorageError::SectionOutOfBounds {
                    section_type,
                    offset: payload_start,
                    length: payload_length,
                })?;

            let payload_end = payload_start.checked_add(payload_length_usize).ok_or(
                StorageError::SectionOutOfBounds {
                    section_type,
                    offset: payload_start,
                    length: payload_length,
                },
            )?;

            if payload_end > bytes.len() {
                return Err(StorageError::SectionOutOfBounds {
                    section_type,
                    offset: payload_start,
                    length: payload_length,
                });
            }

            let range = SectionRange::new(payload_start, payload_end);

            match Section::from_u8(section_type) {
                Some(section) => {
                    if section_flags != flags::NONE {
                        return Err(StorageError::UnsupportedSectionFlags {
                            section_type,
                            flags: section_flags,
                        });
                    }

                    Self::insert_section(&mut sections, section, range)?;
                }

                None if section_flags & flags::OPTIONAL != 0 => {
                    // Unknown optional sections are skipped for
                    // forward compatibility.
                }

                None => {
                    return Err(StorageError::UnknownSection {
                        section_type,
                        offset: header_start,
                    });
                }
            }

            cursor = payload_end;
        }

        Ok(sections)
    }

    fn insert_section(
        sections: &mut LocatedSections,
        section: Section,
        range: SectionRange,
    ) -> Result<(), StorageError> {
        let slot = match section {
            Section::Metadata => &mut sections.metadata,
            Section::Tokens => &mut sections.tokens,
            Section::TokenStringTable => &mut sections.token_string_table,
            Section::Relations => &mut sections.relations,
            Section::Adjacency => &mut sections.adjacency,
        };

        if slot.is_some() {
            return Err(StorageError::DuplicateSection { section_type: section.as_u8() });
        }

        *slot = Some(range);

        Ok(())
    }

    fn required_section(
        section: Option<SectionRange>,
        section_type: Section,
    ) -> Result<SectionRange, StorageError> {
        section.ok_or(StorageError::MissingSection { section_type: section_type.as_u8() })
    }
}

#[cfg(test)]
mod tests {
    use super::DatasetReader;

    use lxdb_format::{Header, Section, SectionHeader, flags};

    use crate::StorageError;

    #[test]
    fn reads_empty_sectioned_dataset() {
        let bytes = empty_dataset_bytes();

        let dataset = DatasetReader::new().read(bytes).expect("dataset should be readable");

        assert_eq!(dataset.token_records(), &[]);
        assert_eq!(dataset.token_string_table(), &[]);
        assert_eq!(dataset.relation_records(), &[]);
        assert_eq!(dataset.adjacency_records(), &[]);

        assert_eq!(dataset.token_count(), 0);
        assert_eq!(dataset.relation_count(), 0);
        assert_eq!(dataset.adjacency_count(), 0);
    }

    #[test]
    fn rejects_invalid_header() {
        let bytes = vec![0_u8; Header::SIZE];

        let error =
            DatasetReader::new().read(bytes).expect_err("invalid header should be rejected");

        assert!(matches!(error, StorageError::InvalidHeader,));
    }

    #[test]
    fn rejects_missing_required_section() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&Header::current().encode());

        append_section(&mut bytes, Section::Tokens, &[]);

        let error =
            DatasetReader::new().read(bytes).expect_err("incomplete dataset should be rejected");

        assert!(matches!(error, StorageError::MissingSection { .. },));
    }

    #[test]
    fn rejects_duplicate_section() {
        let mut bytes = empty_dataset_bytes();

        append_section(&mut bytes, Section::Tokens, &[]);

        let error =
            DatasetReader::new().read(bytes).expect_err("duplicate section should be rejected");

        assert!(matches!(
            error,
            StorageError::DuplicateSection {
                section_type,
            } if section_type == Section::Tokens.as_u8()
        ));
    }

    #[test]
    fn skips_unknown_optional_section() {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&Header::current().encode());

        append_unknown_optional_section(&mut bytes, 200, b"future section");

        append_section(&mut bytes, Section::Tokens, &[]);

        append_section(&mut bytes, Section::TokenStringTable, &[]);

        append_section(&mut bytes, Section::Relations, &[]);

        append_section(&mut bytes, Section::Adjacency, &[]);

        let dataset =
            DatasetReader::new().read(bytes).expect("unknown optional section should be skipped");

        assert_eq!(dataset.token_count(), 0);
    }

    fn empty_dataset_bytes() -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&Header::current().encode());

        append_section(&mut bytes, Section::Tokens, &[]);

        append_section(&mut bytes, Section::TokenStringTable, &[]);

        append_section(&mut bytes, Section::Relations, &[]);

        append_section(&mut bytes, Section::Adjacency, &[]);

        bytes
    }

    fn append_section(output: &mut Vec<u8>, section: Section, payload: &[u8]) {
        let payload_length =
            u64::try_from(payload.len()).expect("test payload length should fit in u64");

        let header = SectionHeader::new(section, flags::NONE, payload_length);

        output.extend_from_slice(&header.encode());
        output.extend_from_slice(payload);
    }

    fn append_unknown_optional_section(output: &mut Vec<u8>, section_type: u8, payload: &[u8]) {
        let payload_length =
            u64::try_from(payload.len()).expect("test payload length should fit in u64");

        let mut header = [0_u8; SectionHeader::SIZE];

        header[0] = section_type;
        header[1] = flags::OPTIONAL;

        header[4..12].copy_from_slice(&payload_length.to_le_bytes());

        output.extend_from_slice(&header);
        output.extend_from_slice(payload);
    }
}
