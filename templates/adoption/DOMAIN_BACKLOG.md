# Domain Backlog

## Scope

Reusable VTRACE template for a domain backlog.

The domain backlog captures a problem domain as concepts, unknowns, hypotheses,
reusable patterns, risks, fixture candidates, and work-package candidates. It is
where discoveries live before they become specs, scenarios, fixtures, packages,
or explicit rejections.

## Backlog Item Types

| Type | Meaning | Exit path |
|---|---|---|
| Concept | A domain idea the system may need to model. | Product spec or explicit rejection. |
| Pattern | A reusable behavior observed in current code, docs, examples, or reference repos. | Spec amendment, scenario, or fixture. |
| Unknown | A question that blocks confident design or implementation. | Discovery package, source-oracle check, or deferral. |
| Hypothesis | A proposed product/platform rule that needs scenario pressure. | Scenario review or spec finding. |
| Risk | A way the system can overclaim, fragment, leak, stale, or mislead. | Boundary, diagnostic, fixture, or review gate. |
| Candidate package | Work that likely deserves a work-package row. | Work-package ledger. |
| Candidate fixture | Scenario artifact that may become a reusable test. | Fixture model promotion. |
| Do-not-adopt | A reference idea intentionally not accepted. | Reuse ledger or spec note. |

## Backlog Fields

Each durable backlog item should name:

1. ID,
2. type,
3. title,
4. source or discovery path,
5. affected problem-space region,
6. affected specs,
7. affected contract boundaries,
8. diagnostic impact,
9. fixture/scenario impact,
10. proposed disposition,
11. owner or next work package,
12. status.

## Backlog Rule

If a discovery does not yet belong in a spec, scenario, fixture, package, or docs
surface, put it here with a disposition. Do not hide it in prose.

## Backlog Table

| ID | Type | Title | Source | Disposition | Status |
|---|---|---|---|---|---|
| DB-001 | concept / pattern / unknown / hypothesis / risk / candidate package / candidate fixture / do-not-adopt |  |  |  | proposed |