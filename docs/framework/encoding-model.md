# Encoding Model

VTRACE encodes standards as small, auditable concepts. A concept is a locally
authored adaptation of source guidance, not a copied source section.

## Unit of Encoding

Each encoded concept should answer five questions:

1. What source or sources informed it?
2. What systems engineering concept does it represent?
3. How does it apply to an ordinary software codebase?
4. Which trace links does it require?
5. What objective evidence should prove it?

The JSON shape is defined in `schemas/encoded-concept.schema.json`.

## Core Concept Set

| Concept | Source Basis | Software Adaptation |
|---|---|---|
| Requirement quality | NASA SE Handbook appendix requirement guidance | Requirements should be clear, feasible, verifiable, traceable, and owned. |
| Bidirectional traceability | NASA SE Handbook glossary and requirements management guidance | Repo decisions should link upward to need and downward to implementation and evidence. |
| Verification matrix | NASA SE Handbook appendix verification matrix guidance | Each requirement gets a verification method, command, artifact, owner, and result pointer. |
| Validation matrix | NASA SE Handbook appendix validation planning guidance | Product claims get user/scenario evidence, not just unit tests. |
| Interface control | NASA SE Handbook appendix interface document outline | APIs, schemas, CLIs, file formats, and external boundaries get explicit owners and change rules. |
| Objective evidence | NASA Software Engineering Handbook guidance | Claims are backed by tests, reviews, generated reports, fixtures, traces, or reproducible commands. |
| Review gate | NASA technical peer review and lifecycle review guidance | Major changes close only after named readiness criteria and findings are resolved or accepted. |

## Rights Boundary

NASA public sources may inform derived concepts. Industry standards should stay
metadata-only unless the operator has explicit access and redistribution rights.
Even then, VTRACE should encode its own adoption guidance rather than mirror the
standard.

## First Artifact Targets

VTRACE should next add these local templates:

- `templates/adoption/mission-need.md`
- `templates/adoption/requirements.md`
- `templates/adoption/trace-matrix.md`
- `templates/adoption/verification-plan.md`
- `templates/adoption/validation-plan.md`
- `templates/adoption/evidence-ledger.md`
- `templates/adoption/review-gate.md`
