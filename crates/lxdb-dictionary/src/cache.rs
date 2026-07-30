use crate::DictionaryError;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn cache_language_directory(root: &Path, language: &str) -> PathBuf {
    root.join(language)
}
pub fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), DictionaryError> {
    let parent = path.parent().ok_or_else(|| {
        DictionaryError::Io(std::io::Error::other("output path has no parent directory"))
    })?;
    fs::create_dir_all(parent)?;
    let temporary = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|x| x.to_str()).unwrap_or("tmp")
    ));
    fs::write(&temporary, bytes)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(temporary, path)?;
    Ok(())
}
