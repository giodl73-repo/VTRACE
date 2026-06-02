# VTRACE CLI Orchestrator

## Purpose

The VTRACE CLI should make the VTRACE process executable for maintainers and
agents without making AI output the source of truth. The CLI is a process
runner, gap finder, evidence recorder, and work-package guide.

## Design Decision

VTRACE should grow from validator to orchestrator in controlled layers:

1. Keep deterministic Markdown artifacts as the canonical record.
2. Add commands that help users initialize, inspect, execute, and close VTRACE
   work packages.
3. Add agent/worktree briefs after work-package semantics are stable.
4. Add LLM/provider integrations only as optional proposal and review helpers.

LLM output is advisory until a maintainer or agent writes it into controlled
VTRACE artifacts with trace IDs, evidence, and review status.

## Proposed Command Surface

| Command | Purpose | Writes |
|---|---|---|
| `vtrace init` | Create a minimum `docs/vtrace/` package from templates. | Stage artifacts under `docs/vtrace/`. |
| `vtrace status` | Summarize trace gaps, open risks, incomplete evidence, and work-package state. | none by default. |
| `vtrace validate` | Run deterministic validation checks against the target repo. | none by default. |
| `vtrace plan` | Propose or refresh DCR and work-package candidates from existing artifacts. | optional DCR/WP draft files. |
| `vtrace work start WP-###` | Show entry criteria, parent IDs, affected surfaces, checks, and review lanes. | execution log or pulse linkage. |
| `vtrace work check WP-###` | Run configured L0/L1/L2 checks for the selected work package. | evidence receipt draft. |
| `vtrace work close WP-###` | Require exit criteria, evidence rows, validation status, review lanes, and git scope. | `WORK_PACKAGES.md`, `VERIFICATION.md`, `VALIDATION.md`, `EVIDENCE.md`, `REVIEW.md`. |
| `vtrace roles review WP-###` | Run or prepare the required `.roles` review lanes. | review record draft. |
| `vtrace agent brief WP-###` | Produce a bounded agent brief for one work package. | brief file or stdout. |
| `vtrace worktree status` | List git worktrees, whether each has a VTRACE ownership record, and the claimed `WP-*` when available. | none. |
| `vtrace worktree plan WP-###` | Derive the branch, path, and command for isolated worktree execution. | none by default. |
| `vtrace worktree create WP-###` | Create a repo-local worktree for isolated execution when the source repo is clean, the target path does not exist, and no active VTRACE-owned worktree already claims the same `WP-*` unless explicitly allowed. | local worktree plus `.vtrace/worktree.md` and `.vtrace/agent-brief.md`. |
| `vtrace worktree remove <path>` | Remove a VTRACE-owned worktree after confirming its ownership record, or require `--force`. | removes local worktree. |

## Work-Package Execution Contract

`vtrace work start` must show:

- parent `NEED-*`, `CON-*`, `REQ-*`, `SPEC-*`, `CR-*`, `IF-*`, and `DCR-*`
  links where present,
- package/crate/module boundaries,
- allowed and forbidden implementation surfaces,
- entry criteria and stop conditions,
- L0/L1/L2 checks and waiver rules,
- required role-review lanes,
- evidence rows expected at closeout,
- git branch, worktree, commit, push, and tracker-pointer guidance.

`vtrace work close` must refuse closure or mark `pass_with_risk` unless:

- required trace links are present,
- required checks have results or accepted waivers,
- required role lanes are recorded,
- evidence rows contain command/review receipts,
- validation impact is recorded,
- git status and commit scope are inspected.

## Provider Integration Contract

Provider support should be an adapter layer, not a dependency of the core
process. Adapters may help with:

- draft mission, CONOPS, requirements, or specs from repo context,
- trace-gap analysis,
- work-package decomposition,
- role-review summaries,
- evidence summarization,
- PR or review-note drafting.

Adapters must not:

- silently edit controlled artifacts,
- mark evidence as passed,
- close work packages,
- run unbounded shell commands,
- bypass source custody,
- promote public claims without review.

Provider output should be stored as `proposed`, `review_needed`, or
`accepted_with_changes` until committed into canonical artifacts.

## Agent And Worktree Model

Agent execution should be one work package at a time. A generated brief should
include:

- mission and CONOPS parentage,
- requirement and spec IDs,
- package boundaries,
- allowed files and surfaces,
- required checks,
- required reviews,
- evidence rows to update,
- stop conditions,
- commit and push guidance.

Worktrees are appropriate when parallel agents are implementing separate
work packages. The CLI should prevent two active worktrees from claiming the
same `WP-*` unless a maintainer explicitly allows it.

Created worktrees should contain local `.vtrace/worktree.md` and
`.vtrace/agent-brief.md` files. The ownership record names the source repo,
worktree path, branch, `WP-*`, and closeout commands. The agent brief gives the
implementer the bounded package context and stop conditions. These files are
local execution context, not objective evidence by themselves.

Worktree removal should require the ownership record unless the operator passes
`--force`; this prevents the CLI from deleting arbitrary git worktrees by
accident.

Worktree creation should refuse a duplicate active worktree for the same
`WP-*` unless the operator passes an explicit duplicate override. Parallel
worktrees are useful, but duplicate package ownership should be intentional.

## MVP Boundary

The first CLI orchestrator implementation should include:

- `vtrace init`,
- `vtrace status`,
- `vtrace validate`,
- `vtrace work start`,
- `vtrace work check`,
- `vtrace work close`,
- evidence receipt generation.

The first implementation should not require network access, provider keys, or
third-party runtime services.

## Later Boundary

Later DCRs may add:

- LLM provider adapters,
- `.roles` execution integrations,
- stronger worktree locking and active-package ownership records,
- generated adoption reports,
- GitHub issue/PR review helpers,
- TRACKER wave/pulse synchronization.

## Validation

This design is ready for implementation planning when:

- the proposed commands map to current VTRACE artifacts,
- no command weakens the deterministic validator contract,
- provider output remains advisory,
- work-package closeout preserves objective evidence,
- role, security, source custody, and configuration concerns have explicit
  command boundaries.
