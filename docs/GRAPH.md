# Semantic Graph

LXDB represents knowledge as a graph.

Every concept is stored as a Token.

Connections between concepts are stored as Relations.

```
Token

↓

Relation

↓

Token
```

The graph is directional.

Relations have weights.

Applications may ignore weights if desired.

The engine exposes graph algorithms.

Applications never manipulate graph internals.