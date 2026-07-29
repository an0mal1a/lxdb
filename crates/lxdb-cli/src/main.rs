mod cli;
mod command;
mod error;

use clap::Parser;
use cli::{Cli, Command};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
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
