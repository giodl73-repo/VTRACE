---
name: vtrace-adopt
description: Apply the VTRACE V-model process to a target repo by creating or updating stage deliverables under docs/vtrace, including mission, CONOPS, requirements, architecture, interfaces, design, verification, validation, trace matrix, and review records.
---

# VTRACE Adopt

Use this skill after assessment, or when the user asks to apply VTRACE to a
repo.

## Workflow

1. Read VTRACE `docs/framework/vtrace-process.md`.
2. Read the target repo's existing docs and validation commands.
3. Choose the adoption scope: repo baseline, feature, wave, release, or claim.
4. Create `docs/vtrace/` in the target repo if it does not exist.
5. Copy or adapt only the needed templates from VTRACE `templates/adoption/`.
6. Fill the minimum first slice:
   `MISSION.md`, `REQUIREMENTS.md`, `TRACE.md`, `VERIFICATION.md`, `REVIEW.md`.
7. Add `CONOPS.md`, `ARCHITECTURE.md`, `INTERFACES.md`, `DESIGN.md`, and
   `VALIDATION.md` when the scope needs them.
8. Link requirements to implementation surfaces and verification commands.
9. Run the target repo's validation commands.
10. Update the review record with command results and open risks.

## Stage Order

Use this order unless the repo already has later-stage artifacts:

```text
MISSION -> CONOPS -> REQUIREMENTS -> ARCHITECTURE -> INTERFACES -> DESIGN
  -> IMPLEMENTATION -> VERIFICATION -> VALIDATION -> TRACE -> REVIEW
```

## Rules

- Keep artifacts concise enough to maintain.
- Do not duplicate existing architecture or product docs; link to them.
- Every requirement should have a verification method.
- Every validation claim should name a user need or scenario.
- If evidence is missing, record a gap and next action.
