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
        /// Output directory; it will contain dictionary.lxdb and build artifacts.
        #[arg(short, long, value_name = "OUTPUT")]
        output: PathBuf,
        /// Deterministic directory containing source fixtures.
        #[arg(long, value_name = "DIRECTORY")]
        source_fixture: Option<PathBuf>,
        /// Dictionary profile.
        #[arg(long, default_value = "development", value_parser = ["development", "game", "full"])]
        profile: String,
        /// Optional per-language TOML configuration path (recorded by the build).
        #[arg(long, value_name = "PATH")]
        config: Option<PathBuf>,
        /// Bound the number of merged lemmas.
        #[arg(long)]
        limit: Option<usize>,
        /// Ignore existing source cache (used by external source provisioners).
        #[arg(long)]
        refresh: bool,
        /// Do not consult network providers; require a fixture or local cache.
        #[arg(long)]
        offline: bool,
        /// Root directory for cached source snapshots.
        #[arg(long, value_name = "DIRECTORY")]
        cache_dir: Option<PathBuf>,
        #[arg(long)]
        without_kaikki: bool,
        #[arg(long)]
        without_hunspell: bool,
        #[arg(long)]
        without_wordnet: bool,
        #[arg(long)]
        without_frequency: bool,
        #[arg(long)]
        without_embeddings: bool,
        /// Copy the normalized compiler source to this path.
        #[arg(long, value_name = "PATH")]
        emit_source: Option<PathBuf>,
        #[arg(long)]
        keep_intermediate: bool,
        /// Do not emit rejected-entries.jsonl.zst.
        #[arg(long)]
        no_rejected_entries: bool,
    },
    /// Refresh the local source manifest without downloading data.
    Update {
        language: String,
        #[arg(long, value_name = "DIRECTORY")]
        cache_dir: Option<PathBuf>,
    },
    /// Display a concise summary of a generated manifest.
    Inspect { manifest: PathBuf },
}
