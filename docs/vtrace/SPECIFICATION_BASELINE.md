# Specification Baseline

## Scope

Repo: VTRACE

Baseline type: mixed

Baseline date: 2026-05-31

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-001 | product | current | `docs/framework/vtrace-process.md` defines the V shape, stage deliverables, stage rules, adoption flow, and definition of done. | inspection | self-adoption scenario | VTRACE | medium | accepted |
| SPEC-002 | REQ-002 | product | current | `templates/adoption/` contains stage templates including mission, requirements, specification baseline, architecture, implementation plan, work packages, verification, validation, trace, and review. | inspection | self-adoption scenario | VTRACE | medium | accepted |
| SPEC-003 | REQ-003 | product | current | `skills/vtrace-assess`, `skills/vtrace-adopt`, and `skills/vtrace-gate` guide assessment, adoption, and review gates. | inspection | agent scenario | VTRACE | medium | accepted |
| SPEC-004 | REQ-004 | product | current | Specification baselines classify current, target, deprecated, and unknown behavior before non-trivial implementation planning. | inspection | existing-repo scenario | VTRACE | medium | accepted |
| SPEC-005 | REQ-005 | product | current | Work packages require parent IDs, V closure rows, L0/L1/L2 validation levels, review lanes, Git execution, and pulse linkage. | inspection | work-package scenario | VTRACE | medium | accepted |
| SPEC-006 | REQ-006 | roadmap | target | Remaining gaps are recorded as `DCR-*` change requests and mapped to future work packages. | inspection | roadmap review | VTRACE | low | accepted |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-001 | covered | Framework process exists. |
| REQ-002 | SPEC-002 | covered | Templates exist. |
| REQ-003 | SPEC-003 | covered | Skills exist. |
| REQ-004 | SPEC-004 | covered | Spec baseline process exists. |
| REQ-005 | SPEC-005 | covered | Implementation management exists. |
| REQ-006 | SPEC-006 | covered | DCRs defined in change control. |

## Specification Gate

Decision: pass_with_risk

Rationale: VTRACE now has a self-baseline. Remaining risks are enforcement,
language-specific tailoring, and larger adoption evidence.
