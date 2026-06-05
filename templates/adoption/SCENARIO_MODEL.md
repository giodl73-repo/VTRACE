# Scenario Model

## Scope

Reusable VTRACE template for scenario packages.

Scenarios validate that the right behavior is being specified and later built.
They are not only demos: they are fixture seeds, error-path probes, and evidence
anchors for requirements, specs, contracts, and work packages.

## Scenario Root

Repo scenario root:

```text
scenarios/<domain>/<package-or-feature>/
```

Each scenario package should contain:

```text
scenario.yaml
99-findings.md
00-workspace/
```

Use repo-local naming if another scenario convention already exists, but keep the
same controls visible.

## Required Scenario Shape

| Field | Required content |
|---|---|
| Scenario ID | Stable `SC-*` or repo-local scenario ID. |
| Actor | User, maintainer, platform engineer, CI owner, operator, or reviewer. |
| Purpose | What product question the scenario answers. |
| Specs exercised | Requirement/spec/contract IDs and spec file paths. |
| Boundary classes | Contract and package boundaries crossed. |
| Positive path | Expected successful workflow. |
| Negative paths | Error, stale, missing, invalid, or adversarial cases. |
| Diagnostics | Expected stable IDs or allocation queue. |
| Evidence | Receipts, reports, logs, inspect output, proof points, or fixture outputs. |
| Fixture candidates | Files or commands that should become reusable tests. |
| Findings | Issues found in specs and how they were resolved or deferred. |

## Findings Rule

Scenarios should usually find issues. A scenario with no findings needs an
explicit explanation of why the exercised spec surface was already sufficient.

Findings should classify:

- critical: blocks the spec from implementation input,
- major: must be fixed before L2 implementation,
- minor: can be fixed before public readiness,
- observation: useful but not blocking.

## Validation Coupling

A validation row should point to scenario evidence when the scenario proves a
user or operator workflow. A verification row should point to scenario evidence
when the scenario proves file completeness, fixture shape, diagnostic allocation,
or contract-boundary coverage.