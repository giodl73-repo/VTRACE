# Specification Baselines

Specification is the bridge between requirements and design. In VTRACE,
requirements say what must be true. Specifications define the controlled
behavior, product contracts, constraints, expected results, and acceptance basis
that implementation must satisfy.

NASA-style rigor brings specification in early: after mission, CONOPS, and
requirements are understood, and before implementation planning treats the
scope as buildable. For existing software repos, specification also comes in as
a retrofit step: discover current behavior, baseline what is relied upon, and
separate accepted targets from unknowns.

## Specification Types

Use one umbrella `SPECIFICATION_BASELINE.md` for small repos. Split it only
when the scope has enough risk or surface area to justify separate files.

| Spec Type | Use When | Typical IDs |
|---|---|---|
| Product/system spec | User-visible behavior or repo-level claims need control. | `SPEC-PROD-*` |
| Software spec | Internal software behavior, constraints, or error handling need control. | `SPEC-SW-*` |
| Interface spec | API, CLI, schema, config, file format, event, or integration contract changes. | `SPEC-IF-*`, `IF-*` |
| Package spec | Crate/package/module/language ownership needs explicit boundaries. | `SPEC-PKG-*`, `PKG-*` |
| Verification/test spec | Procedures, fixtures, expected outputs, thresholds, or acceptance tests need control. | `SPEC-TEST-*`, `VER-*` |
| Operations spec | Runbooks, deployment, operator workflow, or support behavior is part of the claim. | `SPEC-OPS-*` |

## Existing-Repo Retrofit

When applying VTRACE to an existing repo:

1. Inventory current docs, tests, examples, released behavior, CLI/API output,
   schemas, config, and downstream consumers.
2. Mark each behavior as `current`, `target`, `deprecated`, or `unknown`.
3. Assign a `SPEC-*` ID to every behavior or contract that implementation must
   preserve or change.
4. Map each accepted `REQ-*` to at least one `SPEC-*`, or explicitly defer it.
5. Map each `SPEC-*` to verification evidence or a planned verification item.
6. Put public or cross-repo contracts under change control before coding.

Do not rewrite history. If current behavior is messy but relied upon, baseline
it honestly and then create target `SPEC-*` rows for planned improvement.

## Entry Criteria Before Work Packages

Implementation planning can start when:

- the controlled scope is named,
- accepted `REQ-*` IDs are known,
- accepted `SPEC-*` IDs are known,
- public interfaces and package boundaries are identified,
- unknowns are either resolved or placed in discovery work packages,
- verification and validation methods are listed,
- change-control triggers are clear.

If this is not true, the next work package is a discovery/specification package,
not implementation.

## Change-Control Triggers

Update `CHANGE_CONTROL.md` when implementation changes:

- `SPEC-*` meaning,
- public behavior,
- CLI/API/schema/config/file-format contract,
- package/crate/module ownership,
- verification procedure or expected result,
- validation claim,
- accepted risk or waiver.

## Minimum Spec Review Questions

Ask these at the specification gate:

- Is each `REQ-*` testable and mapped to at least one `SPEC-*`?
- Does each `SPEC-*` state current vs target behavior?
- Are assumptions and non-goals explicit?
- Are public contracts identified and owned?
- Are expected results precise enough to test or inspect?
- Are unknowns blocked, deferred, or converted to discovery work?
- Are verification and validation methods credible for the claim?
- Are changes controlled before implementation starts?
