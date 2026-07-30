# Reproducibility

Use a fixed profile, config, source snapshot directory and limit. The pipeline uses sorted maps/sets for merging and relation output, then writes the dataset through `lxdb-compiler`, reopens it with `DatasetReader`, and records an FNV-1a 64 artifact digest in `manifest.json`.

`--offline` rejects a missing cache. Atomic temporary files prevent partially written output artifacts from being treated as successful builds.
