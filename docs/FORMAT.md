# LXDB Format

An LXDB file begins with an 8-byte header (`LXDB`, major and minor version), followed by self-describing sections. Every section has a 12-byte header containing its type, flags and payload length. Readers locate sections rather than assuming offsets.

The current writer emits, in order: `Tokens`, `TokenStringTable`, `Relations` and `Adjacency`. `Metadata` is reserved and optional. Token records refer to UTF-8 strings through offsets relative to the string-table payload; adjacency records point to ranges in the relation-record table.

`lxdb-storage` validates the current header version, required sections, duplicate sections, supported flags, record section lengths and payload bounds. Applications should use storage and engine public APIs instead of parsing raw bytes.
