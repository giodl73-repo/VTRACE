# Work Packages

## Scope

Repo: VTRACE

## Work Package Table

| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | L0 / L1 / L2 | Status |
|---|---|---|---|---|---|---|---|
| WP-001 | Add lightweight validators. | DCR-001 / REQ-VAL-001 / SPEC-007 / CR-001..CR-003 | `tools/vtrace_check.py`, `tests/test_vtrace_check.py`, docs | Artifact contracts stable. | Validator catches missing files, ID coverage gaps, undefined evidence, incomplete WP shape, and incomplete review lanes. | L0: `py -m unittest discover -s tests -p "test_*.py"` / L1: `py tools\vtrace_check.py .` / L2: run on example repos | complete |
| WP-002 | Add language/package profiles. | DCR-002 / REQ-PROFILE-001 / CR-004 | `docs/framework/`, `templates/adoption/` | At least one target repo need identified. | Profiles define applicability, commands, boundaries, and review lanes. | L0: inspection / L1: profile scenario / L2: target repo adoption | proposed |
| WP-003 | Add realistic existing-repo migration example. | DCR-003 / CON-001 / SPEC-004 / SPEC-005 | `examples/` | Candidate scenario selected. | Example shows current/target/unknown spec baseline and one closed WP. | L0: docs sanity / L1: scenario commands / L2: adoption review | proposed |
| WP-004 | Add reusable evidence ledger template/schema and automation. | DCR-004 / IF-003 / SPEC-005 | `templates/adoption/`, optional schema, validator | VTRACE self-ledger fields proven. | Target repos can adopt `EVIDENCE.md`; optional schema/validator checks commands, CI, reviews, artifacts, and findings. | L0: docs sanity / L1: schema if added / L2: used in example | proposed |
| WP-005 | Add gate-specific checklists. | DCR-005 / SPEC-001 / SPEC-005 | `docs/framework/`, `templates/adoption/` | Gate list stable. | Each major gate has checklist, entry criteria, exit criteria, and role lanes. | L0: inspection / L1: self-gate / L2: target repo gate | proposed |
| WP-006 | Deepen NASA specificity encoding. | DCR-006 / NEED-002 / SPEC-001 | `docs/framework/`, `sources/`, `schemas/` | Source rights posture confirmed. | Derived guidance covers technical reviews, traceability, assurance, and configuration management. | L0: source custody review / L1: framework inspection / L2: role review | proposed |

## Orphan Check

- [x] Every accepted `REQ-*` is assigned to current evidence, a DCR, or a work package.
- [x] Every accepted `SPEC-*` is assigned to current evidence or future work.
- [x] Every future implementation slice has a DCR parent.
- [x] Every proposed work package has L0/L1/L2 expectations.
- [x] Tooling/profile gaps are visible instead of hidden in prose.
