# Sources

- Kaikki/Wiktextract is the main lexical input. Kaikki publishes structured JSONL, one JSON object per line; Spanish coverage and snapshots are displayed on its Spanish dictionary page.
- LibreOffice/Hunspell provides orthographic entries only. Its `.dic` source is line-oriented and paired with an `.aff` file for the complete morphology rules. The first version retains listed forms and never invents semantic links from Hunspell.
- Open Multilingual WordNet / Spanish WordNet uses WN-LMF lexical entries, senses, synsets and `SynsetRelation` elements. It contributes same-synset synonyms and curated conceptual relations when present.
- Frequency is a simple exported `word<TAB>zipf` file. It ranks/filter-quality data only and is never a semantic weight.

Builds are cache-first. `lxdb dictionary build es --profile game` downloads the verified Kaikki Spanish JSONL snapshot and LibreOffice `es_ES.dic` into `.lxdb/cache/dictionaries/es/` on first use, then reuses them. `--offline` refuses a cache miss and `--refresh` replaces cached files. Pass `--source-fixture` for deterministic CI.
