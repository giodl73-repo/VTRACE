# Pulse 07: Implementation Management

## Goal

Add the missing implementation procedure so VTRACE adopters do not fill out
stage files and then implement arbitrary slices.

## Changes

- Add implementation management framework guidance.
- Add implementation plan, work package, change-control, and integration-plan
  templates.
- Add execution-control guidance for Git worktrees, commits, pushes, agent
  handoffs, and L0/L1/L2 validation levels.
- Add assurance/security role-review guidance and simulated role review.
- Add staged execution and target-repo role recommendation guidance.
- Add wave/pulse integration guidance so VTRACE work packages are recorded in
  repo-local pulse history.
- Update adoption, assessment, gate, and process docs to require controlled
  work packages for non-trivial implementation.
- Add NPR 7120.5E as project-management source metadata.

## Validation

- `git diff --check`
- JSON parse check for `sources/source-registry.json`

## Status

Complete.
