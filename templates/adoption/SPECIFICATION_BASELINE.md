# Specification Baseline

## Scope

Repo or feature:

Baseline type: current / target / mixed

Baseline date:

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| README / docs |  | current / target / unknown |  |
| Tests / fixtures |  | current / target / unknown |  |
| CLI/API behavior |  | current / target / unknown |  |
| Released package / downstream use |  | current / target / unknown |  |
| Requirement / issue / product plan |  | current / target / unknown |  |

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-001 | product / software / interface / package / test / ops | target |  | test / analysis / inspection / demo | scenario / operator review / n/a |  | low / medium / high | proposed |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 | CLI / API / schema / config / file / event |  |  |  |

## Package / Language Allocation

| Spec IDs | Package / Crate / Module / Language | Responsibility | Forbidden Responsibility | Validation Profile |
|---|---|---|---|---|
| SPEC-001 |  |  |  | L0 / L1 / L2 |

## Nonfunctional Constraints

| Constraint ID | Parent Spec IDs | Constraint | Threshold / Rule | Verification Method | Status |
|---|---|---|---|---|---|
| SPEC-NF-001 | SPEC-001 |  |  |  | proposed |

## Assumptions And Unknowns

| ID | Item | Impact | Disposition | Owner |
|---|---|---|---|---|
| SPEC-UNK-001 |  |  | discovery / defer / block / accept risk |  |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-001 | covered / partial / deferred / unknown |  |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001 | VER-001 / command |  |  | planned |

## Specification Gate

Decision: pass / pass_with_risk / blocked / deferred

Required before implementation planning:

- [ ] Every accepted `REQ-*` maps to one or more `SPEC-*` IDs or a recorded deferral.
- [ ] Every implementation work package can name parent `SPEC-*` IDs or discovery status.
- [ ] Public contracts have owners and change-control triggers.
- [ ] Unknowns are resolved, blocked, deferred, or converted to discovery work.
- [ ] Verification and validation methods are credible for the controlled claim.

Rationale:
