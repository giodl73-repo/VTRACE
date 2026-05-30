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
   package boundaries, code rigor, implementation plan, work packages,
   implementation, verification, validation, trace, review.
4. Record evidence, gaps, and risk by stage.
5. Check whether critical code needs explicit size, complexity, assertion,
   error-handling, warning, or static-analysis constraints before new code is
   written.
6. Check whether implementation needs Git branch/worktree rules and L0/L1/L2
   validation levels.
7. Check whether implementation crosses package/crate/module/language
   boundaries or needs explicit dependency-direction rules.
8. Check whether security/privacy, safety/risk, source-custody, software
   assurance, or configuration/change-control role lanes are required.
9. Check whether the repo has `.roles/ROLE.md` and whether its panel matches
   required VTRACE role lanes.
10. Recommend the smallest adoption slice.

## Output

Write or return an assessment with these sections:

- `Scope`
- `Existing Stage Artifacts`
- `Traceability Gaps`
- `Verification Evidence`
- `Validation Evidence`
- `Code Rigor Need`
- `Implementation Planning Need`
- `Package Boundary Need`
- `Readiness Risks`
- `Required Role Review Lanes`
- `.roles Panel Recommendation`
- `Recommended Adoption Slice`
- `Validation Commands`

## Rules

- Do not add heavyweight artifacts before identifying the actual gap.
- Do not claim compliance.
- Treat missing evidence as a finding, not a failure of the repo.
- Prefer repo-local validation commands over generic process checks.
