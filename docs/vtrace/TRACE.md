# Trace Matrix

| Requirement ID | Parent Need | Requirement | Specification Item | Design Element | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 / NEED-002 | Define source-grounded V-model process. | SPEC-001 | ARCH-001 / DES-004 | already_satisfied | `docs/framework/vtrace-process.md` | inspection | VAL-001 | EVID-001 | verified |
| REQ-002 | NEED-001 | Provide adoption templates. | SPEC-002 | ARCH-002 / DES-001 | already_satisfied | `templates/adoption/` | inspection | VAL-001 | EVID-002 | verified |
| REQ-003 | NEED-003 | Provide assessment, adoption, and gate skills. | SPEC-003 | ARCH-003 | already_satisfied | `skills/` | inspection | VAL-003 | EVID-003 | verified |
| REQ-004 | NEED-001 | Require spec baselines before implementation planning. | SPEC-004 | ARCH-001 / DES-002 | already_satisfied | `docs/framework/specification-baselines.md`, `templates/adoption/SPECIFICATION_BASELINE.md` | inspection | VAL-001 | EVID-004 | verified |
| REQ-005 | NEED-001 / NEED-003 | Map implementation to controlled work packages. | SPEC-005 | ARCH-001 / DES-003 | already_satisfied | `docs/framework/implementation-management.md`, `templates/adoption/WORK_PACKAGES.md` | inspection | VAL-002 | EVID-005 | verified |
| REQ-006 | NEED-001 / NEED-003 | Define future DCRs for missing capabilities. | SPEC-006 | DES-003 | WP-001..WP-006 | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | inspection | VAL-002 | EVID-006 | verified |
| REQ-VAL-001 | NEED-001 / NEED-003 | Automated validation tooling is planned but not yet implemented. | SPEC-006 | DES-003 / CR-001..CR-003 | WP-001 | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | planned validator tests | VAL-002 / future target-repo adoption | DCR-001 | deferred |
| REQ-PROFILE-001 | NEED-001 / NEED-003 | Language/package profiles need evidence from real target repos. | SPEC-006 | DES-003 / CR-004 | WP-002 | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | profile inspection and scenario review | future target-repo adoption | DCR-002 | deferred |

## Open Trace Risks

| Risk | Disposition |
|---|---|
| Reusable target-repo evidence template/schema does not exist yet. | `DCR-004`. |
| Automated trace checking does not exist yet. | `DCR-001`. |
