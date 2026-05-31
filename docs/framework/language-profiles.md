# Language And Package Profiles

Language profiles tailor VTRACE validation to the repo boundary being changed.
They do not replace project-specific commands; they define the minimum command
shape a work package should consider before it closes.

## Profile Rules

- Every non-trivial `WP-*` names the applicable `PROFILE-*` or records why no
  profile applies.
- Profiles must define L0, L1, and L2 expectations.
- A profile may be docs-only, generated-code, package-specific, or
  multi-language.
- The target repo may override commands, but the override belongs in
  `PACKAGE_BOUNDARIES.md`, `VERIFICATION.md`, or `WORK_PACKAGES.md`.

## Standard Profiles

| Profile ID | Applicability | L0 | L1 | L2 |
|---|---|---|---|---|
| PROFILE-DOCS-001 | Docs-only standards/process repos. | `git diff --check`; source-custody inspection when sources change. | `py tools\vtrace_check.py .` when VTRACE package exists. | Role/gate review for public process claims. |
| PROFILE-PYTHON-001 | Python packages, scripts, CLIs, or examples. | `py -m py_compile <files>` or import smoke check. | Unit tests or CLI smoke command. | Integration scenario or downstream consumer check. |
| PROFILE-RUST-001 | Rust workspaces, crates, binaries, or libraries. | `cargo fmt --check`; `cargo check`. | `cargo clippy --all-targets -- -D warnings`; `cargo test`. | Feature/release/integration tests and unsafe/panic review when relevant. |
| PROFILE-TS-001 | TypeScript/JavaScript apps, packages, routes, or components. | package manager install status known; typecheck or syntax check. | unit/component tests and lint when available. | build, route/API integration, or browser/user workflow evidence. |
| PROFILE-GEN-001 | Generated code, schemas, fixtures, or reports. | Source-of-truth and generation command identified. | Regeneration is reproducible and diff is reviewed. | Generated artifact consumed by integration or downstream fixture. |
| PROFILE-MULTI-001 | Multi-language repo, adapter, FFI, or CLI/API boundary. | Per-language L0 checks for touched boundaries. | Integration command across the boundary. | End-to-end scenario plus rollback/compatibility evidence. |

## Review Questions

- Does the work package touch the package/language it claims?
- Are L0/L1/L2 commands realistic for this repo?
- Are generated artifacts separated from source of truth?
- Is public compatibility tested at the right boundary?
- Does the profile create enough evidence without forcing irrelevant tooling?
