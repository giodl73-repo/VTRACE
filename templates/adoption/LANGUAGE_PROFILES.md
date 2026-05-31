# Language Profiles

## Scope

Repo or feature:

## Active Profiles

| Profile ID | Applicability | L0 | L1 | L2 |
|---|---|---|---|---|
| PROFILE-DOCS-001 | Docs-only standards/process changes. | `git diff --check` | VTRACE validator / source-custody inspection | Role/gate review |
| PROFILE-PYTHON-001 | Python scripts, packages, CLIs, or examples. | `py -m py_compile <files>` | unit tests / CLI smoke | integration or downstream scenario |
| PROFILE-RUST-001 | Rust crates, binaries, or workspaces. | `cargo fmt --check`; `cargo check` | `cargo clippy --all-targets -- -D warnings`; `cargo test` | integration/release scenario |
| PROFILE-TS-001 | TypeScript/JavaScript app or package. | typecheck or syntax check | lint/unit/component tests | build and route/API/user workflow |
| PROFILE-GEN-001 | Generated code, schemas, fixtures, reports. | generation command known | reproducible regeneration and diff review | downstream fixture/integration |
| PROFILE-MULTI-001 | Multi-language boundary or adapter. | per-language L0 | boundary integration command | end-to-end scenario |

## Tailoring Notes

| Profile ID | Local Override | Rationale | Reviewer |
|---|---|---|---|
|  |  |  |  |
