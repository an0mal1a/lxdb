# Lemmas and forms

`LexicalEntry.canonical` represents the preferred lemma-like form; `forms` preserves observed surface forms. Forms become independent accepted tokens and receive a directed `inflection_of` edge to the canonical token. This permits lookups such as `casas → casa` while retaining the original lexical layer for future sense-aware format extensions.
