# Configuration

Versioned policy files live in `config/dictionaries/`: `es.toml`, `en.toml`, and `profiles/{development,game,full}.toml`. The CLI accepts `--profile`, `--config`, `--limit`, `--cache-dir`, `--offline`, `--refresh`, `--source-fixture`, source-disable switches, `--emit-source`, and `--keep-intermediate`.

Profiles are deterministic presets: development caps merged lemmas at 25,000 and 16 outgoing relations; game caps at 200,000 merged lemmas and allows 48 relations; full allows multiword terms, proper names and 96. Embeddings are an optional future offline provider and do not run in consumers or tests.
