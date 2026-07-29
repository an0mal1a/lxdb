# LXDB Binary Format Specification

## File Layout

Every LXDB file is composed of the following sections.

+----------------------+
| Header               |
+----------------------+
| Metadata             |
+----------------------+
| Token Records        |
+----------------------+
| Token String Table   |
+----------------------+
| Relation Records     |
+----------------------+
| Adjacency Index      |
+----------------------+

Each section is preceded by a `SectionHeader`.

Sections may appear in a different order in future versions.

Readers MUST use the section headers instead of assuming offsets.

---

## Header

Contains:

- Magic bytes
- Format version

---

## Metadata

Contains dataset information such as:

- Dataset ID
- Language
- Version
- Name

---

## Token records

Each token record occupies 24 bytes.

| Offset | Size | Field |
|---:|---:|---|
| 0 | 4 | Token ID |
| 4 | 4 | Reserved |
| 8 | 8 | String table offset |
| 16 | 4 | String length |
| 20 | 4 | Reserved |

The string table offset is relative to the beginning of the `TokenStringTable` section payload.

---

## Token String Table

A contiguous UTF-8 byte buffer.

Strings are never null terminated.

Each TokenRecord specifies:

- offset
- length

---

## Relation Records

Array of `RelationRecord`.

Relations are sorted by source token.

---

## Adjacency Index

Array of `AdjacencyRecord`.

One record per token.

Allows O(1) lookup of outgoing relations.

---

## Design Goals

- Zero-copy where possible
- Forward compatible
- Compact
- Sequential reads
- Immutable
- Language agnostic