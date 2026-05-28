---
name: vtrace-assess
description: Assess an existing repo against the VTRACE V-model process before adoption. Use when Codex needs to inspect a codebase for mission clarity, requirements, architecture, interfaces, verification evidence, validation evidence, traceability gaps, and readiness risks.
---

# VTRACE Assess

Assess the target repo before adding or changing process artifacts.

## Workflow

1. Read VTRACE `docs/framework/vtrace-process.md` and
   `docs/framework/review-process.md` when available.
2. Inspect the target repo's README, product plan, architecture docs, tests,
   CI, validation commands, issue/wave history, and release claims.
3. Identify existing artifacts for each VTRACE stage:
   mission, CONOPS, requirements, architecture, interfaces, design,
   code rigor, implementation, verification, validation, trace, review.
4. Record evidence, gaps, and risk by stage.
5. Check whether critical code needs explicit size, complexity, assertion,
   error-handling, warning, or static-analysis constraints before new code is
   written.
6. Recommend the smallest adoption slice.

## Output

Write or return an assessment with these sections:

- `Scope`
- `Existing Stage Artifacts`
- `Traceability Gaps`
- `Verification Evidence`
- `Validation Evidence`
- `Code Rigor Need`
- `Readiness Risks`
- `Recommended Adoption Slice`
- `Validation Commands`

## Rules

- Do not add heavyweight artifacts before identifying the actual gap.
- Do not claim compliance.
- Treat missing evidence as a finding, not a failure of the repo.
- Prefer repo-local validation commands over generic process checks.
