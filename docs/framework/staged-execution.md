# Staged Execution

Staged execution turns VTRACE from documents into an implementation control
loop. Each stage has entry criteria, required artifacts, validation level, role
review, and a decision.

## Stage Ladder

| Stage | Purpose | Required Artifacts | Minimum Validation | Gate |
|---|---|---|---|---|
| S0 Intake | Decide whether VTRACE applies and what scope is controlled. | Assessment notes or `MISSION.md` draft. | None or docs sanity. | Intake decision. |
| S1 Specification Baseline | Control mission, CONOPS, requirements, and current/target behavior. | `MISSION.md`, `CONOPS.md` if needed, `REQUIREMENTS.md`, `SPECIFICATION_BASELINE.md`. | Requirement and specification quality review. | Specification review. |
| S2 Design Baseline | Control architecture, package boundaries, interfaces, design, and code rigor. | `ARCHITECTURE.md`, `PACKAGE_BOUNDARIES.md`, `INTERFACES.md`, `DESIGN.md`, `CODE_RIGOR.md` as needed. | Design inspection; interface/schema checks if present. | Design review. |
| S3 Implementation Planning | Convert accepted left-side items into work packages. | `IMPLEMENTATION_PLAN.md`, `WORK_PACKAGES.md`; optional `CHANGE_CONTROL.md`, `INTEGRATION_PLAN.md`. | Orphan check; source-to-WP mapping check. | Implementation readiness review. |
| S4 Work Package Execution | Implement one controlled package. | Active `WP-*`, code/docs/artifacts, trace updates. | L0 before commit; L1 before push. | Work package close review. |
| S5 Integration | Combine packages/components/downstream surfaces. | `INTEGRATION_PLAN.md`, interface fixtures, integration evidence. | L2 when integration affects claims or consumers. | Integration readiness / test readiness review. |
| S6 Readiness / Transition | Decide whether to release, merge, adopt, or claim readiness. | `TRACE.md`, `VERIFICATION.md`, `VALIDATION.md`, `REVIEW.md`. | Required L2 and role lanes complete. | Readiness review. |

## Completeness Rule

A stage is complete only when:

- required artifacts exist or are explicitly not required,
- open findings are dispositioned,
- required validation level passed or is explicitly deferred,
- required role lanes are complete,
- trace rows and evidence pointers are updated,
- the gate decision is recorded.

## Stage Status Values

Use these status values:

- `not_started`
- `in_progress`
- `blocked`
- `deferred`
- `pass`
- `pass_with_risk`

Do not use `complete` without a gate decision. `pass_with_risk` is often the
honest answer for useful work with known evidence limits.

## Execution Board

Use this table in implementation plans or review notes:

| Stage | Status | Gate Decision | Required Next Action |
|---|---|---|---|
| S0 Intake | pass | pass |  |
| S1 Specification Baseline |  |  |  |
| S2 Design Baseline |  |  |  |
| S3 Implementation Planning |  |  |  |
| S4 Work Package Execution |  |  |  |
| S5 Integration |  |  |  |
| S6 Readiness / Transition |  |  |  |

## Agent Execution

Agents should be assigned a stage and a work package. A good agent handoff says:

```text
Stage: S4 Work Package Execution
Work package: WP-002
Allowed boundary IDs: PKG-001, IF-002
Required L0: ...
Required L1: ...
Stop when: WP-002 V closure rows are complete or a blocker is found.
```

If the agent discovers missing left-side control, it should stop and recommend
a change-control or design-baseline update instead of patching around the gap.

## Existing Repo Spec Rule

For existing repos, S1 is not complete until current behavior is classified as
`current`, `target`, `deprecated`, or `unknown` in
`SPECIFICATION_BASELINE.md`. Unknown behavior that affects implementation must
be resolved or converted into a discovery work package before S3 passes.
