# NASA V-Model Source Pass

Date: 2026-05-28

## Research Question

What public NASA systems and software engineering guidance should anchor the
first VTRACE framework and skills?

## Sources

### NASA Systems Engineering Handbook

URL: <https://www.nasa.gov/reference/systems-engineering-handbook/>

The handbook is the main public source for NASA systems engineering concepts,
including life-cycle thinking, technical processes, requirements, design,
product realization, technical management, and verification and validation.

Implication for VTRACE: use the handbook as the top-level source for the
software-adapted V-model framework, while keeping repo guidance lightweight and
explicitly non-official.

### NASA Systems Engineering Handbook Appendix

URL: <https://www.nasa.gov/reference/system-engineering-handbook-appendix/>

The appendix material includes outlines and matrices relevant to requirements,
verification, validation, and traceability. It explicitly treats V&V planning
and requirement verification matrices as structured planning artifacts.

Implication for VTRACE: first-class artifacts should include a requirement
verification matrix, validation plan outline, and bidirectional traceability
links between requirements, design, implementation, and evidence.

### NASA Software Engineering Handbook

URL: <https://swehb.nasa.gov/>

The Software Engineering Handbook maps software requirements to rationale,
guidance, lessons learned, assurance, and objective evidence.

Implication for VTRACE: codebase skills should ask for objective evidence, not
just stated intent. Tests, review records, generated reports, architecture
notes, and release gates should be attached to claims.

## Working Framework

VTRACE maps the V-model to codebase work as follows:

| Left side | Software adaptation | Right side |
|---|---|---|
| Mission need | User/stakeholder problem and operating context | Validation evidence |
| System requirements | Product and repo-level requirements | Acceptance evidence |
| Architecture | Modules, boundaries, interfaces, data contracts | Integration evidence |
| Detailed design | APIs, algorithms, schemas, invariants | Unit/component evidence |
| Implementation | Code, docs, fixtures, generated artifacts | Build/test/review evidence |

The central invariant is traceability. Each major artifact should be connected
upward to intent and downward to verification or validation evidence.

## Adopt Now

- Create a `v-model-rigor` skill for assessing a repo and establishing a
  traceability spine.
- Keep the first validation gate simple: `git diff --check`.
- Add later templates only after this foundation commit lands.

## Defer

- Automated trace validators.
- Schema-enforced requirements databases.
- Portfolio-wide adoption requirements.
- Claims of NASA compliance.
