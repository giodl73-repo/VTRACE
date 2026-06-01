# CONOPS

## Scope

Repo: VTRACE

## Operating Scenarios

| ID | Actor | Trigger | Normal Path | Failure / Degraded Path | Output |
|---|---|---|---|---|---|
| CON-001 | Maintainer | Existing repo lacks rigor before a feature or release. | Run `vtrace-assess`, create minimum docs, baseline specs, plan work packages, execute gates. | Missing evidence becomes a finding or discovery work package. | Repo-local `docs/vtrace/` package. |
| CON-002 | Agent | Asked to implement a VTRACE-scoped change. | Read target repo VTRACE docs, pick one `WP-*`, run L0/L1/L2 as required, update trace/evidence. | Stop when parent IDs or spec baseline are missing. | Controlled patch and updated evidence. |
| CON-003 | Reviewer | Asked whether a repo can claim readiness. | Run `vtrace-gate`, inspect trace, verification, validation, work-package closure, and role lanes. | Use `blocked`, `deferred`, or `pass_with_risk` when evidence is incomplete. | Review decision in `REVIEW.md`. |
| CON-004 | CLI operator | Asked to run VTRACE procedurally in an existing repo. | Use CLI commands to initialize artifacts, inspect status, start/check/close one work package, record evidence, prepare role review, and generate bounded agent briefs. | CLI refuses closure or marks risk when trace, evidence, checks, or review lanes are incomplete. | Auditable VTRACE artifacts and optional execution receipts. |

## Handoffs

- VTRACE framework docs define process.
- Templates create target repo artifacts.
- Skills guide agent execution.
- Target repo pulse/wave records hold execution history when present.
