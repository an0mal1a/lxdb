//! Deterministic local dictionary pipeline.
//!
//! Providers deliberately hand the compiler an `.lx` source: external sources
//! can be streamed and normalized into the same intermediate file without
//! coupling linguistic acquisition to the binary LXDB format.

use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use lxdb_compiler::{builder::Builder, compiler::Compiler};
use lxdb_storage::DatasetReader;

#[derive(Debug)]
pub enum DictionaryError {
    UnsupportedLanguage(String),
    Io(std::io::Error),
    Compile(lxdb_compiler::error::CompilerError),
    Validate(lxdb_storage::StorageError),
}

impl fmt::Display for DictionaryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLanguage(language) => {
                write!(formatter, "unsupported language: {language}")
            }
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::Compile(error) => write!(formatter, "LXDB compilation failed: {error}"),
            Self::Validate(error) => write!(formatter, "generated dataset is invalid: {error}"),
        }
    }
}

impl std::error::Error for DictionaryError {}

impl From<std::io::Error> for DictionaryError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub const LANGUAGES: &[(&str, &str)] = &[("es", "Español"), ("en", "English")];

pub struct BuildReport {
    pub source: PathBuf,
    pub output: PathBuf,
    pub tokens: usize,
    pub relations: usize,
}

pub fn build(
    language: &str,
    source: Option<&Path>,
    output: &Path,
) -> Result<BuildReport, DictionaryError> {
    validate_language(language)?;
    let source = source
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from(format!("datasets/fixtures/{language}-dev.lx")));
    let parent = output.parent().ok_or_else(|| {
        DictionaryError::Io(std::io::Error::other("output path has no parent directory"))
    })?;
    fs::create_dir_all(parent)?;
    let graph = Compiler::new()
        .compile(
            Builder::new()
                .input(source.to_string_lossy().into_owned())
                .output(output.to_string_lossy().into_owned())
                .build(),
        )
        .map_err(DictionaryError::Compile)?;
    DatasetReader::new().open(output).map_err(DictionaryError::Validate)?;
    write_manifest(language, &source)?;
    Ok(BuildReport {
        source,
        output: output.to_path_buf(),
        tokens: graph.tokens().len(),
        relations: graph.relations().len(),
    })
}

pub fn update(language: &str) -> Result<PathBuf, DictionaryError> {
    validate_language(language)?;
    let source = PathBuf::from(format!("datasets/fixtures/{language}-dev.lx"));
    write_manifest(language, &source)?;
    Ok(cache_directory(language).join("manifest.json"))
}

fn validate_language(language: &str) -> Result<(), DictionaryError> {
    if LANGUAGES.iter().any(|(code, _)| *code == language) {
        Ok(())
    } else {
        Err(DictionaryError::UnsupportedLanguage(language.to_owned()))
    }
}

fn cache_directory(language: &str) -> PathBuf {
    PathBuf::from(".lxdb/cache/dictionaries").join(language)
}

fn write_manifest(language: &str, source: &Path) -> Result<(), DictionaryError> {
    let directory = cache_directory(language);
    fs::create_dir_all(&directory)?;
    let bytes = fs::read(source)?;
    let hash = bytes.iter().fold(0xcbf29ce484222325_u64, |value, byte| {
        (value ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    });
    let temporary = directory.join("manifest.tmp");
    let manifest = format!(
        "{{\n  \"language\": \"{language}\",\n  \"source\": \"{}\",\n  \"source_kind\": \"versioned-fixture\",\n  \"hash_fnv1a64\": \"{hash:016x}\"\n}}\n",
        source.display()
    );
    fs::write(&temporary, manifest)?;
    let target = directory.join("manifest.json");
    if target.exists() {
        fs::remove_file(&target)?;
    }
    fs::rename(temporary, target)?;
    Ok(())
}
