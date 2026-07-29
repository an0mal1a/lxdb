# Engine

The engine is responsible for querying semantic knowledge.

The engine never modifies LXDB.

Responsibilities:

- load database
- query tokens
- traverse graph
- calculate paths
- generate deterministic challenges

The engine has no knowledge about games or UI.