# LXDB

[![CI](https://github.com/an0mal1a/lxdb/actions/workflows/ci.yml/badge.svg)](https://github.com/an0mal1a/lxdb/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/github/v/release/an0mal1a/lxdb?include_prereleases)](https://github.com/an0mal1a/lxdb/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**LXDB is an open-source binary format, compiler and zero-copy query engine for multilingual semantic datasets.**

It provides the infrastructure required to build, validate, store and efficiently query large lexical or semantic graphs.

LXDB was originally created as the semantic subsystem behind [Lexicon](https://github.com/an0mal1a/lexicon), but it is designed as an independent and reusable project.

> LXDB stores and queries semantic knowledge.  
> Applications decide what to do with that knowledge.

---

## Why LXDB?

Language datasets can contain hundreds of thousands of words and millions of relationships.

Loading those datasets into conventional in-memory structures can introduce:

- long startup times;
- high memory usage;
- unnecessary allocations;
- duplicated strings;
- complex application-specific preprocessing.

LXDB compiles semantic data into a validated binary representation that applications can query without rebuilding the complete graph in memory.

Its main goals are:

- compact binary storage;
- deterministic compilation;
- validated datasets;
- zero-copy access where possible;
- lazy relation traversal;
- language-independent architecture;
- reproducible dictionary generation;
- clean separation between data infrastructure and applications.

---

## Project status

LXDB is currently under active development. The workspace has a reproducible
compiler → storage → query workflow covered by integration tests and CI.
The roadmap and current limitations are tracked in [ROADMAP.md](ROADMAP.md).

The project is stabilizing its first usable end-to-end workflow:

```text
linguistic sources
    ↓
dictionary pipeline
    ↓
semantic graph
    ↓
LXDB compiler
    ↓
.lxdb dataset
    ↓
storage validation
    ↓
zero-copy query engine
    ↓
CLI or application
```

---

## Workspace

```
lxdb/
├── crates/
│   ├── lxdb-core/
│   ├── lxdb-format/
│   ├── lxdb-storage/
│   ├── lxdb-engine/
│   ├── lxdb-compiler/
│   ├── lxdb-dictionary/
│   └── lxdb-cli/
├── config/
├── dictionary/
├── examples/
├── docs/
├── Cargo.toml
└── README.md
```

---

## Crates 

| Crate             | Responsibility                                                         |
| ----------------- | ---------------------------------------------------------------------- |
| `lxdb-core`       | Domain primitives, strongly typed IDs and semantic graph models        |
| `lxdb-format`     | Binary format definitions, records, sections and format validation     |
| `lxdb-storage`    | Safe loading and structural validation of `.lxdb` datasets             |
| `lxdb-engine`     | Lazy and zero-copy dataset queries                                     |
| `lxdb-compiler`   | Source parsing, validation, graph construction and binary writing      |
| `lxdb-dictionary` | Multilingual lexical source ingestion and dataset generation           |
| `lxdb-cli`        | Command-line interface for compiling, inspecting and querying datasets |

---

## Core architecture

LXDB is divided into independent layers:
```
                ┌─────────────────────┐
                │    lxdb-dictionary  │
                │ external sources    │
                └──────────┬──────────┘
                            │
                ┌──────────▼──────────┐
                │    lxdb-compiler    │
                │ source → graph      │
                └──────────┬──────────┘
                            │
                ┌──────────▼──────────┐
                │     lxdb-format     │
                │ binary contract     │
                └──────────┬──────────┘
                            │
                ┌──────────▼──────────┐
                │    lxdb-storage     │
                │ loading/validation  │
                └──────────┬──────────┘
                            │
                ┌──────────▼──────────┐
                │     lxdb-engine     │
                │ query/traversal     │
                └─────────────────────┘
```

Applications depend on the public LXDB APIs.

LXDB never depends on application-specific rules.

---

## Binary dataset

An LXDB file is divided into validated sections.

The current format includes sections conceptually equivalent to:

- metadata;
- tokens;
- token string table;
- relations;
- adjacency records.

The exact binary contract is documented in
[`docs/FORMAT.md`](docs/FORMAT.md).

---

## Quick start

### Requirements
- Rust stable
- Cargo

Check your toolchain: 
```
rustc --version
cargo --version
```
### Build the workspace
```cargo build --workspace```

### Run all tests
`cargo test --workspace`

### Check formatting
`cargo fmt --all --check`

### Run Clippy
`cargo clippy --workspace --all-targets --all-features -- -D warnings`

---

## Command-line interface

Run the CLI through Cargo:

`cargo run -p lxdb-cli -- --help`

Install it locally:

`cargo install --path crates/lxdb-cli`

After installation, the binary is available as:

`lxdb`

## Compile a source dataset
`lxdb compile examples/knowledge.lx --output examples/knowledge.lxdb`

Or through Cargo:

`cargo run -p lxdb-cli -- compile examples/knowledge.lx --output examples/knowledge.lxdb`

The accepted source syntax is documented in
[`docs/COMPILER.md`](docs/COMPILER.md).

Do not assume the textual source format is identical to the binary format. The source format is an authoring and testing interface; `.lxdb` is the runtime representation.

## Inspect a dataset

```bash
lxdb inspect examples/knowledge.lxdb
```

Example output:

```text
Dataset: knowledge.lxdb
Version: 0.1
Tokens: 7
Relations: 6
Adjacency records: 7
Token string table: 51 bytes
File size: 483 bytes
```

---

## Query a token

```bash
lxdb query examples/knowledge.lxdb rust
```

Example:

```text
rust
├─ 0.950 → language
├─ 0.700 → compiler
└─ 0.880 → memory
```

The exact output may evolve with the CLI.

---

## Generate a language dataset

The dictionary subsystem transforms external linguistic sources into an LXDB dataset.

Example:

```bash
lxdb dictionary build es \
  --output datasets/generated/es.lxdb
```

A development-sized build:

```bash
lxdb dictionary build es \
  --output datasets/generated/es-dev.lxdb \
  --limit 50000
```

Force source updates:

```bash
lxdb dictionary build es \
  --output datasets/generated/es.lxdb \
  --refresh
```

Use only locally cached sources:

```bash
lxdb dictionary build es \
  --output datasets/generated/es.lxdb \
  --offline
```

List supported languages:

```bash
lxdb dictionary languages
```

Inspect the provenance of a generated dataset:

```bash
lxdb dictionary sources datasets/generated/es.lxdb
```

See the complete process in:

```text
docs/dictionaries/README.md
```

---

## Dictionary pipeline

The dictionary generator follows a reproducible pipeline:

```text
language profile
    ↓
source resolution
    ↓
download/cache
    ↓
streaming extraction
    ↓
Unicode normalization
    ↓
lexical filtering
    ↓
deduplication
    ↓
lemma and form resolution
    ↓
semantic relation extraction
    ↓
optional offline similarity generation
    ↓
graph construction
    ↓
LXDB compilation
    ↓
dataset validation
    ↓
manifest and attribution output
```

A generated dataset must include enough metadata to identify:

* language;
* creation timestamp;
* LXDB format version;
* source snapshots;
* source licenses;
* generator version;
* normalization profile;
* filtering profile;
* relation strategy;
* optional embedding model and version.

---

## Dataset provenance

Generated datasets are reproducible artifacts, not opaque binaries.

Every production dataset should be accompanied by a manifest:

```text
datasets/generated/es/
├── dictionary.lxdb
├── manifest.json
├── ATTRIBUTION.md
└── build-report.json
```

Example manifest:

```json
{
  "language": "es",
  "format_version": "0.1",
  "generator_version": "0.1.0-alpha.1",
  "created_at": "2026-07-30T10:00:00Z",
  "sources": [
    {
      "name": "Kaikki Spanish",
      "snapshot": "2026-07",
      "license": "CC BY-SA / GFDL"
    }
  ],
  "normalization": {
    "unicode": "NFC",
    "case_policy": "preserve-and-fold",
    "include_multiword_terms": true
  }
}
```

Generated files under `datasets/generated/` are normally excluded from Git because they can be large and reproducible.

Small deterministic fixtures may be committed under:

```text
datasets/fixtures/
```

---

## Using LXDB from Rust

```toml
[dependencies]
lxdb-storage = "0.1"
lxdb-engine = "0.1"
```

Until crates are published, use the Git repository:

```toml
[dependencies]
lxdb-storage = {
    git = "https://github.com/an0mal1a/lxdb",
    tag = "v0.1.0-alpha.1"
}

lxdb-engine = {
    git = "https://github.com/an0mal1a/lxdb",
    tag = "v0.1.0-alpha.1"
}
```

Example:

```rust
use lxdb_engine::BinaryDatasetExt;
use lxdb_storage::DatasetReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dataset = DatasetReader::new().open("datasets/es.lxdb")?;
    let query = dataset.query();

    if let Some(token) = query.token_by_text("casa")? {
        println!("Token: {}", token.text()?);

        for relation in query.resolved_outgoing(token.id())? {
            println!(
                "{} -> {} ({:.3})",
                relation.source().text()?,
                relation.target().text()?,
                relation.weight()
            );
        }
    }

    Ok(())
}
```

Consult the crate-level documentation for the exact API supported by the current release.

---

## Performance principles

LXDB follows several architectural rules:

* datasets are validated when opened;
* string data is resolved from shared tables;
* records are iterated lazily;
* runtime queries avoid rebuilding the complete graph;
* allocations are avoided where borrowing is possible;
* large source files are processed in streaming mode;
* generated output is written atomically;
* binary compatibility is versioned explicitly.

Benchmark notes will be added under `docs/` as the benchmark suite grows.

---

## Relationship semantics

Relations may represent:

* synonymy;
* antonymy;
* hypernymy;
* hyponymy;
* derivation;
* inflection;
* translation;
* general semantic proximity;
* embedding-generated neighbors.

A relation's numeric weight represents confidence or semantic strength according to the dataset's declared relation strategy.

Frequency is not treated as semantic similarity.

Dataset-specific semantics must be recorded in the manifest.

---

## Development datasets

Small fixtures are intended for:

* compiler tests;
* storage validation;
* engine tests;
* API development;
* examples;
* CI.

The checked-in fixtures live under:

```text
crates/lxdb-dictionary/tests/fixtures/
```

Large generated dictionaries should not be committed directly.

---

## Documentation

```text
docs/
├── ARCHITECTURE.md
├── FORMAT.md
├── COMPILER.md
├── ENGINE.md
├── GRAPH.md
├── SPEC.md
├── CHANGELOG.md
├── dictionaries/
│   ├── README.md
│   ├── sources.md
│   ├── spanish.md
│   ├── normalization.md
│   ├── semantic-relations.md
│   └── reproducibility.md
└── ...
```

The top-level [ROADMAP.md](ROADMAP.md) tracks planned work. See
[CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and quality
checks expected in every change.

---

## Relationship with Lexicon

[Lexicon](https://github.com/an0mal1a/lexicon) is the first game built on LXDB.

Lexicon uses LXDB to:

* validate player words;
* obtain semantic proximity;
* explore relationships;
* generate playable challenges;
* calculate deterministic game results.

Game rules, player sessions, daily challenges and statistics belong to Lexicon, not LXDB.

---

## Roadmap

### v0.1

* stable source-to-binary compilation;
* validated dataset loading;
* token and relation queries;
* CLI compile, inspect and query commands;
* deterministic fixtures;
* documented binary format.

### v0.2

* multilingual dictionary pipeline;
* source provenance;
* lexical metadata;
* relation types;
* Spanish reference dataset.

### v0.3

* optimized indexes;
* configurable semantic paths;
* dataset packaging;
* benchmark suite;
* broader language support.

---

## Contributing

Contributions are welcome while the project is evolving.

Before opening a pull request:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

Changes to the binary format must include:

* compatibility analysis;
* tests;
* format documentation;
* an architecture decision record when appropriate.

---

## Security

Do not open untrusted datasets without using the validated storage APIs.

Please report security issues privately instead of publishing exploitable details in a public issue.

---

## License

LXDB is licensed under either of:

* Apache License, Version 2.0
* MIT License

at your option.

Generated dictionaries may contain data from sources with separate attribution or redistribution requirements. See each dataset's `ATTRIBUTION.md` and manifest before redistributing it.
