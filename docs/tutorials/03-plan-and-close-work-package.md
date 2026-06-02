# Tutorial 03: Plan And Close A Work Package

Goal: execute one bounded VTRACE implementation slice.

1. Add a `WP-*` row in `WORK_PACKAGES.md`.
2. Include parent IDs, affected surfaces, entry criteria, exit criteria, and
   L0/L1/L2 checks.
3. Run `vtrace work start WP-001 <repo>`.
4. Implement only the affected surfaces.
5. Run `vtrace work check WP-001 <repo>`.
6. Update verification, validation, evidence, trace, and review files.
7. Run `vtrace work close WP-001 <repo>`.

Expected result: the close command reports validator findings, work-package
status, review lanes, affected surfaces, git scope, and changed paths before
passing or blocking closure.
