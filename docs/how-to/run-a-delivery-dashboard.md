# Run a Delivery Dashboard

Use this guide when a VTRACE adoption needs a compact operating view for the current work. A delivery dashboard is a short artifact that answers three questions quickly:

1. What should we do next?
2. What evidence is still required?
3. What is blocked or still unproven?

## When to use it

Create or update a delivery dashboard when:

- a repo has a growing VTRACE spine but the active work is hard to read quickly,
- multiple work packages are in flight and the team needs a short summary,
- a review or readiness gate needs a concise evidence-backed status view.

## Recommended contents

A delivery dashboard should stay short. Recommended sections:

- Current delivery posture: one paragraph summarizing the active product focus.
- Priority packages: the next one or two packages that should move.
- Evidence status: the verification commands or evidence rows that still need proof.
- Blockers: anything that prevents closeout or promotion.
- Next action: one concrete action for each package.

## Suggested file layout

Keep the dashboard in the repo-local VTRACE surface, for example:

- `docs/vtrace/DELIVERY_DASHBOARD.md` for a product repo adoption
- `docs/vtrace/agent-platform-toolchain/DELIVERY_DASHBOARD.md` for a scoped product track

The dashboard should point back to the authoritative files:

- `MISSION.md`
- `REQUIREMENTS.md`
- `TRACE.md`
- `WORK_PACKAGES.md`
- `VERIFICATION.md`
- `VALIDATION.md`

## Operating rule

A dashboard is only useful if it is evidence-backed. If a package cannot point to a real verification command, evidence row, or validation output, the dashboard should say that plainly rather than implying readiness.

## Minimal template

Use this shape:

```md
# Delivery Dashboard

## Current delivery posture

- Product focus:
- Current milestone:
- Main risk:

## Priority packages

- Package: <id>
  - Objective:
  - Suggested next action:
  - Evidence still required:
  - Blockers:

## Evidence status

- Verification command or evidence row:
- Current status:

## Blockers

- <list>
```

## Adoption note

This pattern helps keep VTRACE lightweight. The dashboard is a working summary, not a replacement for the requirement spine, the work-package ledger, or the evidence rows.
