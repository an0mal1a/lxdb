use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourceKind {
    Kaikki,
    Hunspell,
    WordNet,
    Frequency,
    Embedding,
}
impl SourceKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Kaikki => "kaikki",
            Self::Hunspell => "hunspell",
            Self::WordNet => "wordnet",
            Self::Frequency => "frequency",
            Self::Embedding => "embedding",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LexicalRelationKind {
    Synonym,
    Antonym,
    Hypernym,
    Hyponym,
    Meronym,
    Holonym,
    DerivedFrom,
    InflectionOf,
    Related,
    Translation,
    EmbeddingNeighbor,
}
impl LexicalRelationKind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Synonym => "synonym",
            Self::Antonym => "antonym",
            Self::Hypernym => "hypernym",
            Self::Hyponym => "hyponym",
            Self::Meronym => "meronym",
            Self::Holonym => "holonym",
            Self::DerivedFrom => "derived_from",
            Self::InflectionOf => "inflection_of",
            Self::Related => "related",
            Self::Translation => "translation",
            Self::EmbeddingNeighbor => "embedding_neighbor",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexicalRelation {
    pub target: String,
    pub kind: LexicalRelationKind,
    pub source: SourceKind,
    pub confidence: f32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SourceReference {
    pub source: SourceKind,
    pub snapshot: String,
    pub source_id: Option<String>,
    pub confidence: f32,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SemanticQuality {
    pub accepted: bool,
    pub connected: bool,
    pub quality_score: f32,
    pub relation_count: u32,
    pub reachable_within_two: u32,
    pub frequency: Option<f32>,
}
#[derive(Debug, Clone)]
pub struct LexicalEntry {
    pub canonical: String,
    pub normalized_key: String,
    pub language: String,
    pub part_of_speech: BTreeSet<String>,
    pub forms: BTreeSet<String>,
    pub senses: Vec<String>,
    pub relations: Vec<LexicalRelation>,
    pub frequency: Option<f32>,
    pub provenance: Vec<SourceReference>,
    pub quality: SemanticQuality,
}
impl LexicalEntry {
    pub fn new(word: String, language: &str) -> Self {
        let normalized_key = word.to_lowercase();
        Self {
            canonical: word,
            normalized_key,
            language: language.to_owned(),
            part_of_speech: BTreeSet::new(),
            forms: BTreeSet::new(),
            senses: Vec::new(),
            relations: Vec::new(),
            frequency: None,
            provenance: Vec::new(),
            quality: SemanticQuality { accepted: true, ..SemanticQuality::default() },
        }
    }
}
