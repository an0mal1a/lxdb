use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "lxdb", version, about = "Compile and query LXDB semantic datasets")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Query the outgoing relations of a token.
    Query {
        /// Path to the compiled .lxdb dataset.
        dataset: PathBuf,

        /// Exact token text to query.
        token: String,
    },
}
