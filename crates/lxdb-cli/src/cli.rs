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
    /// Compile a text source into an LXDB binary dataset.
    Compile {
        /// Path to the source dataset.
        source: PathBuf,

        /// Output path for the compiled .lxdb file.
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,
    },

    /// Query the outgoing relations of a token.
    Query {
        /// Path to the compiled .lxdb dataset.
        dataset: PathBuf,

        /// Exact token text to query.
        token: String,
    },

    /// Display structural information about a compiled .lxdb dataset.
    Inspect {
        /// Path to the compiled .lxdb dataset.
        dataset: PathBuf,
    },
}
