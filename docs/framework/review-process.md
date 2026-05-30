# VTRACE Review Process

VTRACE uses three review gates. Each gate is scoped: review the current feature,
wave, release, or repo baseline, not the whole universe.

Use `assurance-security-review.md` to decide which role-review lanes are
required for the scope.
Use `roles-recommendation.md` to recommend a ROLES-conformant `.roles` panel for
target repos.

## Gate 1: Specification Review

Run before serious implementation or before accepting a major feature scope.

Checks:

- Mission or need is clear.
- CONOPS scenarios are realistic enough for the scope.
- Requirements are testable.
- Non-goals and constraints are explicit.
- Each requirement has a planned verification method.
- Validation scenarios are identified or explicitly deferred.
- Required role-review lanes are identified.

Output: `docs/vtrace/REVIEW.md` section or a dated review note.

Allowed decisions:

- `pass`: implementation can proceed.
- `pass_with_risk`: implementation can proceed with named risks.
- `blocked`: requirements or mission are too unclear.
- `deferred`: scope is not ready or no longer priority.

## Gate 2: Design Review

Run before implementation locks in hard-to-change architecture, interfaces, or
data contracts.

Checks:

- Architecture satisfies the accepted requirements.
- Interfaces are explicit and owned.
- Data flow and persistence are understood.
- Failure modes are named.
- Tradeoffs and rejected alternatives are recorded.
- Verification approach matches design risk.
- Validation path still matches the mission need.
- Security/privacy, safety/risk, assurance, and source-custody lanes are
  required or explicitly not required.

Allowed decisions are the same four gate outcomes.

## Gate 3: Readiness Review

Run before release, merge, public claim, adoption by another repo, or closing a
wave as complete.

Checks:

- Implementation surfaces link to requirements or design IDs.
- Verification evidence exists and commands passed.
- Validation evidence exists or is explicitly deferred with rationale.
- Trace matrix has no unowned critical gaps.
- Open findings are accepted, blocked, or deferred.
- Required role-review lanes are complete.
- Claims in README, docs, or release notes match the evidence.

Allowed decisions are the same four gate outcomes.

## Finding Severity

| Severity | Meaning | Required Action |
|---|---|---|
| Critical | Claim is unsupported, unsafe, misleading, or likely broken. | Block unless scope is changed. |
| Major | Requirement, design, interface, or evidence gap affects readiness. | Fix or accept risk explicitly. |
| Minor | Local clarity, documentation, or maintainability issue. | Track or fix opportunistically. |
| Note | Useful observation without readiness impact. | No required action. |

## Review Record

Each review should record:

- scope,
- gate type,
- decision,
- participants or review lenses,
- role review matrix,
- evidence inspected,
- findings,
- accepted risks,
- required follow-up,
- validation commands and results.

## Review Discipline

- Do not pass a gate by weakening the trace matrix after the fact.
- Do not mark validation complete with only unit tests.
- Do not mark verification complete with only user demos.
- Do not invent compliance status.
- Use `pass_with_risk` when the repo is useful but the evidence boundary is
  narrower than the desired claim.
