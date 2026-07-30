use std::path::Path;

use lxdb_dictionary::{BuildOptions, LANGUAGES, build, inspect_manifest, update};

use crate::error::CliError;

pub fn execute_dictionary_languages() -> Result<(), CliError> {
    for language in LANGUAGES {
        println!("{}\t{}\t{}", language.iso_639_1, language.display_name, language.iso_639_3);
    }
    Ok(())
}

pub fn execute_dictionary_build(options: BuildOptions) -> Result<(), CliError> {
    let language = options.language.clone();
    let output = options.output_dir.clone();
    let profile = options.profile.name();
    let report = build(&options).map_err(|error| CliError::Message(error.to_string()))?;
    println!(
        "LXDB dictionary build\nLanguage: {}\nProfile: {profile}\nEntries read: {}\nEntries accepted: {}\nUnique lemmas: {}\nSurface forms: {}\nRelations: {}\nRejected: {}\nOutput: {}",
        language,
        report.entries_read,
        report.entries_accepted,
        report.unique_lemmas,
        report.surface_forms,
        report.relations,
        report.entries_rejected,
        output.join("dictionary.lxdb").display(),
    );
    Ok(())
}

pub fn execute_dictionary_update(language: &str, cache_dir: &Path) -> Result<(), CliError> {
    let manifest =
        update(language, cache_dir).map_err(|error| CliError::Message(error.to_string()))?;
    println!("Updated local dictionary manifest: {}", manifest.display());
    Ok(())
}

pub fn execute_dictionary_inspect(manifest: &Path) -> Result<(), CliError> {
    print!("{}", inspect_manifest(manifest).map_err(|error| CliError::Message(error.to_string()))?);
    Ok(())
}
