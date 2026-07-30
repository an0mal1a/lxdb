use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DictionaryProfile {
    Development,
    Game,
    Full,
}

impl DictionaryProfile {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "development" => Some(Self::Development),
            "game" => Some(Self::Game),
            "full" => Some(Self::Full),
            _ => None,
        }
    }
    pub const fn name(self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Game => "game",
            Self::Full => "full",
        }
    }
    pub const fn max_entries(self) -> Option<usize> {
        match self {
            Self::Development => Some(25_000),
            // This is intentionally large enough for a real game vocabulary,
            // while bounding the current in-memory compiler implementation.
            Self::Game => Some(200_000),
            Self::Full => None,
        }
    }
    pub const fn max_relations_per_token(self) -> usize {
        match self {
            Self::Development => 16,
            Self::Game => 48,
            Self::Full => 96,
        }
    }
    pub const fn include_multiword_terms(self) -> bool {
        matches!(self, Self::Full)
    }
    pub const fn include_proper_nouns(self) -> bool {
        matches!(self, Self::Full)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SourceSelection {
    pub kaikki: bool,
    pub hunspell: bool,
    pub wordnet: bool,
    pub frequency: bool,
    pub embeddings: bool,
}
impl Default for SourceSelection {
    fn default() -> Self {
        Self { kaikki: true, hunspell: true, wordnet: true, frequency: true, embeddings: false }
    }
}

#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub language: String,
    pub profile: DictionaryProfile,
    pub output_dir: PathBuf,
    pub fixture_dir: Option<PathBuf>,
    pub cache_dir: PathBuf,
    pub config_path: Option<PathBuf>,
    pub limit: Option<usize>,
    pub refresh: bool,
    pub offline: bool,
    pub sources: SourceSelection,
    pub emit_source: Option<PathBuf>,
    pub keep_intermediate: bool,
    pub emit_rejected: bool,
}
impl BuildOptions {
    pub fn new(language: impl Into<String>, output_dir: impl Into<PathBuf>) -> Self {
        Self {
            language: language.into(),
            profile: DictionaryProfile::Development,
            output_dir: output_dir.into(),
            fixture_dir: None,
            cache_dir: PathBuf::from(".lxdb/cache/dictionaries"),
            config_path: None,
            limit: None,
            refresh: false,
            offline: false,
            sources: SourceSelection::default(),
            emit_source: None,
            keep_intermediate: false,
            emit_rejected: true,
        }
    }
    pub fn effective_limit(&self) -> Option<usize> {
        self.limit.or(self.profile.max_entries())
    }
}
