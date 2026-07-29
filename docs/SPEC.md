# LXDB Specification

LXDB is a binary format designed to store semantic knowledge.

The format is intended to be:

- deterministic
- portable
- immutable
- versioned
- local-first

LXDB does not know anything about games.

It only represents semantic knowledge.

Applications consume LXDB.

Applications never modify LXDB.

A compiler produces LXDB snapshots, and readers consume them without modifying their bytes. Creating a new dataset writes a new file; in-place updates are not part of the current format API.
