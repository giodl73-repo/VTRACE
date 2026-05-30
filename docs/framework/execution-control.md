# Execution Control

Execution control defines how agents and maintainers implement VTRACE work
packages in Git without losing traceability, evidence, or review boundaries.

## Git Discipline

Use one of these scopes for every implementation branch or worktree:

- one work package,
- one review-gate fix set,
- one integration package,
- one explicitly named discovery package.

Avoid mixing unrelated requirements or unrelated work packages in one commit.

## Branch And Worktree Rules

For multi-agent or risky work, prefer separate Git worktrees:

```powershell
git worktree add ..\<repo>-wp-001 -b wp-001/<short-name>
```

Each worktree should have:

- one active `WP-*`,
- its own validation commands,
- no unrelated repo cleanup,
- trace updates for the work package,
- review findings recorded before merge.

Agents should read, in order:

1. `docs/vtrace/IMPLEMENTATION_PLAN.md`
2. `docs/vtrace/WORK_PACKAGES.md`
3. the active work package details,
4. linked requirements/design/interface/code-rigor files,
5. verification and validation plans.

## Commit Rules

Commit messages should name the work package or controlled artifact:

```text
WP-001 implement JSON report interface
WP-001 add schema fixtures and verification evidence
CHG-002 update report timestamp requirement
REVIEW close WP-001 readiness findings
```

Before committing:

- run the package's L0 checks,
- update `TRACE.md`,
- update `VERIFICATION.md` evidence pointers,
- update `WORK_PACKAGES.md` status,
- record review findings or accepted risk when relevant.

Before pushing:

- run L1 checks for the work package,
- ensure branch contains only intended files,
- ensure no unrelated dirty state remains,
- include evidence commands in the PR, review note, or wave/pulse record.

Before merging or marking ready:

- run L2 checks when the package affects integration, public interfaces,
  validation claims, downstream consumers, or release readiness.

## Validation Levels

VTRACE uses three validation levels so agents know how much proof is needed.

| Level | Purpose | Typical Commands | Required When |
|---|---|---|---|
| L0 | Fast local sanity for one work package. | format check, focused unit test, syntax/schema check, targeted lint. | Before every commit. |
| L1 | Full repo confidence for the work package. | repo test suite, full lint/static checks, fixture/golden checks, doc checks. | Before push or PR. |
| L2 | Integration/readiness proof. | end-to-end tests, downstream fixture checks, release build, scenario validation, role/review gate. | Before merge, release, downstream adoption, or public claim. |

Each work package should list its required L0, L1, and L2 checks. Low-risk docs
work may mark L2 as not required, but the reason must be explicit.

## Completeness Driver

Execution completeness is checked in stages, not by vibes. Use
`STAGE_EXECUTION.md` to record:

- stage status,
- gate decision,
- required artifacts,
- validation level,
- required role lanes,
- evidence pointer,
- next action.

No stage is complete until its gate decision is recorded.

## Agent Handoff Rules

When handing a work package to an agent, provide:

- repo path,
- branch or worktree path,
- active `WP-*`,
- allowed files or modules,
- L0/L1/L2 commands,
- expected trace/evidence updates,
- stop condition.

The agent should return:

- files changed,
- commands run and results,
- trace/evidence updates,
- review findings,
- open risks,
- whether the work package is complete, blocked, or ready for review.

## Wave/Pulse Integration

Repos that already use TRACKER-style waves and pulses should record work-package
execution inside the repo-local wave history. VTRACE does not replace that
model; it gives each pulse stronger engineering control.

Recommended mapping:

| VTRACE Item | Wave/Pulse Record |
|---|---|
| Implementation baseline | Active wave `WAVE.md` goal/thesis and pulse table. |
| Work package | One pulse, or a clearly named group of pulses when the package is large. |
| L0/L1/L2 checks | Pulse `Validation` section. |
| Evidence pointers | Pulse `Evidence` or `Outcome` section plus `VERIFICATION.md`. |
| Review findings | Pulse status notes plus `REVIEW.md`. |
| Accepted risk | Pulse status notes plus `REVIEW.md` accepted-risk table. |
| Blocker | Pulse status `blocked`, with next required decision. |

Each pulse that executes VTRACE work should name:

- active `WP-*`,
- parent `REQ-*` / `DES-*` / `IF-*` / `CR-*`,
- allowed boundary IDs,
- L0/L1/L2 commands,
- trace/evidence files updated,
- review gate decision.

For large work packages, split execution into multiple pulses but keep one
`WP-*` owner record in `WORK_PACKAGES.md`. Do not create orphan pulses that
change implementation without a work-package ID.

## Commit/Pulse Alignment

Prefer one commit per completed pulse when practical. If a pulse spans multiple
commits, each commit should still name the active `WP-*` or review/change ID.

Examples:

```text
WP-002 pulse 01 define JSON schema fixture
WP-002 pulse 02 implement report writer
WP-002 pulse 03 close integration verification
```

The final pulse for a work package should update:

- `WORK_PACKAGES.md` status,
- `TRACE.md`,
- `VERIFICATION.md`,
- `VALIDATION.md` if affected,
- `REVIEW.md`.

## Discovery Work

Discovery is allowed when the implementation path is unknown, but it still gets
controlled:

- use a `WP-DISC-*` or `WP-*` marked `discovery`,
- define the question being answered,
- restrict allowed changes,
- record findings,
- convert accepted findings into normal requirements/design/work packages before
  production implementation.

Discovery code should not become production code without a normal work package
and review gate.
