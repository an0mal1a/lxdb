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

    /// Build and manage language datasets through the dictionary pipeline.
    Dictionary {
        #[command(subcommand)]
        command: DictionaryCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum DictionaryCommand {
    /// List the built-in language pipelines.
    Languages,
    /// Compile a versioned linguistic source into a validated LXDB dataset.
    Build {
        /// BCP-47 base language code, currently es or en.
        language: String,
        /// Output LXDB file.
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,
        /// Optional normalized .lx input. Defaults to the versioned development fixture.
        #[arg(long, value_name = "SOURCE")]
        source: Option<PathBuf>,
        /// Reserved capacity limit for a future streaming provider. The local source remains deterministic.
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Refresh the local source manifest without downloading data.
    Update { language: String },
}
