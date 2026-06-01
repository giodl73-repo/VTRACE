# Pulse 17: CI Validation

## Goal

Add a repo-local GitHub Actions workflow that runs the same Rust validator
checks recorded in VTRACE evidence.

## Scope

- Add `.github/workflows/ci.yml`.
- Run Rust formatting, clippy, unit tests, VTRACE self-validation, and migration
  example validation on pushes and pull requests.
- Update self-trace DCR, work-package, evidence, verification, and trace records.

## Validation

- `git diff --check`
- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo run -- .`
- `cargo run -- examples\existing-repo-migration`

## Outcome

Complete. VTRACE now has a first-party CI workflow for the Rust validator path.

