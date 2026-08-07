# Contributing to LXDB

Thanks for helping improve LXDB. The project is a Cargo workspace, so changes
should be checked from the repository root and should keep the public crates
independently usable.

## Development workflow

1. Create a focused branch from `master`.
2. Make the smallest change that solves the problem and update documentation
   or tests when behavior changes.
3. Run the same checks used by CI:

   ```text
   cargo fmt --all --check
   cargo check --workspace --all-targets
   cargo test --workspace
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   ```

4. Open a pull request with a concise description, test results, and any
   compatibility or format-version impact.

## Source and dataset changes

Small fixtures belong in `crates/*/tests/fixtures` or `examples/`. Generated
datasets and downloaded source snapshots should stay out of Git unless they
are intentionally added as a reproducible fixture. Keep format changes
backwards-compatible when possible and document any new section or record in
`docs/FORMAT.md` and `docs/CHANGELOG.md`.

## Commit messages

Use an imperative, scoped subject when practical, for example:

```text
fix(cli): restore the end-to-end example fixture
```

Keep commits focused so they are easy to review and revert.
