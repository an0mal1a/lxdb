use std::path::Path;

use lxdb_dictionary::{LANGUAGES, build, update};

use crate::error::CliError;

pub fn execute_dictionary_languages() -> Result<(), CliError> {
    for (code, name) in LANGUAGES {
        println!("{code}\t{name}");
    }
    Ok(())
}

pub fn execute_dictionary_build(
    language: &str,
    source: Option<&Path>,
    output: &Path,
    limit: Option<usize>,
) -> Result<(), CliError> {
    if let Some(limit) = limit {
        if limit == 0 {
            return Err(CliError::Message("--limit must be greater than zero".to_owned()));
        }
    }
    let report =
        build(language, source, output).map_err(|error| CliError::Message(error.to_string()))?;
    println!(
        "Built {language} dictionary: {} tokens, {} relations",
        report.tokens, report.relations
    );
    println!("Source: {}", report.source.display());
    println!("Output: {}", report.output.display());
    Ok(())
}

pub fn execute_dictionary_update(language: &str) -> Result<(), CliError> {
    let manifest = update(language).map_err(|error| CliError::Message(error.to_string()))?;
    println!("Updated local dictionary manifest: {}", manifest.display());
    Ok(())
}
