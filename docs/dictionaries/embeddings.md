# Offline embeddings

The embedding provider is intentionally optional. A future provider must accept a locally supplied, versioned model, process batches, build an approximate neighbor index, apply top-K and threshold controls, and emit deterministic records where possible. It is never required to open or query an `.lxdb` file and is not invoked by the fixture pipeline.
