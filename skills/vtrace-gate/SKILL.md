---
name: vtrace-gate
description: Run a VTRACE specification, design, or readiness review gate for a repo, feature, wave, release, or claim. Use when Codex needs to decide pass, pass_with_risk, blocked, or deferred based on requirements, design, verification, validation, traceability, and evidence.
---

# VTRACE Gate

Use this skill to run a scoped review gate.

## Inputs

- Scope: repo baseline, feature, wave, release, or claim.
- Gate type: specification, design, or readiness.
- Target repo validation commands.

## Workflow

1. Read VTRACE `docs/framework/review-process.md`.
2. Read target repo `docs/vtrace/` artifacts if present.
3. Inspect implementation surfaces and validation command results relevant to
   the scope.
4. Compare claims against evidence.
5. For readiness gates, check code-rigor constraints and evidence when
   `docs/vtrace/CODE_RIGOR.md` exists or critical implementation is in scope.
6. Record findings by severity.
7. Choose one decision:
   `pass`, `pass_with_risk`, `blocked`, or `deferred`.
8. Update or create `docs/vtrace/REVIEW.md`.

## Decision Rules

- Use `pass` only when requirements, verification, validation, and traceability
  are sufficient for the stated scope.
- Use `pass_with_risk` when the work is usable but evidence limits or accepted
  risks must be visible.
- Use `blocked` when a critical claim, requirement, design, or evidence gap
  prevents the stated scope.
- Use `deferred` when the scope is no longer ready or no longer current.

## Finding Rules

- Lead with critical and major findings.
- Tie every finding to a concrete artifact, command, or missing trace link.
- Do not block on process polish if objective evidence is sufficient.
- Do block on unsupported claims.
