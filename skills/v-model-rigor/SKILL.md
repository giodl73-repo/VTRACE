---
name: v-model-rigor
description: Assess or establish NASA-inspired V-model systems engineering rigor in a software codebase, including traceability, V&V planning, objective evidence, and review gates.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
  - Bash
---

# V-Model Rigor

Use this skill when a codebase needs stronger systems engineering discipline.
The goal is not ceremony. The goal is an auditable trace from need to
requirements, design, implementation, verification, validation, and evidence.

## Grounding

VTRACE is inspired by public NASA systems and software engineering guidance, but
it is not NASA-authored or NASA-endorsed. When making standards claims, cite the
repo's source notes or public source URLs.

## Workflow

1. Read the target repo's `README.md`, product plan, architecture docs, tests,
   validation commands, and active wave/pulse records if present.
2. Identify the repo's mission need and intended users.
3. Inventory existing requirements, explicit or implicit.
4. Map architecture elements, interfaces, APIs, schemas, and major modules to
   those requirements.
5. Identify verification evidence: tests, static checks, reviews, generated
   reports, fixtures, proofs, or reproducible commands.
6. Identify validation evidence: user scenarios, acceptance checks, demos,
   field data, stakeholder review, or product-level success criteria.
7. Record gaps as traceability findings, not vague quality concerns.
8. Propose the smallest complete adoption slice.

## Required Output

Produce a concise rigor report with these sections:

- `Mission Need`
- `Current Trace Spine`
- `Requirements`
- `Architecture and Interfaces`
- `Verification Evidence`
- `Validation Evidence`
- `Gaps`
- `Adoption Slice`
- `Validation Commands`

## Rules

- Do not invent compliance status.
- Do not require heavyweight templates before the repo has stable requirements.
- Keep domain-specific policy in the target repo.
- Prefer objective evidence over prose assertions.
- Every proposed requirement should have a planned verification method.
- Every validation claim should name the user need or operating context it
  answers.
