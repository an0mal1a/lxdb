mod cli;
mod command;
mod error;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Command, DictionaryCommand};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Compile { source, output } => command::execute_compile(&source, &output),

        Command::Query { dataset, token } => command::execute_query(&dataset, &token),

        Command::Inspect { dataset } => command::execute_inspect(&dataset),

        Command::Dictionary { command } => match command {
            DictionaryCommand::Languages => command::execute_dictionary_languages(),
            DictionaryCommand::Build {
                language,
                output,
                source_fixture,
                profile,
                config,
                limit,
                refresh,
                offline,
                cache_dir,
                without_kaikki,
                without_hunspell,
                without_wordnet,
                without_frequency,
                without_embeddings,
                emit_source,
                keep_intermediate,
                no_rejected_entries,
            } => {
                let Some(profile) = lxdb_dictionary::DictionaryProfile::parse(&profile) else {
                    return ExitCode::FAILURE;
                };
                let mut options = lxdb_dictionary::BuildOptions::new(language, output);
                options.profile = profile;
                options.fixture_dir = source_fixture;
                options.config_path = config;
                options.limit = limit;
                options.refresh = refresh;
                options.offline = offline;
                if let Some(cache_dir) = cache_dir {
                    options.cache_dir = cache_dir;
                }
                options.sources.kaikki = !without_kaikki;
                options.sources.hunspell = !without_hunspell;
                options.sources.wordnet = !without_wordnet;
                options.sources.frequency = !without_frequency;
                options.sources.embeddings = !without_embeddings
                    && matches!(
                        profile,
                        lxdb_dictionary::DictionaryProfile::Game
                            | lxdb_dictionary::DictionaryProfile::Full
                    );
                options.emit_source = emit_source;
                options.keep_intermediate = keep_intermediate;
                options.emit_rejected = !no_rejected_entries;
                command::execute_dictionary_build(options)
            }
            DictionaryCommand::Update { language, cache_dir } => {
                command::execute_dictionary_update(
                    &language,
                    cache_dir
                        .as_deref()
                        .unwrap_or_else(|| std::path::Path::new(".lxdb/cache/dictionaries")),
                )
            }
            DictionaryCommand::Inspect { manifest } => {
                command::execute_dictionary_inspect(&manifest)
            }
        },
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,

        Err(error) => {
            eprintln!("error: {error}");

            ExitCode::FAILURE
        }
    }
}
