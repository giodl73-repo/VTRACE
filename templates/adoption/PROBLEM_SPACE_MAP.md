# Problem Space Map

## Scope

Reusable VTRACE template for a large problem-space or world map.

Use this file when the domain is too large for a single linear checklist. The
map names regions, boundaries, contracts, scenarios, evidence gaps, and traversal
order so agents and reviewers can work through the space systematically.

## Map Rule

Every substantial discovery should land in a region or explicitly create a new
region. Work packages should name the region they are working in and adjacent
regions they affect.

## World Regions

| Region | Responsibility | Primary specs | Boundary pressure |
|---|---|---|---|
| RGN-001 |  |  |  |
| RGN-002 |  |  |  |
| RGN-003 |  |  |  |

## Traversal Order

Default traversal:

1. mission and operating identity,
2. actors and workflows,
3. source/config/model inputs,
4. core state or graph,
5. contracts and providers,
6. diagnostics and evidence,
7. fixtures and scenarios,
8. implementation packages,
9. docs and public claims.

When a scenario crosses regions, the package must name the crossing and check
the relevant boundary files.

## Cross-Region Risks

| Risk | Regions | Required control |
|---|---|---|
| System fragments into local tools instead of an integrated platform. | all | contract boundaries, package boundaries |
| Evidence is implied rather than produced. | evidence-adjacent regions | verification, validation, fixtures |
| Docs overclaim readiness. | docs and implementation regions | communications strategy, evidence pointers |
| Failure cause is hard to locate. | diagnostics and source/model regions | diagnostic model and inspect/report fixtures |
| Reference sources become accidental authority. | reuse regions | domain backlog and spec model |
| Scenarios never become reusable proof. | scenarios and fixtures | fixture model promotion |