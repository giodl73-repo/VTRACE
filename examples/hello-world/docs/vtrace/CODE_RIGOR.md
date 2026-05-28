# Code Rigor

## Scope

Repo or feature: `examples/hello-world`

Risk level: low

Language/toolchain: Python

## Coding Constraints

| ID | Constraint | Applies To | Verification | Exception Rule |
|---|---|---|---|---|
| CR-001 | Functions should stay small enough for focused review; soft cap is 20 logical lines for this tiny example. | `src/hello_world.py` | inspection | Larger functions require rationale. |
| CR-002 | No recursion or complex control flow. | `src/hello_world.py` | inspection | Not expected for this scope. |
| CR-003 | No runtime dependencies beyond Python. | `src/hello_world.py` | inspection | Additional dependencies require architecture update. |
| CR-005 | Static syntax check should pass. | `src/hello_world.py` | `python -m py_compile src\hello_world.py` | Waivers are not allowed for this example. |

## Tailoring

| Area | Rule | Rationale |
|---|---|---|
| Function size | 20-line soft cap | The example is intentionally tiny. |
| Recursion | forbidden | No recursive behavior is needed. |

## Exceptions / Waivers

| ID | Constraint | Exception | Rationale | Owner | Revisit Trigger |
|---|---|---|---|---|---|
| none | n/a | n/a | n/a | n/a | n/a |

## Verification Evidence

| Evidence ID | Constraint IDs | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| EVID-CR-001 | CR-001, CR-002, CR-003 | inspection | pass | REVIEW.md |
| EVID-002 | CR-005 | `python -m py_compile src\hello_world.py` | pass | VERIFICATION.md |
