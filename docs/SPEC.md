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

A compiler produces LXDB snapshots.

Readers consume them.

LXDB is append-only.

A new dataset always produces a new file.

Never an update.