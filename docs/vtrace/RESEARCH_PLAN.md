# Research Plan

## Scope

Repo: VTRACE self-adoption

This self-trace copy proves the reusable research-plan template. VTRACE uses
research records to keep source-grounded framework changes, adoption lessons,
and large-domain discoveries from becoming uncontrolled implementation
authority.

## Research Rule

Research findings are advisory until they affect a controlled artifact through a
DCR, requirement row, spec row, scenario finding, fixture candidate, source-basis
entry, or work-package row.

## Research Tracks

| Track ID | Topic | Problem-space regions | Expected requirement pressure | Status |
|---|---|---|---|---|
| RCH-001 | Source-grounded process guidance | framework sources, source custody, review controls | Requirements and specs for new VTRACE controls. | active |
| RCH-002 | Adoption reuse patterns | templates, examples, CLI, agent workflows | Template additions and stage-order updates. | active |
| RCH-003 | Large-world control patterns | problem-space map, domain backlog, fixtures, diagnostics | Optional S1/S2 controls for complex target repos. | active |

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
6. Update controlled artifacts only after the DCR is accepted.

## Paper / Brief Backlog

| Research ID | Working title | Type | Track | Related artifacts | Requirement output | Status |
|---|---|---|---|---|---|---|
| RCH-PAPER-001 | Source custody for derived VTRACE guidance | source pass | RCH-001 | `SOURCE_BASIS.md`, `docs/framework/nasa-technical-controls.md` | Confirms pointer-first source posture and no compliance overclaim. | accepted |
| RCH-PAPER-002 | Communications strategy as controlled docs derivation | module note | RCH-002 | `COMMUNICATIONS_STRATEGY.md`, `docs/framework/communications-strategy.md` | Drove `REQ-COMMS-001` and `SPEC-018`. | accepted |
| RCH-PAPER-003 | Large-world adoption controls | paper / brief | RCH-003 | `PROBLEM_SPACE_MAP.md`, `DOMAIN_BACKLOG.md`, `FIXTURE_MODEL.md` | Drove `REQ-PROBLEM-SPACE-001`, `REQ-DOMAIN-BACKLOG-001`, and `REQ-FIXTURE-MODEL-001`. | accepted |
| RCH-PAPER-004 | Research plan as generic requirements input | module note | RCH-003 | `RESEARCH_PLAN.md`, `STAGE_EXECUTION.md` | Drives `REQ-RESEARCH-001` and `SPEC-026`. | accepted |

## Module Review

| Module | Research items | Review result | DCR impact | Status |
|---|---|---|---|---|
| Source-grounded framework controls | RCH-PAPER-001 | Passed as pointer-first derived guidance. | DCR-006 | accepted |
| Communications and docs derivation | RCH-PAPER-002 | Passed as user-docs control separate from specs. | DCR-011, DCR-012 | accepted |
| Large-world controls | RCH-PAPER-003, RCH-PAPER-004 | Passed as optional S1/S2 controls for complex repos. | DCR-015, DCR-016 | accepted |

## Decision

RESEARCH_PLAN is accepted as a reusable VTRACE adoption template. It belongs in
S1 for large-domain or source-heavy repos, after problem-space/domain discovery
and before implementation specs, fixtures, or work packages harden findings.
