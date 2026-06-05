# Spec Model

## Scope

Reusable VTRACE template for defining the specification model of a repo, product,
feature, or platform package.

Use this file when a VTRACE adoption needs more than a flat specification
baseline. It defines what a controlled product spec must contain before a work
package can use it as implementation input.

## Authority Boundary

Record the authoritative homes for this adoption:

| Surface | Authority |
|---|---|
| Product specs |  |
| VTRACE planning and trace | `docs/vtrace` |
| Scenarios and fixtures |  |
| Reference inputs |  |
| Implementation evidence |  |

Reference inputs do not define target behavior until the behavior is restated in
an authoritative product spec.

## Spec Classes

| Class | Purpose | Required home | Required evidence |
|---|---|---|---|
| Product spec | Defines durable behavior, schema, operation, invariant, or diagnostic behavior. |  | role review, scenario reference, work-package row |
| Amendment spec | Adds behavior to an existing spec area without replacing the area. |  | before/after boundary note, compatibility note |
| Contract spec | Defines a durable public or internal contract. |  | descriptor or explicit no-descriptor rationale |
| Boundary spec | Defines ownership and dependency limits. | `PACKAGE_BOUNDARIES.md`, `CONTRACT_BOUNDARIES.md`, or local equivalent | boundary review |
| Adapter/profile spec | Defines how files, records, artifacts, or evidence are read, written, profiled, or addressed. |  | adapter fixtures and negative cases |
| Diagnostic allocation spec | Converts symbolic diagnostics into stable IDs and assertion fixtures. |  | negative fixture asserting the ID |
| Scenario spec | Describes a workflow, adversarial path, and expected proof points. | scenario root | scenario file, findings file, linked specs |
| Docs/corpus spec | Defines canonical docs and corpus movement after evidence exists. | docs handoff / communications strategy | docs review and corpus row |

## Required Spec Shape

Every product spec used for non-trivial implementation must define or explicitly
mark out of scope:

1. status and authority boundary,
2. owning spec area,
3. reference inputs and what was not adopted,
4. target-owned behavior statement,
5. users or platform actors,
6. data model, schemas, profiles, or descriptors,
7. operations and lifecycle states,
8. invariants,
9. positive examples,
10. negative examples and error paths,
11. diagnostics and allocation status,
12. path/address impact when addressable content is involved,
13. descriptor impact for durable contracts,
14. fixture and scenario impact,
15. proof/evidence impact,
16. compatibility and migration behavior,
17. implementation ownership or explicit deferral,
18. docs and corpus impact,
19. trace and work-package IDs.

If a spec cannot answer these fields, it remains draft planning input and cannot
authorize L2 implementation by itself.

## Scenario Coupling Rule

A spec is not deep enough for implementation planning until at least one scenario
or fixture path shows how the behavior succeeds or fails.

Each scenario package should identify:

1. specs exercised,
2. boundary classes crossed,
3. positive workflow expectations,
4. adversarial or stale-state cases,
5. expected diagnostics,
6. expected receipts or evidence,
7. inspect or report output expectations,
8. reusable fixture candidates,
9. open findings and spec changes made from those findings.

## Promotion Rule

A draft spec may move from planning evidence to implementation input only when:

1. it is in the correct spec home,
2. required spec-shape fields are present or explicitly out of scope,
3. contract-boundary impact is recorded,
4. descriptor impact is recorded or explicitly not required,
5. diagnostics are allocated or queued for allocation,
6. at least one scenario or fixture path is named,
7. trace, verification, validation, evidence, and package ledgers are updated,
8. role review closes critical and major findings.