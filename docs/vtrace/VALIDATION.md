# Validation

## Scope

Repo: VTRACE

## Validation Scenarios

| ID | User / Actor | Scenario | Success Criteria | Evidence | Status |
|---|---|---|---|---|---|
| VAL-001 | VTRACE maintainer | Apply VTRACE to VTRACE itself. | Repo has mission, requirements, spec baseline, architecture, interfaces, design, DCRs, work packages, trace, verification, validation, and review. | `docs/vtrace/` | passed |
| VAL-002 | Future adopter | Use VTRACE DCRs to choose next implementation pulse. | DCRs map to proposed work packages with L0/L1/L2 expectations. | `CHANGE_CONTROL.md`, `WORK_PACKAGES.md` | passed |
| VAL-003 | Future agent | Pick a VTRACE work package and know when to stop. | Work package has parent IDs, affected surfaces, entry/exit criteria, and validation levels. | `WORK_PACKAGES.md` | passed_with_risk |
| VAL-004 | VTRACE maintainer | Run the validator against VTRACE itself. | Validator returns success and reports actionable findings when contracts are broken in tests. | `tools/vtrace_check.py`, `tests/test_vtrace_check.py`, `EVID-013`, `EVID-014` | passed |
| VAL-005 | Existing repo maintainer | Apply VTRACE to a small existing behavior with a target change. | Current and target behavior are separated, implemented, verified, and reviewed. | `examples/existing-repo-migration/` | passed |
| VAL-006 | Evidence reviewer | Adopt an evidence ledger in a target repo. | Trace evidence IDs resolve to ledger rows with complete status. | `templates/adoption/EVIDENCE.md`, validator tests | passed |
| VAL-007 | Gate reviewer | Use gate-specific checklist rows for a review. | Required checklist rows cannot remain pending. | `REVIEW_CHECKLISTS.md`, validator tests | passed |
| VAL-008 | Source custody reviewer | Review deeper NASA-inspired control guidance. | Guidance is derived and pointer-first, without compliance claim. | `docs/framework/nasa-technical-controls.md` | passed |

## Open Validation Gaps

| Gap | Disposition |
|---|---|
| No realistic existing-repo migration example yet. | `DCR-003` / `WP-003`. |
| Validator is not packaged as CI yet. | Future validator hardening. |
