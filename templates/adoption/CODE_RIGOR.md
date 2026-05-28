# Code Rigor

## Scope

Repo or feature:

Risk level: low / medium / high / safety-critical

Language/toolchain:

## Coding Constraints

| ID | Constraint | Applies To | Verification | Exception Rule |
|---|---|---|---|---|
| CR-001 | Functions should stay small enough for focused review; default soft cap is 60 logical lines unless tailored. | Critical hand-authored code | size/complexity check and code review | Larger units require rationale and evidence. |
| CR-002 | Complex control flow should be bounded, tested, or justified. | Critical logic | design inspection and tests | Record why complexity is necessary. |
| CR-003 | Public interfaces should handle invalid inputs and errors explicitly. | APIs, CLIs, schemas, file formats | interface tests and review | Waive only for impossible states with rationale. |
| CR-004 | Critical invariants should have assertions, checks, property tests, or inspection evidence. | Algorithms, state transitions, data contracts | tests and review | Explain if invariant is enforced elsewhere. |
| CR-005 | Static analysis, lint, compiler warnings, and formatter checks should be clean or waived. | Whole scope | tool output | Waivers need owner and revisit trigger. |

## Tailoring

List project-specific thresholds or tool choices.

| Area | Rule | Rationale |
|---|---|---|
|  |  |  |

## Exceptions / Waivers

| ID | Constraint | Exception | Rationale | Owner | Revisit Trigger |
|---|---|---|---|---|---|
|  |  |  |  |  |  |

## Verification Evidence

| Evidence ID | Constraint IDs | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| EVID-CR-001 | CR-001 |  | pending |  |
