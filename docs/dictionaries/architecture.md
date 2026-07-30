# Architecture

`source` streams Kaikki JSONL and Hunspell dictionaries and reads a WN-LMF adapter for WordNet. Parsed records become `LexicalEntry`, preserving source references, forms, senses, frequency and typed relations. `pipeline` normalizes, filters, merges, calculates generic semantic quality, renders a deterministic `.lx` intermediary, and delegates binary encoding to `lxdb-compiler`.

The existing LXDB 0.1 binary format retains only token text and weighted directed relations. Lexical relation kinds, provenance, forms, frequencies and quality therefore remain in the build artifacts and intermediate model. No binary layout is changed, so 0.1 readers remain compatible.

The `.lx` grammar additionally accepts `token <text>` for accepted isolated words. This is backward compatible with relation-only source files and avoids fake self-relations.
