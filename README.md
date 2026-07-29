# Lexinexo / LXDB

LXDB is a local-first binary format and query engine for immutable semantic datasets. Lexinexo is the first application planned on top of it: an offline semantic puzzle game that finds paths between ideas without cloud services or remote APIs.

## Architecture

```text
.lx source → lxdb-compiler → parser → validator → graph builder → binary writer
         → .lxdb → lxdb-storage → lxdb-engine → CLI / Lexinexo
```

- `lxdb-core`: domain IDs, graph and model types.
- `lxdb-format`: stable binary records, headers and section definitions.
- `lxdb-storage`: validated, zero-copy views over a `.lxdb` byte buffer.
- `lxdb-engine`: lazy read-only token and relation queries.
- `lxdb-compiler`: source parser and deterministic binary writer.
- `lxdb-cli`: compile, query and inspect commands.
- `apps/web`: future Lexinexo web application shell.

## `.lx` source format

Each non-empty, non-comment line declares a directed relation and implicitly declares its two tokens:

```text
source token -> target token : weight
```

`weight` is an `f32` from `0.0` to `1.0`, inclusive. Whitespace around the source, `->`, target and `:` is ignored. Lines whose trimmed text starts with `#` and empty lines are ignored. Inline comments are not supported. Tokens cannot reference themselves.

There is currently no standalone token declaration: a token with no outgoing relations is represented by using it only as a relation target.

See [examples/knowledge.lx](examples/knowledge.lx) for a complete source dataset.

## End-to-end example

```powershell
cargo run -p lxdb-cli -- compile ./examples/knowledge.lx -o ./examples/knowledge.lxdb
cargo run -p lxdb-cli -- query ./examples/knowledge.lxdb rust
cargo run -p lxdb-cli -- inspect ./examples/knowledge.lxdb
```

The query prints:

```text
rust
├─ 0.950 → language
├─ 0.700 → compiler
└─ 0.880 → memory
```

`query` reports an absent token without failing; malformed sources, unreadable datasets and invalid binary files return a non-zero exit code.

## Current status and limits

The MVP supports compiling a relation dataset, reading its validated binary sections, zero-copy token text resolution, outgoing relation queries and structural inspection. The source language does not yet support metadata or standalone tokens, and the engine does not yet include graph search, similarity or challenge generation. The format currently writes the required token, string-table, relation and adjacency sections; metadata is reserved for a future compatible extension.

## Verification

```powershell
cargo fmt --all
cargo check --workspace
cargo test --workspace
```

## License

MIT
