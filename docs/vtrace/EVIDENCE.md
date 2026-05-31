# Evidence Ledger

## Scope

Repo: VTRACE self-adoption

This ledger records objective evidence for the current VTRACE readiness claim.
Command receipts should be updated when validation is run.

## Evidence Records

| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status |
|---|---|---|---|---|---|
| EVID-001 | inspection | `docs/framework/vtrace-process.md` | V-model process and stage rules exist. | Present and traced by `REQ-001`. | passed |
| EVID-002 | inspection | `templates/adoption/` | Adoption templates exist for required stages. | Present and traced by `REQ-002`. | passed |
| EVID-003 | inspection | `skills/vtrace-assess`, `skills/vtrace-adopt`, `skills/vtrace-gate` | Assessment, adoption, and gate skills exist. | Present and traced by `REQ-003`. | passed |
| EVID-004 | inspection | `docs/framework/specification-baselines.md`, `templates/adoption/SPECIFICATION_BASELINE.md` | Specification baseline guidance and template exist. | Present and traced by `REQ-004`. | passed |
| EVID-005 | inspection | `docs/framework/implementation-management.md`, `templates/adoption/WORK_PACKAGES.md` | Work-package execution rules exist. | Present and traced by `REQ-005`. | passed |
| EVID-006 | inspection | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | DCRs map to proposed work packages. | Present and traced by `REQ-006`. | passed |
| EVID-007 | source custody | `py -m json.tool sources\source-registry.json` | Source registry parses as JSON. | Passed on 2026-05-31. | passed |
| EVID-008 | docs sanity | `git diff --check` | No whitespace errors. | Passed on 2026-05-31. | passed |
| EVID-009 | executable example | `py -m py_compile examples\hello-world\src\hello_world.py` | Hello-world compiles. | Passed on 2026-05-31. | passed |
| EVID-010 | executable example | `py examples\hello-world\src\hello_world.py` | Prints `Hello, VTRACE!`. | Printed `Hello, VTRACE!` on 2026-05-31. | passed |
| EVID-011 | review | `docs/vtrace/REVIEW.md` | Self-adoption gate records decision and findings. | `pass_with_risk`. | passed |
| EVID-012 | role review | `.roles/` against `docs/vtrace/` | Required review lanes are represented and deferred requirements remain trace-visible. | Role-lane fixes recorded in `REVIEW.md` and `TRACE.md` on 2026-05-31. | passed |
| EVID-013 | automated test | `cargo test` | Validator unit tests pass. | 8 tests passed on 2026-05-31. | passed |
| EVID-014 | local command | `cargo run -- .` | VTRACE self-package validates. | Printed `VTRACE validation passed` on 2026-05-31. | passed |
| EVID-015 | inspection / validator | `docs/vtrace/LANGUAGE_PROFILES.md`; `cargo run -- .` | Profiles declare applicability and L0/L1/L2 levels. | Passed on 2026-05-31. | passed |
| EVID-016 | command | `py -m py_compile examples\existing-repo-migration\src\report.py` | Migration example compiles. | Passed on 2026-05-31. | passed |
| EVID-017 | command | `py examples\existing-repo-migration\src\report.py` | Prints `actuator: ready`. | Passed on 2026-05-31. | passed |
| EVID-018 | command | `cargo run -- examples\existing-repo-migration` | Migration example VTRACE package validates. | Passed on 2026-05-31. | passed |
| EVID-019 | automated test / schema inspection | `cargo test`; `schemas/evidence-ledger.schema.json` | Evidence ledger checks are covered and row schema exists. | Passed on 2026-05-31. | passed |
| EVID-020 | automated test / inspection | `docs/vtrace/REVIEW_CHECKLISTS.md`; validator checklist tests | Gate checklist rows close. | Passed on 2026-05-31. | passed |
| EVID-021 | source-custody inspection | `docs/framework/nasa-technical-controls.md`; `docs/vtrace/SOURCE_BASIS.md` | NASA-inspired controls are derived guidance, not compliance claims. | Passed on 2026-05-31. | passed |
| EVID-022 | automated test | `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; `cargo test` | Rust validator is formatted, lint-clean, and tested. | Passed on 2026-05-31. | passed |
| EVID-023 | local command | `cargo run -- .` | Rust validator validates VTRACE self-package. | Printed `VTRACE validation passed` on 2026-05-31. | passed |
| EVID-024 | local command | `cargo run -- examples\existing-repo-migration` | Rust validator validates migration example package. | Printed `VTRACE validation passed` on 2026-05-31. | passed |

## Evidence Rules

- Evidence must point to a file, command, CI run, review record, artifact, or
  explicit inspection target.
- Evidence for public claims must be reproducible or reviewable.
- A future validator may generate this ledger, but this Markdown file is the
  current source of truth.
