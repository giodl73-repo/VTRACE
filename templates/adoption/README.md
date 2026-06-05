# VTRACE Adoption Templates

Copy the needed templates into a target repo under `docs/vtrace/`.

Minimum first slice:

- `MISSION.md`
- `REQUIREMENTS.md`
- `TRACE.md`
- `VERIFICATION.md`
- `REVIEW.md`

Add `CONOPS.md`, `ARCHITECTURE.md`, `INTERFACES.md`, `DESIGN.md`, and
`VALIDATION.md` as risk, maturity, and adoption scope require. Add
`CODE_RIGOR.md` before implementation when the repo needs explicit coding
constraints for reviewability, static analysis, complexity, assertions, or
error-handling discipline.

For non-trivial implementation, also add:

- `PROBLEM_SPACE_MAP.md`
- `DOMAIN_BACKLOG.md`
- `RESEARCH_PLAN.md`
- `SPEC_MODEL.md`
- `CONTRACT_BOUNDARIES.md`
- `SCENARIO_MODEL.md`
- `DIAGNOSTIC_MODEL.md`
- `FIXTURE_MODEL.md`
- `IMPLEMENTATION_PLAN.md`
- `WORK_PACKAGES.md`

Add `CHANGE_CONTROL.md` when implementation changes controlled requirements,
interfaces, design, validation claims, or verification methods. Add
`INTEGRATION_PLAN.md` when multiple components, schemas, generated artifacts, or
downstream consumers are involved.

Use `docs/framework/execution-control.md` for recommended Git branch/worktree
discipline, commit/push policy, L0/L1/L2 validation levels, and agent handoff
rules.

Use `STAGE_EXECUTION.md` to drive staged completeness and
`ROLE_RECOMMENDATIONS.md` to define a ROLES-conformant target review panel.
Use `PULSE_EXECUTION.md` inside repo-local pulse files to connect TRACKER-style
wave/pulse execution with VTRACE work packages.
