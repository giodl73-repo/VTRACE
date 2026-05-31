# Language Profiles

## Scope

Repo: VTRACE

## Active Profiles

| Profile ID | Applicability | L0 | L1 | L2 |
|---|---|---|---|---|
| PROFILE-DOCS-001 | VTRACE framework, templates, roles, source maps, and self-trace docs. | `git diff --check` | `cargo run -- .`; source-custody inspection when sources change | Role/gate review in `docs/vtrace/REVIEW.md` |
| PROFILE-RUST-001 | `src/`, `Cargo.toml`, `Cargo.lock`, and validator tests. | `cargo fmt --check` | `cargo test`; `cargo run -- .` | Validator run against VTRACE and example repos |
| PROFILE-PYTHON-001 | Python examples. | `py -m py_compile examples\hello-world\src\hello_world.py`; `py -m py_compile examples\existing-repo-migration\src\report.py` | example CLI smoke checks | Example VTRACE packages validated by Rust validator |
| PROFILE-GEN-001 | Future schemas, generated reports, and generated evidence. | Source-of-truth and generation command identified | Regeneration produces expected diff or no diff | Generated output consumed by validator/example evidence |
| PROFILE-MULTI-001 | Future target repos with multiple language boundaries. | Per-language L0 checks named in `PACKAGE_BOUNDARIES.md` | Boundary integration command | End-to-end validation scenario |

## Tailoring Notes

| Profile ID | Local Override | Rationale | Reviewer |
|---|---|---|---|
| PROFILE-DOCS-001 | VTRACE self-trace requires more artifacts than target repo minimum slice. | VTRACE must prove itself thoroughly. | Systems Engineering Steward |
| PROFILE-RUST-001 | Validator stays std-only. | Keeps adoption lightweight, offline, and packageable. | Software Assurance Guardian |
| PROFILE-PYTHON-001 | Python remains example-only. | Keeps examples simple while the validator moves to Rust. | Adoption Usability Reviewer |
