# Pulse 16: Rust Validator

## Goal

Port the local VTRACE artifact validator from Python to Rust while preserving
the current validation contracts and self-trace evidence.

## Scope

- Add a std-only Rust crate with `vtrace` binary and reusable library checks.
- Preserve validator behavior for required files, trace visibility, evidence
  pointers, evidence ledger completeness, work-package shape, review lanes,
  review checklists, and language profiles.
- Remove the Python validator and Python unit test implementation.
- Update VTRACE self-trace, language profiles, package boundaries, evidence,
  and validation commands to use Cargo.

## Validation

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo run -- .`
- `cargo run -- examples\existing-repo-migration`

## Outcome

Complete. The validator is now a Rust CLI with 8 unit tests and no third-party
crate dependency.
