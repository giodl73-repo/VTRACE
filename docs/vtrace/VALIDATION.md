# Validation

## Scope

Repo: VTRACE

## Validation Scenarios

| ID | User / Actor | Scenario | Success Criteria | Evidence | Status |
|---|---|---|---|---|---|
| VAL-001 | VTRACE maintainer | Apply VTRACE to VTRACE itself. | Repo has mission, requirements, spec baseline, architecture, interfaces, design, DCRs, work packages, trace, verification, validation, and review. | `docs/vtrace/` | passed |
| VAL-002 | Future adopter | Use VTRACE DCRs to choose next implementation pulse. | DCRs map to proposed work packages with L0/L1/L2 expectations. | `CHANGE_CONTROL.md`, `WORK_PACKAGES.md` | passed |
| VAL-003 | Future agent | Pick a VTRACE work package and know when to stop. | Work package has parent IDs, affected surfaces, entry/exit criteria, and validation levels. | `WORK_PACKAGES.md` | passed_with_risk |

## Open Validation Gaps

| Gap | Disposition |
|---|---|
| No realistic existing-repo migration example yet. | `DCR-003` / `WP-003`. |
| No executable validator yet. | `DCR-001` / `WP-001`. |
