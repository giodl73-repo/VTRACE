# Specification Baseline

## Scope

Repo or feature: `examples/hello-world`

Baseline type: target

Baseline date: 2026-05-31

## Specification Sources

| Source | Evidence | Status | Notes |
|---|---|---|---|
| Requirements | `REQUIREMENTS.md` | target | Defines visible behavior and reviewability constraints. |
| Interfaces | `INTERFACES.md` | target | Defines stdout as the only public interface. |
| Code rigor | `CODE_RIGOR.md` | target | Defines reviewability constraints before implementation. |
| Verification | `VERIFICATION.md` | target | Defines expected command and output evidence. |

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-001 | product / interface | target | Running `src/hello_world.py` prints exactly `Hello, VTRACE!` followed by one newline to stdout. | demonstration | VAL-001 scenario | VTRACE | low | accepted |
| SPEC-002 | REQ-002 / REQ-CODE-001 | software | target | The program remains dependency-free, directly reviewable, and within local code-rigor constraints. | inspection / static check | VAL-001 scenario | VTRACE | low | accepted |

## Public Contracts

| Contract ID | Spec IDs | Surface | Compatibility Rule | Change-Control Trigger | Verification Evidence |
|---|---|---|---|---|---|
| IF-001 | SPEC-001 | stdout | Exact output text and trailing newline are controlled. | Any output, invocation, or dependency change. | VER-001 |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-001 | covered | Visible behavior. |
| REQ-002 | SPEC-002 | covered | Reviewability. |
| REQ-CODE-001 | SPEC-002 | covered | Code rigor. |

## Spec-To-Verification Coverage

| Spec ID | Verification IDs / Commands | Expected Result | Evidence Pointer | Status |
|---|---|---|---|---|
| SPEC-001 | VER-001 / `python src/hello_world.py` | Exact stdout `Hello, VTRACE!` plus newline. | EVID-001 | verified |
| SPEC-002 | VER-002, VER-003 / inspection and compile check | No dependencies; code-rigor constraints satisfied. | EVID-002, EVID-CR-001 | verified |

## Specification Gate

Decision: pass

Rationale: The tiny example has complete requirement-to-spec and spec-to-evidence
coverage for its controlled behavior.
