# LXDB Format

A LXDB file contains five logical sections.

```
Header

Dictionary

Relations

Indexes

Metadata
```

All offsets are stored inside the header.

Applications never parse bytes directly.

The reader crate exposes a safe API.