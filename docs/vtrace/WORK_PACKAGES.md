# Work Packages

## Scope

Repo: VTRACE

## Work Package Table

| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |
|---|---|---|---|---|---|---|---|
| WP-001 | Add lightweight validators. | DCR-001 / REQ-VAL-001 / SPEC-007 / CR-001..CR-003 | `src/`, `Cargo.toml`, `Cargo.lock`, docs | Artifact contracts stable. | Validator catches missing files, ID coverage gaps, undefined evidence, incomplete WP shape, and incomplete review lanes. | L0: `cargo fmt --check` / L1: `cargo test`; `cargo run -- .` / L2: run on example repos | complete |
| WP-002 | Add language/package profiles. | DCR-002 / REQ-PROFILE-001 / SPEC-008 / CR-004 | `docs/framework/language-profiles.md`, `templates/adoption/LANGUAGE_PROFILES.md`, `docs/vtrace/LANGUAGE_PROFILES.md` | At least one target repo need identified. | Profiles define applicability, commands, boundaries, and review lanes. | L0: docs sanity / L1: `cargo run -- .` / L2: profile used in migration example | complete |
| WP-003 | Add realistic existing-repo migration example. | DCR-003 / REQ-EXAMPLE-001 / CON-001 / SPEC-004 / SPEC-005 / SPEC-009 | `examples/existing-repo-migration/` | Candidate scenario selected. | Example shows current/target spec baseline and one closed WP. | L0: `py -m py_compile examples\existing-repo-migration\src\report.py` / L1: `py examples\existing-repo-migration\src\report.py` / L2: `cargo run -- examples\existing-repo-migration` | complete |
| WP-004 | Add reusable evidence ledger template/schema and automation. | DCR-004 / REQ-EVIDENCE-001 / IF-003 / SPEC-005 / SPEC-010 | `templates/adoption/EVIDENCE.md`, `schemas/evidence-ledger.schema.json`, validator | VTRACE self-ledger fields proven. | Target repos can adopt `EVIDENCE.md`; schema defines row shape; validator checks trace references and evidence row completeness. | L0: docs sanity / L1: validator tests / L2: used in migration example | complete |
| WP-005 | Add gate-specific checklists. | DCR-005 / REQ-GATE-001 / SPEC-001 / SPEC-005 / SPEC-011 | `docs/framework/gate-checklists.md`, `templates/adoption/REVIEW_CHECKLISTS.md`, `docs/vtrace/REVIEW_CHECKLISTS.md`, validator | Gate list stable. | Major gates have checklist guidance and required checklist items are validator-checked when present. | L0: docs sanity / L1: validator tests / L2: self-gate checklist | complete |
| WP-006 | Deepen NASA specificity encoding. | DCR-006 / REQ-NASA-001 / NEED-002 / SPEC-001 / SPEC-012 | `docs/framework/nasa-technical-controls.md` | Source rights posture confirmed. | Derived guidance covers technical reviews, traceability, assurance, configuration management, interfaces, V&V, and technical data. | L0: source custody review / L1: framework inspection / L2: role review | complete |
| WP-007 | Port validator to Rust. | DCR-007 / REQ-RUST-001 / REQ-VAL-001 / SPEC-007 / SPEC-013 / IF-005 | `src/`, `Cargo.toml`, `Cargo.lock`, `.gitignore`, validator docs | Python validator behavior is understood and tests are portable. | Rust validator preserves current checks, unit tests pass, and self/example packages validate. | L0: `cargo fmt --check` / L1: `cargo test`; `cargo run -- .` / L2: `cargo run -- examples\existing-repo-migration` | complete |
| WP-008 | Add CI validation workflow. | DCR-008 / REQ-CI-001 / REQ-RUST-001 / SPEC-014 / IF-006 | `.github/workflows/ci.yml`, self-trace docs | Rust validator path is stable locally. | CI workflow runs formatting, clippy, tests, self-validation, and migration-example validation on push and pull request. | L0: workflow inspection / L1: local command parity / L2: first remote CI run after push | complete |

## Orphan Check

- [x] Every accepted `REQ-*` is assigned to current evidence, a DCR, or a work package.
- [x] Every accepted `SPEC-*` is assigned to current evidence or future work.
- [x] Every future implementation slice has a DCR parent.
- [x] Every proposed work package has L0/L1/L2 expectations.
- [x] Tooling/profile gaps are visible instead of hidden in prose.
