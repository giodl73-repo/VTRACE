# Implementation Management

VTRACE implementation work is not random slices. Implementation proceeds from a
controlled baseline through planned work packages, integration, verification,
and review gates.

This is adapted from NASA project life-cycle discipline: projects move through
formulation and implementation phases, use life-cycle reviews and decision
points, apply common technical processes recursively to product layers, and
control requirements, interfaces, risks, configurations, and technical data.

## VTRACE Implementation Flow

```text
Baseline scope
  -> implementation plan
  -> work packages
  -> branch/change control
  -> component implementation
  -> local verification
  -> integration
  -> integration verification
  -> validation rehearsal
  -> readiness review
  -> transition/release
```

## Required Implementation Artifacts

For any non-trivial repo adoption, add:

- `docs/vtrace/IMPLEMENTATION_PLAN.md`
- `docs/vtrace/WORK_PACKAGES.md`

Add these when risk or cross-repo impact requires them:

- `docs/vtrace/CHANGE_CONTROL.md`
- `docs/vtrace/INTEGRATION_PLAN.md`

Use `docs/framework/execution-control.md` for Git branch/worktree discipline,
commit/push expectations, validation levels, and agent handoff rules.
Use `docs/framework/assurance-security-review.md` for role-review lanes,
security/privacy, safety/risk, assurance, source-custody, and change-control
classification.

## Baseline First

Before implementation starts, establish the baseline:

- accepted mission/CONOPS scope,
- accepted requirement IDs,
- accepted architecture/interface/design IDs,
- accepted package/crate/module boundary IDs,
- code-rigor constraints,
- verification plan,
- validation scenarios,
- open risks and accepted deferrals.

The baseline does not mean everything is perfect. It means the team knows what
is controlled and what would count as a change.

## Work Package Rule

Every implementation slice should be a work package, not a random task. A work
package has:

- ID,
- objective,
- parent requirement IDs,
- package/crate/module boundary IDs,
- design/interface IDs,
- files or modules likely affected,
- entry criteria,
- exit criteria,
- verification commands,
- L0/L1/L2 validation levels,
- validation impact,
- risks,
- assurance/security review lanes,
- review gate.

## Work Package V Closure

Each work package closes its own small V. Do not wait until the end of the
project to ask whether the V is complete.

```text
left-side controls -> implementation package -> right-side evidence
```

For each `WP-*`, check:

| V Area | Closure Question | Typical Evidence |
|---|---|---|
| Need / CONOPS | Which need or scenario does this package support? | `NEED-*`, `CON-*`, validation scenario link. |
| Requirements | Which accepted requirements are in scope? | `REQ-*` rows assigned in `IMPLEMENTATION_PLAN.md`. |
| Architecture / Interface | Which component or contract changes? | `ARCH-*`, `IF-*`, schema/API/CLI fixture. |
| Design / Code Rigor | Which implementation decisions and constraints apply? | `DES-*`, `CR-*`, design note, code-rigor waiver if any. |
| Implementation | What files, modules, docs, or generated artifacts changed? | Paths, commits, generated artifact IDs. |
| Verification | How did we prove the package was built correctly? | L0/L1 commands, tests, static analysis, review. |
| Validation | How did we prove or preserve intended use? | `VAL-*`, scenario run, operator review, accepted deferral. |
| Trace | Did trace rows link the package to evidence? | `TRACE.md` rows with `WP-*` and `EVID-*`. |
| Gate | Is the package closed, blocked, deferred, or accepted with risk? | Review finding and decision. |

If any row is not applicable, mark it `n/a` with a reason. If any required row
is missing, the work package is not closed.

Good work package names describe the engineering product, not just activity:

- `WP-001: Add trace matrix validator`
- `WP-002: Implement JSON report interface`
- `WP-003: Integrate schema fixtures into CI`

Bad work package names hide scope:

- `cleanup`
- `refactor stuff`
- `random fixes`
- `make tests pass`

## Turning VTRACE Files Into Work Packages

Use this mapping sequence after the left-side files are drafted:

1. Start with each accepted `REQ-*` row.
2. Attach the related `ARCH-*`, `IF-*`, `DES-*`, and `CR-*` IDs.
3. Group IDs that must be implemented and verified together.
4. Create one work package for each coherent group.
5. Put any requirement that cannot be grouped into an explicit deferred,
   blocked, or discovery work package.

The mapping should be visible in `IMPLEMENTATION_PLAN.md` before coding starts.

| Source Item | Implementation Question | Work Package Rule |
|---|---|---|
| Mission need | Which user or operational outcome does this support? | Do not create a work package from mission text alone; require at least one `REQ-*`. |
| CONOPS scenario | Which workflow must be enabled or protected? | Create validation or integration work packages when scenarios need end-to-end proof. |
| Requirement | What must be built or changed? | Every accepted `REQ-*` is assigned to at least one work package or explicitly deferred. |
| Architecture element | Which component owns the behavior? | Work package names the affected component and boundary. |
| Package boundary | Which crate, package, module, adapter, schema, or generated artifact owns the behavior? | Work package names `PKG-*` IDs and allowed touches. |
| Interface | Which contract must be created or preserved? | Interface-changing work packages require compatibility and fixture evidence. |
| Design decision | Which implementation approach is controlled? | Work package entry criteria include accepted design decision. |
| Code-rigor constraint | Which implementation discipline must be proven? | Work package exit criteria include code-rigor evidence. |
| Verification item | How will the slice close? | Work package exit criteria include the command or inspection. |
| Validation scenario | What user-facing proof is affected? | Work package records validation impact and updates `VALIDATION.md`. |

If a proposed implementation slice has no parent `REQ-*`, `IF-*`, `DES-*`, or
`CR-*`, it is not ready to implement. Either add the missing left-side control
item or classify the work as discovery.

## Entry And Exit Criteria

Do not start a work package until entry criteria are satisfied. Examples:

- requirement accepted,
- interface owner identified,
- design decision recorded,
- test fixture format known,
- code-rigor tailoring complete.

Do not close a work package until exit criteria are satisfied. Examples:

- implementation completed,
- local verification passed,
- trace rows updated,
- evidence pointers recorded,
- L0/L1/L2 checks completed or explicitly dispositioned,
- required assurance/security lanes complete or accepted with risk,
- review findings resolved or accepted.

## Change Control

During implementation, changes are normal. Uncontrolled changes are the problem.

Create or update `CHANGE_CONTROL.md` when implementation changes:

- requirement meaning,
- public interface,
- architecture boundary,
- validation claim,
- verification method,
- accepted risk,
- code-rigor constraint.

Each change should record:

- change ID,
- reason,
- affected IDs,
- decision,
- reviewer or role lens,
- trace updates required.

## Integration Discipline

Implementation does not finish at local code completion. Integration is a
separate product-realization step.

Plan integration when:

- multiple modules interact,
- a public interface changes,
- downstream repos consume the artifact,
- generated files or schemas are involved,
- validation needs an end-to-end path.

Integration evidence should appear in `VERIFICATION.md`, not just in commit
messages.

## Review Gates During Implementation

Use these gates:

| Gate | When | Decision |
|---|---|---|
| Implementation Readiness Review | Before starting work packages. | Is the baseline clear enough to implement? |
| Work Package Close Review | Before marking a work package complete. | Did the slice satisfy its exit criteria? |
| Integration Readiness Review | Before integrating across modules/repos. | Are interfaces, fixtures, and rollback paths ready? |
| Test Readiness Review | Before formal verification/validation run. | Are commands, fixtures, and expected results ready? |
| Release/Transition Readiness Review | Before release, adoption, or public claim. | Is the product ready for its intended use or next layer? |

## Anti-Patterns

- Starting code before requirements and interfaces are testable.
- Calling arbitrary PRs "slices" without entry/exit criteria.
- Changing requirements inside a code patch without change control.
- Treating unit tests as integration or validation evidence.
- Merging implementation without updating trace rows.
- Closing a wave because code exists, not because evidence exists.

## Minimal Version For Small Repos

For small repos, use one implementation plan and one work package table. Keep it
short, but keep the control loop:

```text
baseline -> work package -> verify -> trace -> review
```
