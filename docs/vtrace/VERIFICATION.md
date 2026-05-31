# Verification

## Scope

Repo: VTRACE

## Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Result | Status | Evidence |
|---|---|---|---|---|---|
| REQ-001 | inspection | Inspect `docs/framework/vtrace-process.md` | V-model process and stage rules exist. | passed | EVID-001 |
| REQ-002 | inspection | Inspect `templates/adoption/` | Required adoption templates exist. | passed | EVID-002 |
| REQ-003 | inspection | Inspect `skills/vtrace-*` | Assessment, adoption, and gate skills exist. | passed | EVID-003 |
| REQ-004 | inspection | Inspect spec baseline guidance and template | Spec baseline required before non-trivial implementation. | passed | EVID-004 |
| REQ-005 | inspection | Inspect implementation management and work-package template | Work packages require parent IDs, V closure, validation levels, and role lanes. | passed | EVID-005 |
| REQ-006 | inspection | Inspect `CHANGE_CONTROL.md` and `WORK_PACKAGES.md` | DCRs map to future work packages. | passed | EVID-006 |
| REQ-VAL-001 | automated test / local command | `cargo test`; `cargo run -- .` | Validator tests pass and VTRACE self-package validates. | passed | EVID-013, EVID-014 |
| REQ-PROFILE-001 | inspection / validator | Inspect language-profile docs and run validator. | Profiles define applicability and L0/L1/L2 levels. | passed | EVID-015 |
| REQ-EXAMPLE-001 | command / validator | Compile/run migration example and validate its VTRACE package. | Example proves current/target migration with closed WP. | passed | EVID-016, EVID-017, EVID-018 |
| REQ-EVIDENCE-001 | automated test / inspection | Validator unit tests and evidence template inspection. | Evidence ledger rows are reusable and validator-checked. | passed | EVID-019 |
| REQ-GATE-001 | automated test / inspection | Validator unit tests and review checklist inspection. | Required checklist rows close. | passed | EVID-020 |
| REQ-NASA-001 | source-custody inspection | Inspect derived control map and source basis. | No compliance or endorsement overclaim. | passed | EVID-021 |
| REQ-RUST-001 | automated test / local command | `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; `cargo test`; `cargo run -- .` | Rust validator is formatted, lint-clean, tested, and validates VTRACE. | passed | EVID-022, EVID-023 |

## Validation Commands

| Level | Command | Status |
|---|---|---|
| L0 | `git diff --check` | passed |
| L1 | `py -m json.tool sources\source-registry.json` | passed |
| L1 | `py -m py_compile examples\hello-world\src\hello_world.py` | passed |
| L1 | `py examples\hello-world\src\hello_world.py` | passed |
| L1 | `cargo fmt --check` | passed |
| L1 | `cargo clippy --all-targets -- -D warnings` | passed |
| L1 | `cargo test` | passed |
| L1 | `cargo run -- .` | passed |
| L1 | `py -m py_compile examples\existing-repo-migration\src\report.py` | passed |
| L1 | `py examples\existing-repo-migration\src\report.py` | passed |
| L2 | `cargo run -- examples\existing-repo-migration` | passed |

## Evidence Ledger

| Evidence ID | Type | Pointer | Result |
|---|---|---|---|
| EVID-001 | inspection | `docs/framework/vtrace-process.md` | passed |
| EVID-002 | inspection | `templates/adoption/` | passed |
| EVID-003 | inspection | `skills/vtrace-assess`, `skills/vtrace-adopt`, `skills/vtrace-gate` | passed |
| EVID-004 | inspection | `docs/framework/specification-baselines.md`, `templates/adoption/SPECIFICATION_BASELINE.md` | passed |
| EVID-005 | inspection | `docs/framework/implementation-management.md`, `templates/adoption/WORK_PACKAGES.md` | passed |
| EVID-006 | inspection | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | passed |
| EVID-007 | source custody | `docs/vtrace/SOURCE_BASIS.md`, `sources/source-registry.json` | passed |
| EVID-008 | command | `git diff --check` | passed |
| EVID-009 | command | `py -m json.tool sources\source-registry.json` | passed |
| EVID-010 | command | `py -m py_compile examples\hello-world\src\hello_world.py`; `py examples\hello-world\src\hello_world.py` | passed |
| EVID-013 | command | `cargo test` | passed |
| EVID-014 | command | `cargo run -- .` | passed |
| EVID-015 | inspection / command | `docs/vtrace/LANGUAGE_PROFILES.md`; `cargo run -- .` | passed |
| EVID-016 | command | `py -m py_compile examples\existing-repo-migration\src\report.py` | passed |
| EVID-017 | command | `py examples\existing-repo-migration\src\report.py` | passed |
| EVID-018 | command | `cargo run -- examples\existing-repo-migration` | passed |
| EVID-019 | test / schema inspection | evidence ledger validator tests; `schemas/evidence-ledger.schema.json` | passed |
| EVID-020 | test / inspection | gate checklist validator tests and `docs/vtrace/REVIEW_CHECKLISTS.md` | passed |
| EVID-021 | inspection | `docs/framework/nasa-technical-controls.md`; `docs/vtrace/SOURCE_BASIS.md` | passed |
| EVID-022 | command | `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; `cargo test` | passed |
| EVID-023 | command | `cargo run -- .` | passed |
| EVID-024 | command | `cargo run -- examples\existing-repo-migration` | passed |
