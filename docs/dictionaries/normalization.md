# Normalization

The pipeline trims input, discards control characters, collapses internal whitespace, preserves original casing and produces a Unicode lowercase comparison key. Spanish composed values and the common decomposed accent/diaeresis/tilde sequences are normalized to NFC-equivalent composed forms; tildes, `ñ`, and `ü` are never stripped.

Empty strings, URLs, markup delimiters, disallowed symbols, out-of-range lengths, disabled multiwords, and disabled proper nouns are rejected with a reason. Alphabetic words, valid hyphens and apostrophes remain valid.
