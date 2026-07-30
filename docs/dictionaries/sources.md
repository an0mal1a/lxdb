# Sources

- Kaikki/Wiktextract is the main lexical input. Kaikki publishes structured JSONL, one JSON object per line; Spanish coverage and snapshots are displayed on its Spanish dictionary page.
- LibreOffice/Hunspell provides orthographic entries only. Its `.dic` source is line-oriented and paired with an `.aff` file for the complete morphology rules. The first version retains listed forms and never invents semantic links from Hunspell.
- Open Multilingual WordNet / Spanish WordNet uses WN-LMF lexical entries, senses, synsets and `SynsetRelation` elements. It contributes same-synset synonyms and curated conceptual relations when present.
- Frequency is a simple exported `word<TAB>zipf` file. It ranks/filter-quality data only and is never a semantic weight.

Builds are local-first. Pass `--source-fixture` for deterministic CI, or place verified snapshots under `.lxdb/cache/dictionaries/<language>/` with the same input names. `dictionary update` creates a cache manifest; it does not silently download a multi-gigabyte dump.
