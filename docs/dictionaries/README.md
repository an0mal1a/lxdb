# Dictionary builds

`lxdb-dictionary` turns independently parsed linguistic sources into a validated LXDB dataset. It is application-agnostic: it does not contain game, user, score, or session rules.

```powershell
cargo run -p lxdb-cli -- dictionary languages
cargo run -p lxdb-cli -- dictionary build es --profile development --source-fixture .\crates\lxdb-dictionary\tests\fixtures --output .\target\dictionary-tests\es
```

The output directory contains `dictionary.lxdb`, `manifest.json`, `build-report.json`, `ATTRIBUTION.md`, and (unless disabled) `rejected-entries.jsonl.zst`. The last file is a valid Zstandard stream containing newline-delimited JSON records.

See [architecture](architecture.md), [Spanish](spanish.md), [configuration](configuration.md), and [reproducibility](reproducibility.md).
