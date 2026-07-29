# Compiler

The compiler turns a text `.lx` source into an immutable `.lxdb` dataset.

```text
Parser → Validator → GraphBuilder → Writer → filesystem
```

The parser accepts `source -> target : weight` relations, infers the token table from both endpoints and preserves the first token occurrence order. The validator rejects weights outside `0.0..=1.0` and self references. The graph builder assigns token IDs, sorts relations by source token and builds one adjacency record for each token. The writer emits the current LXDB header followed by token, string table, relation and adjacency sections.

Compilation is deterministic for the same source and format version.
