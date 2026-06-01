# Trace Matrix

| Requirement ID | Parent Need | Requirement | Specification Item | Design Element | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 / NEED-002 | Define source-grounded V-model process. | SPEC-001 | ARCH-001 / DES-004 | already_satisfied | `docs/framework/vtrace-process.md` | inspection | VAL-001 | EVID-001 | verified |
| REQ-002 | NEED-001 | Provide adoption templates. | SPEC-002 | ARCH-002 / DES-001 | already_satisfied | `templates/adoption/` | inspection | VAL-001 | EVID-002 | verified |
| REQ-003 | NEED-003 | Provide assessment, adoption, and gate skills. | SPEC-003 | ARCH-003 | already_satisfied | `skills/` | inspection | VAL-003 | EVID-003 | verified |
| REQ-004 | NEED-001 | Require spec baselines before implementation planning. | SPEC-004 | ARCH-001 / DES-002 | already_satisfied | `docs/framework/specification-baselines.md`, `templates/adoption/SPECIFICATION_BASELINE.md` | inspection | VAL-001 | EVID-004 | verified |
| REQ-005 | NEED-001 / NEED-003 | Map implementation to controlled work packages. | SPEC-005 | ARCH-001 / DES-003 | already_satisfied | `docs/framework/implementation-management.md`, `templates/adoption/WORK_PACKAGES.md` | inspection | VAL-002 | EVID-005 | verified |
| REQ-006 | NEED-001 / NEED-003 | Define future DCRs for missing capabilities. | SPEC-006 | DES-003 | WP-001..WP-006 | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | inspection | VAL-002 | EVID-006 | verified |
| REQ-VAL-001 | NEED-001 / NEED-003 | Provide lightweight local validation tooling. | SPEC-007 | ARCH-007 / IF-005 / DES-003 / CR-001..CR-003 | WP-001 / WP-007 | `src/`, `Cargo.toml`, `Cargo.lock` | cargo test and local validator command | VAL-002 / self-adoption validator run | EVID-013, EVID-014, EVID-022 | verified |
| REQ-PROFILE-001 | NEED-001 / NEED-003 | Define language/package validation profiles. | SPEC-008 | ARCH-001 / CR-004 | WP-002 | `docs/framework/language-profiles.md`, `templates/adoption/LANGUAGE_PROFILES.md`, `docs/vtrace/LANGUAGE_PROFILES.md` | validator and inspection | VAL-004 | EVID-015 | verified |
| REQ-EXAMPLE-001 | NEED-001 / CON-001 | Include a realistic existing-repo migration example. | SPEC-009 | ARCH-004 | WP-003 | `examples/existing-repo-migration/` | compile, smoke, validator | VAL-005 | EVID-016, EVID-017, EVID-018 | verified |
| REQ-EVIDENCE-001 | NEED-001 / NEED-003 | Provide reusable evidence ledger artifacts and validation checks. | SPEC-010 | ARCH-002 / IF-003 | WP-004 | `templates/adoption/EVIDENCE.md`, `schemas/evidence-ledger.schema.json`, `src/` | validator tests | VAL-006 | EVID-019 | verified |
| REQ-GATE-001 | NEED-001 / CON-003 | Provide gate-specific review checklist artifacts. | SPEC-011 | ARCH-001 / ARCH-002 | WP-005 | `docs/framework/gate-checklists.md`, `templates/adoption/REVIEW_CHECKLISTS.md` | validator tests and self-checklist | VAL-007 | EVID-020 | verified |
| REQ-NASA-001 | NEED-002 | Encode deeper NASA-inspired technical controls as derived guidance. | SPEC-012 | DES-004 | WP-006 | `docs/framework/nasa-technical-controls.md` | source-custody inspection | VAL-008 | EVID-021 | verified |
| REQ-RUST-001 | NEED-001 / NEED-003 | Implement local validator as Rust CLI. | SPEC-013 | ARCH-007 / IF-005 | WP-007 | `src/`, `Cargo.toml`, `Cargo.lock` | cargo fmt/test/run | VAL-002 / validator packaging scenario | EVID-022, EVID-023, EVID-024 | verified |
| REQ-CI-001 | NEED-001 / NEED-003 | Run Rust validation path in CI. | SPEC-014 | ARCH-008 / IF-006 | WP-008 | `.github/workflows/ci.yml` | workflow inspection, local command parity, and remote CI run | CI validation scenario | EVID-025, EVID-026, EVID-027 | verified |
| REQ-CLI-001 | NEED-001 / NEED-003 / CON-004 | Define and implement the first CLI orchestrator slice for VTRACE procedural execution. | SPEC-015 | ARCH-009 / DES-005 / IF-007 | WP-009 | `docs/framework/cli-orchestrator.md`, `src/` commands, `tests/cli.rs` | design inspection, CLI commands, and integration tests | VAL-009 | EVID-028 / EVID-029 / EVID-030 | verified |
| REQ-AI-001 | NEED-003 / CON-002 / CON-004 | Define optional LLM provider and agent/worktree boundaries. | SPEC-016 | ARCH-009 / DES-006 / IF-008 | WP-009 | `docs/framework/cli-orchestrator.md`, provider boundaries, agent brief command | design inspection and role review | VAL-009 | EVID-028 / EVID-029 | verified |

## Open Trace Risks

| Risk | Disposition |
|---|---|
| Remote CI run evidence depends on GitHub Actions retention. | Durable evidence row records run URL, run ID, commit SHA, and conclusion. |
| LLM provider adapters are designed but not implemented. | Provider support remains optional and advisory until a later DCR adds adapter code. |
