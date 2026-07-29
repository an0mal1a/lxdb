mod cli;
mod command;
mod error;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Compile { source, output } => command::execute_compile(&source, &output),

        Command::Query { dataset, token } => command::execute_query(&dataset, &token),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,

        Err(error) => {
            eprintln!("error: {error}");

            ExitCode::FAILURE
        }
    }
}
