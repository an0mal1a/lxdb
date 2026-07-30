# Semantic relations

Relation types are synonym, antonym, hypernym, hyponym, meronym, holonym, derived-from, inflection-of, related, translation and embedding-neighbor. Version 1 computes `clamp(base × source_confidence, 0, 1)`: synonym 1.00, antonym .92, hypernym/hyponym .88, meronym/holonym .82, related .78, derived .72, inflection .70.

Edges are deduplicated and stably ordered by source, target, type and provider. A profile caps outgoing explicit relations before compilation.
