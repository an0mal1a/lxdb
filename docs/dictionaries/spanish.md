# Spanish reference pipeline

Spanish (`es`, `spa`) is the reference pipeline. A build is complete only relative to the snapshots and configuration recorded in its generated manifest; it does **not** claim to include every Spanish word universally.

The fixture exercises accents, `ñ`, `ü`, plural forms, a hyphenated valid term, explicit Kaikki links, WN-LMF same-synset synonyms and hypernyms, and frequency coverage. The validation vocabulary includes `casa`, `casas`, `árbol`, `niño`, `pingüino`, `acción`, `hacer`, `hecho`, `rápido`, `rápidamente`, `océano`, `electricidad`, `vehículo`, `automóvil`, `perro`, and `animal`.

```powershell
lxdb dictionary build es --profile development --source-fixture .\crates\lxdb-dictionary\tests\fixtures --output .\target\dictionary-tests\es
lxdb inspect .\target\dictionary-tests\es\dictionary.lxdb
lxdb query .\target\dictionary-tests\es\casa
```
