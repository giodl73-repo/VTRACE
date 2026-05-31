# VTRACE Process

VTRACE is a V-model process for turning repo intent into controlled
deliverables. The purpose is not ceremony. The purpose is traceable engineering:
each claim should connect to a need, requirement, design decision,
implementation surface, verification result, validation result, and evidence.

## The V Shape

```text
Mission / Need                         Validation
    \                                 /
     CONOPS                    Acceptance scenarios
      \                       /
       Requirements      System verification
        \                 /
       Specifications Product verification
         \               /
         Architecture  Integration verification
          \             /
           Interfaces Interface verification
            \         /
             Design + Code Rigor Component/unit/static verification
              \     /
            Implementation
```

The left side defines the system. The bottom builds it. The right side proves
that the left-side decisions were satisfied.

The practical rule is simple: every left-side stage must name the right-side
evidence that will prove it.

## Stage Deliverables

| Stage | Deliverable | Purpose | Exit Criteria |
|---|---|---|---|
| Mission / Need | `docs/vtrace/MISSION.md` | Define why the repo exists and what success means. | Users, operating context, constraints, non-goals, and success criteria are explicit. |
| CONOPS | `docs/vtrace/CONOPS.md` | Define how the system is used in real workflows. | Actors, scenarios, inputs, outputs, failure paths, and handoffs are recorded. |
| Requirements | `docs/vtrace/REQUIREMENTS.md` | Turn mission and CONOPS into testable statements. | Each requirement has an ID, rationale, priority, owner, and verification method. |
| Specification Baseline | `docs/vtrace/SPECIFICATION_BASELINE.md` | Define controlled behavior, product contracts, constraints, and acceptance basis for an existing repo or feature. | Current/as-planned behavior, public contracts, nonfunctional limits, assumptions, and derived specs are baselined or explicitly deferred. |
| Architecture | `docs/vtrace/ARCHITECTURE.md` | Define components, boundaries, data flows, and dependency posture. | Requirements map to system elements and known risks. |
| Package Boundaries | `docs/vtrace/PACKAGE_BOUNDARIES.md` | Define crate/package/module/language ownership, dependency direction, and validation profiles. | Work packages know which boundaries they may touch and which commands apply. |
| Interfaces | `docs/vtrace/INTERFACES.md` | Control APIs, CLIs, schemas, file formats, config, and external boundaries. | Interfaces have owners, compatibility rules, and verification methods. |
| Detailed Design | `docs/vtrace/DESIGN.md` or feature notes | Define algorithms, invariants, state transitions, edge cases, and tradeoffs. | Design decisions link to requirements and expected component evidence. |
| Code Rigor | `docs/vtrace/CODE_RIGOR.md` | Define coding constraints that make implementation reviewable, testable, statically checkable, and maintainable. | Size, complexity, assertion, error-handling, warning, and static-analysis expectations are set before code is written. |
| Implementation Planning | `docs/vtrace/IMPLEMENTATION_PLAN.md`, `docs/vtrace/WORK_PACKAGES.md` | Convert the accepted baseline into controlled work packages. | Work packages have parent IDs, entry criteria, exit criteria, verification commands, and review gates. |
| Implementation | Code, docs, fixtures, generated artifacts | Build the controlled design through approved work packages. | Meaningful implementation surfaces reference requirement, design, interface, code-rigor, and work-package IDs. |
| Verification | `docs/vtrace/VERIFICATION.md` | Prove the system was built correctly. | Tests, inspections, static checks, schema checks, simulations, or reports cover requirements. |
| Validation | `docs/vtrace/VALIDATION.md` | Prove the right thing was built for the intended use. | User workflows, acceptance scenarios, demos, or operator review cover mission success criteria. |
| Trace Matrix | `docs/vtrace/TRACE.md` | Connect the whole chain. | Rows link need, requirement, design, implementation, verification, validation, and evidence. |
| Review Gate | `docs/vtrace/REVIEW.md` | Make the readiness decision. | Decision is `pass`, `pass_with_risk`, `blocked`, or `deferred`, with findings recorded. |

## Minimum Adoption Slice

Do not start by demanding every file above. For an existing repo, the first
slice should produce:

1. `docs/vtrace/MISSION.md`
2. `docs/vtrace/REQUIREMENTS.md`
3. `docs/vtrace/SPECIFICATION_BASELINE.md`
4. `docs/vtrace/TRACE.md`
5. `docs/vtrace/VERIFICATION.md`
6. `docs/vtrace/REVIEW.md`

Add CONOPS, architecture, package boundaries, interfaces, detailed design, code rigor,
implementation planning, and validation depth as the repo's risk and maturity
justify it.

## Stage Rules

### Mission / Need

Write the mission as operating intent, not a marketing pitch. Name:

- primary users,
- decision or workflow the repo supports,
- operating context,
- constraints,
- non-goals,
- success criteria,
- first validation scenarios.

### CONOPS

Use concrete scenarios. For each scenario, name:

- actor,
- trigger,
- inputs,
- normal path,
- failure or degraded path,
- outputs,
- handoff to another system or person.

### Requirements

Every requirement should be:

- stable enough to reference,
- clear enough to test or inspect,
- linked to a mission need or CONOPS scenario,
- assigned a verification method,
- owned by the repo, not a vague external dependency.

Avoid requirements that merely say "support", "handle", or "be robust" unless
the measurable condition is explicit.

### Specification Baseline

Specification comes into play after mission, CONOPS, and requirements are
understood, and before architecture, detailed design, or implementation
planning is treated as ready.

NASA-style rigor does not rely on a single magic "spec" file. It controls
technical products: system requirements, interface requirements, software
requirements, expected results, verification procedures, validation plans,
technical data, and configuration baselines. VTRACE translates that into
`SPECIFICATION_BASELINE.md` plus narrower derived specs when risk requires
them.

For existing repos, start with an observed-current specification. Do not invent
an aspirational spec and pretend the repo already satisfies it. Separate:

- `current`: behavior observed in docs, tests, releases, examples, or
  downstream consumers,
- `target`: behavior accepted for this scope,
- `deprecated`: behavior intentionally kept or removed through change control,
- `unknown`: behavior requiring discovery before implementation.

Implementation planning should not start for non-trivial work until every
accepted `REQ-*` maps to a `SPEC-*` item or an explicit deferral.

### Architecture

Architecture explains system shape. Include:

- components and responsibilities,
- data flow,
- persistence and generated artifacts,
- external dependencies,
- boundaries and non-boundaries,
- failure modes,
- risks and mitigations.

### Interfaces

Interfaces include more than HTTP APIs. Track:

- CLIs and flags,
- JSON/YAML/TOML schemas,
- file formats,
- events,
- environment variables,
- config files,
- public functions or crates,
- external services.

### Detailed Design

Use detailed design when implementation choices affect correctness,
compatibility, safety, performance, or maintainability. Include:

- algorithm choice,
- invariants,
- state transitions,
- edge cases,
- rejected alternatives,
- rollout or migration plan.

### Code Rigor

Code rigor is the left-side pre-code agreement for implementation quality. It
belongs before or during detailed design, not after a large patch exists.

Use code rigor when the repo has critical logic, generated code, public APIs,
cross-repo consumers, safety/security consequences, difficult reviews, or a
history of regressions.

Define constraints for:

- maximum function or method size,
- cyclomatic or branch complexity,
- module/file size,
- assertion or invariant expectations,
- error and return-value handling,
- static analysis and warning policy,
- testability of critical paths,
- exceptions requiring written rationale.

The common VTRACE default is:

- small functions by default, with a soft cap around 60 logical lines unless a
  project sets a stricter language-specific rule,
- complex control flow requires design rationale,
- public interfaces require input, error, and compatibility evidence,
- critical logic requires assertions, invariants, property tests, or inspection,
- warnings and static analysis findings are clean or explicitly waived.

The right side of this stage is code-rigor verification: lint, static analysis,
review findings, complexity checks, unit/component tests, and evidence ledger
entries.

### Implementation Planning

Implementation should start only after the relevant baseline is understood.
For non-trivial work, create `IMPLEMENTATION_PLAN.md` and `WORK_PACKAGES.md`.

Each work package should name:

- objective,
- parent requirement/design/interface/code-rigor IDs,
- parent specification IDs,
- affected files or modules,
- entry criteria,
- exit criteria,
- verification commands,
- validation impact,
- risk,
- review gate.

Use `CHANGE_CONTROL.md` when implementation changes requirements,
specifications, public interfaces, architecture boundaries, validation claims,
verification methods, accepted risks, or code-rigor constraints.

Use `INTEGRATION_PLAN.md` when multiple components, generated artifacts,
schemas, public interfaces, or downstream consumers are involved.

### Verification

Verification proves "built correctly." Methods include:

- automated tests,
- static checks,
- coding-standard and complexity checks,
- schema validation,
- fixture comparison,
- inspections,
- generated reports,
- reproducible commands.

### Validation

Validation proves "built the right thing." Methods include:

- user workflows,
- acceptance scenarios,
- operator review,
- demo scripts,
- realistic examples,
- benchmark interpretation tied to user need.

### Trace Matrix

The trace matrix is the control surface. It should expose gaps without shame:
missing evidence is a finding, not a reason to fake proof.

### Review Gate

The review gate decides whether the repo can claim readiness for a scope. A
gate can pass with accepted risks, but accepted risks must be explicit.

## Applying VTRACE To A Repo

1. Run `vtrace-assess` to inspect current docs, tests, specifications,
   architecture, and gaps.
2. Run `vtrace-adopt` to create the minimum VTRACE deliverables under
   `docs/vtrace/`.
3. Establish the implementation baseline and write work packages.
4. Implement one work package at a time.
5. Run the work package verification commands.
6. Update trace rows and evidence pointers.
7. Run `vtrace-gate` to record the scoped readiness decision.
8. Commit the repo-local VTRACE artifacts with the implementation or evidence
   they control.

## Definition of Done

A VTRACE adoption slice is done when:

- a scope is named,
- requirements are testable,
- specifications baseline current and target behavior,
- verification commands are listed,
- implementation work packages have entry and exit criteria,
- validation scenarios are listed or explicitly deferred,
- trace rows connect intent to evidence,
- review findings are recorded,
- open risks are accepted, blocked, or deferred.
