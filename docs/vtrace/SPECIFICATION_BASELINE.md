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
| SPEC-007 | REQ-VAL-001 | software | current | `tools/vtrace_check.py` validates required VTRACE artifacts, requirement/spec trace visibility, evidence pointers, work-package shape, and required review-lane closure. | automated test / local command | self-adoption scenario | VTRACE | medium | accepted |
| SPEC-008 | REQ-PROFILE-001 | product | current | `docs/framework/language-profiles.md`, `templates/adoption/LANGUAGE_PROFILES.md`, and `docs/vtrace/LANGUAGE_PROFILES.md` define profile applicability and L0/L1/L2 expectations. | inspection / validator | profile scenario | VTRACE | medium | accepted |
| SPEC-009 | REQ-EXAMPLE-001 | product | current | `examples/existing-repo-migration/` demonstrates current/target spec baselining and one closed work package. | example command / validator | existing-repo scenario | VTRACE | medium | accepted |
| SPEC-010 | REQ-EVIDENCE-001 | product | current | `templates/adoption/EVIDENCE.md` and validator evidence checks define reusable evidence ledger behavior. | validator tests | evidence scenario | VTRACE | medium | accepted |
| SPEC-011 | REQ-GATE-001 | product | current | `docs/framework/gate-checklists.md`, `templates/adoption/REVIEW_CHECKLISTS.md`, and `docs/vtrace/REVIEW_CHECKLISTS.md` define gate-specific checklist closure. | inspection / validator | gate scenario | VTRACE | medium | accepted |
| SPEC-012 | REQ-NASA-001 | product | current | `docs/framework/nasa-technical-controls.md` encodes NASA-inspired technical controls as locally authored derived guidance. | source-custody inspection | source review | VTRACE | low | accepted |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-001 | covered | Framework process exists. |
| REQ-002 | SPEC-002 | covered | Templates exist. |
| REQ-003 | SPEC-003 | covered | Skills exist. |
| REQ-004 | SPEC-004 | covered | Spec baseline process exists. |
| REQ-005 | SPEC-005 | covered | Implementation management exists. |
| REQ-006 | SPEC-006 | covered | DCRs defined in change control. |
| REQ-VAL-001 | SPEC-007 | covered | Lightweight validator exists. |
| REQ-PROFILE-001 | SPEC-008 | covered | Language/package profiles exist. |
| REQ-EXAMPLE-001 | SPEC-009 | covered | Realistic migration example exists. |
| REQ-EVIDENCE-001 | SPEC-010 | covered | Evidence template and validator checks exist. |
| REQ-GATE-001 | SPEC-011 | covered | Gate checklist guidance and template exist. |
| REQ-NASA-001 | SPEC-012 | covered | NASA-inspired technical control guidance exists. |

## Specification Gate

Decision: pass_with_risk

Rationale: VTRACE now has a self-baseline, local validator, profiles, evidence
ledger template, gate checklists, realistic migration example, and deeper
source-grounded controls.
