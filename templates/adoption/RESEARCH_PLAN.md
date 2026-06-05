# Research Plan

## Scope

Reusable VTRACE template for research modules, tracks, papers, source passes, or
investigation briefs that drive requirements before implementation.

Use this file when a repo needs disciplined research between a large world map,
domain backlog, and implementation specs. Research plans are not specs,
fixtures, or evidence by themselves. They produce reviewed findings that may
become requirements, spec amendments, scenario packages, fixture candidates,
work packages, docs boundaries, or explicit rejections.

## Research Rule

Every research output must name the requirement, spec, scenario, fixture,
domain-backlog item, problem-space region, or DCR it affects. If no controlled
artifact is affected, the research remains advisory.

## Research Tracks

| Track ID | Topic | Problem-space regions | Expected requirement pressure | Status |
|---|---|---|---|---|
| RCH-001 |  |  |  | proposed |
| RCH-002 |  |  |  | proposed |
| RCH-003 |  |  |  | proposed |

## Research Item Types

| Type | Meaning | Exit path |
|---|---|---|
| Source pass | Reviews source families, custody, rights posture, and citation limits. | Source basis, DCR, or rejection. |
| Module note | Summarizes a domain area or platform subsystem. | Domain backlog, spec model, scenario model, or work package. |
| Paper / brief | Makes a bounded argument with evidence, theory, risks, and open questions. | Requirement/spec update, scenario finding, fixture candidate, or explicit rejection. |
| Comparative review | Compares possible approaches, libraries, standards, datasets, or architecture choices. | Architecture, contract boundary, package boundary, or do-not-adopt entry. |
| Negative finding | Shows that a source, feature, relationship, fixture, or claim should not be adopted. | Domain backlog, source basis, review checklist, or DCR. |

## Required Research Shape

Every durable research item should define:

1. research ID,
2. title,
3. type,
4. parent problem-space region or domain-backlog item,
5. question being answered,
6. source families or inspection targets,
7. custody or citation posture,
8. affected requirements and specs,
9. affected scenarios or fixtures,
10. affected contracts or package boundaries,
11. expected diagnostics or evidence impact,
12. review roles,
13. decision options,
14. accepted findings,
15. DCR or disposition.

## Research-To-Requirement Flow

Research can change controlled artifacts only through review:

1. Capture discovery in `DOMAIN_BACKLOG.md` if it is not yet spec-ready.
2. Link the item to a problem-space region in `PROBLEM_SPACE_MAP.md`.
3. Write the research plan or paper with source/custody posture.
4. Run role review and record findings.
5. Open or update a DCR for accepted requirement/spec changes.
6. Update `REQUIREMENTS.md`, `SPECIFICATION_BASELINE.md`, `SPEC_MODEL.md`,
   `SCENARIO_MODEL.md`, `FIXTURE_MODEL.md`, `WORK_PACKAGES.md`, or
   `COMMUNICATIONS_STRATEGY.md` only after the DCR is accepted.

## Paper / Brief Backlog

| Research ID | Working title | Type | Track | Related artifacts | Requirement output | Status |
|---|---|---|---|---|---|---|
| RCH-PAPER-001 |  | source pass / module note / paper / comparative review / negative finding | RCH-001 |  |  | proposed |

## Module Review

| Module | Research items | Review result | DCR impact | Status |
|---|---|---|---|---|
|  |  |  |  | proposed |

## Anti-Patterns

| Anti-pattern | Required control |
|---|---|
| Treating research prose as implementation authority. | DCR plus accepted requirement/spec row. |
| Promoting source text without custody review. | Source pass and source-basis update. |
| Letting world-map discoveries vanish into notes. | Domain backlog item with disposition. |
| Writing papers that do not affect controlled artifacts. | Advisory status until an artifact impact is named. |
| Using research to publish unvalidated product claims. | Communications strategy and evidence review. |
