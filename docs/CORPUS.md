# Documentation Corpus

## Purpose

This corpus records VTRACE documentation surfaces, owners, and update
obligations. It keeps user docs synchronized with controlled VTRACE artifacts
without turning user docs into specifications.

## Surface Inventory

| Surface | Owner | Source IDs | Update Trigger | Required Review | Status |
|---|---|---|---|---|---|
| `docs/README.md` | VTRACE | COMMS-README-001 / WP-012 | Any docs taxonomy or audience change. | adoption guide / configuration control | current |
| `docs/concepts/` | VTRACE | COMMS-CONCEPTS-001 / REQ-COMMS-001 | Process semantics, stage meaning, or docs/spec boundary changes. | adoption guide / traceability | current |
| `docs/how-to/` | VTRACE | COMMS-CLI-001 / REQ-CLI-001 / REQ-COMMS-001 | CLI workflow, adoption task, or review task changes. | adoption guide / V&V | current |
| `docs/tutorials/` | VTRACE | COMMS-RUNE-PATTERN-001 / VAL-011 | Adoption learning path or example sequence changes. | adoption guide / V&V | current |
| `docs/examples/` | VTRACE | SPEC-018 / VAL-011 | Template, artifact, or expected-output examples change. | V&V / traceability | current |
| `docs/traces/` | VTRACE | WP-011 / WP-012 / EVID-052 / EVID-053 | Work packages close or evidence claims change. | traceability / configuration control | current |
| `docs/decks/` | VTRACE | COMMS-RUNE-PATTERN-001 / VAL-011 | Stakeholder rollout or adoption narrative changes. | adoption guide / stakeholder review | current |
| `docs/framework/` | VTRACE | SPEC-001 / SPEC-018 | Framework guidance changes. | systems engineering / source custody as needed | current |
| `docs/vtrace/` | VTRACE | all accepted self-trace IDs | Any VTRACE requirement, spec, work package, evidence, or review change. | required VTRACE lanes | current |

## Update Rules

- User docs must not define new requirements or specs by themselves.
- User docs claims should name the controlled artifact or evidence that supports
  them when the claim affects adoption, readiness, or public behavior.
- Tutorials and examples should include expected outputs or success criteria.
- Trace pages should point to `docs/vtrace/TRACE.md`, `VERIFICATION.md`,
  `VALIDATION.md`, and `EVIDENCE.md` when making readiness claims.
- When multiple docs surfaces change in one work package, update this corpus in
  the same work package.
