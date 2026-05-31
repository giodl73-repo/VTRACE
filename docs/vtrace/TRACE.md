# Trace Matrix

| Requirement ID | Parent Need | Requirement | Specification Item | Design Element | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 / NEED-002 | Define source-grounded V-model process. | SPEC-001 | ARCH-001 / DES-004 | already_satisfied | `docs/framework/vtrace-process.md` | inspection | VAL-001 | EVID-001 | verified |
| REQ-002 | NEED-001 | Provide adoption templates. | SPEC-002 | ARCH-002 / DES-001 | already_satisfied | `templates/adoption/` | inspection | VAL-001 | EVID-002 | verified |
| REQ-003 | NEED-003 | Provide assessment, adoption, and gate skills. | SPEC-003 | ARCH-003 | already_satisfied | `skills/` | inspection | VAL-003 | EVID-003 | verified |
| REQ-004 | NEED-001 | Require spec baselines before implementation planning. | SPEC-004 | ARCH-001 / DES-002 | already_satisfied | `docs/framework/specification-baselines.md`, `templates/adoption/SPECIFICATION_BASELINE.md` | inspection | VAL-001 | EVID-004 | verified |
| REQ-005 | NEED-001 / NEED-003 | Map implementation to controlled work packages. | SPEC-005 | ARCH-001 / DES-003 | already_satisfied | `docs/framework/implementation-management.md`, `templates/adoption/WORK_PACKAGES.md` | inspection | VAL-002 | EVID-005 | verified |
| REQ-006 | NEED-001 / NEED-003 | Define future DCRs for missing capabilities. | SPEC-006 | DES-003 | WP-001..WP-006 | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | inspection | VAL-002 | EVID-006 | verified |
| REQ-VAL-001 | NEED-001 / NEED-003 | Provide lightweight local validation tooling. | SPEC-007 | ARCH-007 / IF-005 / DES-003 / CR-001..CR-003 | WP-001 | `tools/vtrace_check.py`, `tests/test_vtrace_check.py` | unittest and local validator command | VAL-002 / self-adoption validator run | EVID-013, EVID-014 | verified |
| REQ-PROFILE-001 | NEED-001 / NEED-003 | Define language/package validation profiles. | SPEC-008 | ARCH-001 / CR-004 | WP-002 | `docs/framework/language-profiles.md`, `templates/adoption/LANGUAGE_PROFILES.md`, `docs/vtrace/LANGUAGE_PROFILES.md` | validator and inspection | VAL-004 | EVID-015 | verified |
| REQ-EXAMPLE-001 | NEED-001 / CON-001 | Include a realistic existing-repo migration example. | SPEC-009 | ARCH-004 | WP-003 | `examples/existing-repo-migration/` | compile, smoke, validator | VAL-005 | EVID-016, EVID-017, EVID-018 | verified |
| REQ-EVIDENCE-001 | NEED-001 / NEED-003 | Provide reusable evidence ledger artifacts and validation checks. | SPEC-010 | ARCH-002 / IF-003 | WP-004 | `templates/adoption/EVIDENCE.md`, `schemas/evidence-ledger.schema.json`, `tools/vtrace_check.py` | validator tests | VAL-006 | EVID-019 | verified |
| REQ-GATE-001 | NEED-001 / CON-003 | Provide gate-specific review checklist artifacts. | SPEC-011 | ARCH-001 / ARCH-002 | WP-005 | `docs/framework/gate-checklists.md`, `templates/adoption/REVIEW_CHECKLISTS.md` | validator tests and self-checklist | VAL-007 | EVID-020 | verified |
| REQ-NASA-001 | NEED-002 | Encode deeper NASA-inspired technical controls as derived guidance. | SPEC-012 | DES-004 | WP-006 | `docs/framework/nasa-technical-controls.md` | source-custody inspection | VAL-008 | EVID-021 | verified |

## Open Trace Risks

| Risk | Disposition |
|---|---|
| Automated trace checking exists for core Markdown contracts but not yet as a reusable CI workflow. | Future validator hardening. |
