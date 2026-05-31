# Architecture

## Scope

Repo: VTRACE

## Components

| ID | Component | Responsibility | Related Specs |
|---|---|---|---|
| ARCH-001 | `docs/framework/` | Define reusable VTRACE process, review, execution, roles, source, and specificity guidance. | SPEC-001, SPEC-004, SPEC-005 |
| ARCH-002 | `templates/adoption/` | Provide target-repo artifact templates. | SPEC-002 |
| ARCH-003 | `skills/` | Provide agent-usable assessment, adoption, gate, and rigor workflows. | SPEC-003 |
| ARCH-004 | `examples/` | Demonstrate complete and scenario-based adoption. | SPEC-001..SPEC-005 |
| ARCH-005 | `sources/` and `schemas/` | Preserve source custody and encoding structure. | SPEC-001 |
| ARCH-006 | `context/waves/` | Record VTRACE repo execution history. | SPEC-006 |

## Risks

| Risk ID | Risk | Mitigation |
|---|---|---|
| RISK-001 | Framework drift if templates, skills, and process docs evolve separately. | Self-trace and future validators. |
| RISK-002 | Adoption remains subjective without automated checks. | `DCR-001` validator work package. |
| RISK-003 | Guidance may be too generic for language-specific repos. | `DCR-002` language/package profiles. |
