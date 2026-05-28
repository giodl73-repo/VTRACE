# Validation Plan

## Scope

Repo or feature: `examples/hello-world`

## Validation Scenarios

| Scenario ID | User / Actor | Need | Workflow | Success Criteria | Evidence Pointer | Result |
|---|---|---|---|---|---|---|
| VAL-001 | VTRACE adopter | See one full VTRACE chain on a runnable example. | Read docs, run program, inspect TRACE and REVIEW. | Program output and trace chain are understandable. | EVID-001, TRACE.md, REVIEW.md | pass |

## Acceptance Evidence

| Evidence ID | Scenario ID | Evidence | Result |
|---|---|---|---|
| EVID-001 | VAL-001 | `python src\hello_world.py` prints expected output. | pass |
| EVID-CR-001 | VAL-001 | Code-rigor constraints are visible and satisfied. | pass |

## Deferred Validation

| Scenario | Reason Deferred | Risk | Revisit Trigger |
|---|---|---|---|
| Apply VTRACE to a real repo | This example is intentionally tiny. | Process gaps may appear at larger scope. | First portfolio adoption. |
