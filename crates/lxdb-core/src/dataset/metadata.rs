use crate::ids::{DatasetId, LanguageId};

#[derive(Debug, Clone)]
pub struct Metadata {
    id: DatasetId,
    language: LanguageId,
    version: u16,
    name: Box<str>,
}

impl Metadata {
    pub fn new(
        id: DatasetId,
        language: LanguageId,
        version: u16,
        name: Box<str>,
    ) -> Self {
        Self {
            id,
            language,
            version,
            name,
        }
    }

    pub const fn id(&self) -> DatasetId {
        self.id
    }

    pub const fn language(&self) -> LanguageId {
        self.language
    }

    pub const fn version(&self) -> u16 {
        self.version
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}