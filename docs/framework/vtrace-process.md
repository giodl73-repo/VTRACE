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
         Architecture  Integration verification
          \             /
           Interfaces Interface verification
            \         /
             Design Component/unit verification
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
| Architecture | `docs/vtrace/ARCHITECTURE.md` | Define components, boundaries, data flows, and dependency posture. | Requirements map to system elements and known risks. |
| Interfaces | `docs/vtrace/INTERFACES.md` | Control APIs, CLIs, schemas, file formats, config, and external boundaries. | Interfaces have owners, compatibility rules, and verification methods. |
| Detailed Design | `docs/vtrace/DESIGN.md` or feature notes | Define algorithms, invariants, state transitions, edge cases, and tradeoffs. | Design decisions link to requirements and expected component evidence. |
| Implementation | Code, docs, fixtures, generated artifacts | Build the controlled design. | Meaningful implementation surfaces reference requirement or design IDs. |
| Verification | `docs/vtrace/VERIFICATION.md` | Prove the system was built correctly. | Tests, inspections, static checks, schema checks, simulations, or reports cover requirements. |
| Validation | `docs/vtrace/VALIDATION.md` | Prove the right thing was built for the intended use. | User workflows, acceptance scenarios, demos, or operator review cover mission success criteria. |
| Trace Matrix | `docs/vtrace/TRACE.md` | Connect the whole chain. | Rows link need, requirement, design, implementation, verification, validation, and evidence. |
| Review Gate | `docs/vtrace/REVIEW.md` | Make the readiness decision. | Decision is `pass`, `pass_with_risk`, `blocked`, or `deferred`, with findings recorded. |

## Minimum Adoption Slice

Do not start by demanding every file above. For an existing repo, the first
slice should produce:

1. `docs/vtrace/MISSION.md`
2. `docs/vtrace/REQUIREMENTS.md`
3. `docs/vtrace/TRACE.md`
4. `docs/vtrace/VERIFICATION.md`
5. `docs/vtrace/REVIEW.md`

Add CONOPS, architecture, interfaces, detailed design, and validation depth as
the repo's risk and maturity justify it.

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

### Verification

Verification proves "built correctly." Methods include:

- automated tests,
- static checks,
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

1. Run `vtrace-assess` to inspect current docs, tests, architecture, and gaps.
2. Run `vtrace-adopt` to create the minimum VTRACE deliverables under
   `docs/vtrace/`.
3. Implement or document the smallest missing evidence slice.
4. Run the repo's normal validation commands.
5. Run `vtrace-gate` to record the readiness decision.
6. Commit the repo-local VTRACE artifacts with the implementation or evidence
   they control.

## Definition of Done

A VTRACE adoption slice is done when:

- a scope is named,
- requirements are testable,
- verification commands are listed,
- validation scenarios are listed or explicitly deferred,
- trace rows connect intent to evidence,
- review findings are recorded,
- open risks are accepted, blocked, or deferred.
