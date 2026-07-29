# Lexinexo

> Find the path between ideas.

Lexinexo is an open-source semantic puzzle game built on top of **LXDB**, a local-first semantic knowledge engine written in Rust.

Unlike traditional word games, Lexinexo does not rely on predefined puzzles, APIs or cloud services.

Every challenge is generated from a compiled semantic graph that runs entirely on the user's device.

---

## Philosophy

Lexinexo is not the project.

LXDB is.

Lexinexo exists as the first application demonstrating what LXDB is capable of.

The long-term objective is to create an open semantic database format and engine that anyone can use for offline semantic applications.

---

## Workspace

```
apps/
    web/

crates/
    lxdb-core
    lxdb-engine
    lxdb-format
    lxdb-storage
    lxdb-compiler
    lxdb-cli

dictionary/

data/

docs/
```

---

## Goals

- Local-first
- No cloud
- No APIs
- Fast
- Cross-platform
- Open format
- Rust-first

---

## License

MIT