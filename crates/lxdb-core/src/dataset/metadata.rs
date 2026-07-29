use crate::ids::{DatasetId, LanguageId};

#[derive(Debug, Clone)]
pub struct Metadata {
    pub id: DatasetId,
    pub language: LanguageId,
    pub version: u16,
    pub name: Box<str>,
}