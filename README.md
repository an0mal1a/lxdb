# LXDB

LXDB is a Rust toolkit for immutable binary semantic datasets. It compiles
human-readable `.lx` relations into a compact `.lxdb` file, validates the
format on load, and exposes lazy token and relation queries.

LXDB is deliberately application-agnostic. Games, search tools and language
products belong in downstream repositories and consume the public `lxdb` crate.

## Crates

- `lxdb`: public facade for ordinary applications.
- `lxdb-core`: identifiers and graph-domain models.
- `lxdb-format`: stable binary records and section headers.
- `lxdb-storage`: validated zero-copy byte views.
- `lxdb-engine`: lazy read-only queries.
- `lxdb-compiler`: `.lx` parser and binary writer.
- `lxdb-dictionary`: normalized language-source pipeline primitives.
- `lxdb-cli`: compiler, query, inspect and fixture-dictionary command line tool.

## Source format

Each non-empty, non-comment line is one weighted directed relation:

```text
source token -> target token : weight
```

Weights are `f32` values from `0.0` through `1.0`. Tokens are inferred from
relations; a token with no outgoing relations appears only as a target.

## Quick start

```powershell
cargo run -p lxdb-cli -- compile .\examples\knowledge.lx -o .\examples\knowledge.lxdb
cargo run -p lxdb-cli -- query .\examples\knowledge.lxdb rust
cargo run -p lxdb-cli -- inspect .\examples\knowledge.lxdb
```

As a library:

```toml
[dependencies]
lxdb = "0.1"
```

```rust
use lxdb::{BinaryDatasetExt, DatasetReader};

let dataset = DatasetReader::new().open("knowledge.lxdb")?;
let related = dataset.query().related_to("rust")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Verification

```powershell
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
cargo package --allow-dirty --no-verify -p lxdb
```
