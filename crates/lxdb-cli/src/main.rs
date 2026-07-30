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
            DictionaryCommand::Build { language, output, source, limit } => {
                command::execute_dictionary_build(&language, source.as_deref(), &output, limit)
            }
            DictionaryCommand::Update { language } => command::execute_dictionary_update(&language),
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
