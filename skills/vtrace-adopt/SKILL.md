---
name: vtrace-adopt
description: Apply the VTRACE V-model process to a target repo by creating or updating stage deliverables under docs/vtrace, including mission, CONOPS, requirements, architecture, interfaces, design, implementation plan, work packages, verification, validation, trace matrix, and review records.
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
   Add `STAGE_EXECUTION.md` for staged completeness and
   `ROLE_RECOMMENDATIONS.md` when the target repo needs a `.roles` panel.
6. Fill the minimum first slice:
   `MISSION.md`, `REQUIREMENTS.md`, `SPECIFICATION_BASELINE.md`, `TRACE.md`,
   `VERIFICATION.md`, `REVIEW.md`.
7. Add `CONOPS.md`, `ARCHITECTURE.md`, `PACKAGE_BOUNDARIES.md`,
   `INTERFACES.md`, `DESIGN.md`, and `VALIDATION.md` when the scope needs them.
8. Use `SPECIFICATION_BASELINE.md` to classify existing behavior as
   `current`, `target`, `deprecated`, or `unknown`; map each accepted `REQ-*`
   to `SPEC-*` before non-trivial implementation planning.
9. Add `CODE_RIGOR.md` before implementation when code size, complexity,
   assertions, error handling, static analysis, or reviewability matter.
10. Add `IMPLEMENTATION_PLAN.md` and `WORK_PACKAGES.md` before non-trivial
   implementation.
   Map every accepted `REQ-*` and `SPEC-*` to a work package or explicit
   disposition before coding starts.
   Define branch/worktree strategy, commit/push conditions, and L0/L1/L2
   validation requirements.
   Map each work package to package/crate/module boundary IDs and language
   validation commands.
   If the target repo uses waves/pulses, map each work package to pulse files
   and use `PULSE_EXECUTION.md` inside those pulse records.
11. Add `CHANGE_CONTROL.md` if implementation changes controlled requirements,
    specifications, interfaces, design, validation claims, verification
    methods, or risks.
12. Add `INTEGRATION_PLAN.md` if multiple components, generated artifacts,
    schemas, or downstream consumers are involved.
13. Link requirements and specifications to work packages, implementation surfaces, verification
    commands, and evidence.
14. Run the target repo's validation commands.
15. Update the review record with command results and open risks.

## Stage Order

Use this order unless the repo already has later-stage artifacts:

```text
MISSION -> CONOPS -> REQUIREMENTS -> SPECIFICATION_BASELINE -> ARCHITECTURE
  -> INTERFACES -> DESIGN -> CODE_RIGOR -> IMPLEMENTATION_PLAN
  -> WORK_PACKAGES -> IMPLEMENTATION -> VERIFICATION -> VALIDATION
  -> TRACE -> REVIEW
```

## Rules

- Keep artifacts concise enough to maintain.
- Do not duplicate existing architecture or product docs; link to them.
- Every requirement should have a verification method.
- Every accepted requirement should map to a specification item or explicit deferral.
- Every non-trivial implementation slice should be a work package with entry and
  exit criteria.
- Every non-trivial implementation slice should name package/crate/module
  boundaries and allowed dependency direction.
- Every work package should state L0, L1, and L2 validation requirements, even
  when a level is not required.
- Every validation claim should name a user need or scenario.
- Recommend ROLES-conformant `.roles` files when role-review lanes are required.
- Preserve the repo's wave/pulse operating model; VTRACE work packages should
  be recorded in pulses, not hidden in untracked chat history.
- If evidence is missing, record a gap and next action.
