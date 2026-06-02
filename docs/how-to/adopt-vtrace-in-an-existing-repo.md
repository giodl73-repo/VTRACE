# Adopt VTRACE In An Existing Repo

1. Create `docs/vtrace/`.
2. Add the minimum slice: `MISSION.md`, `REQUIREMENTS.md`,
   `SPECIFICATION_BASELINE.md`, `TRACE.md`, `VERIFICATION.md`, and
   `REVIEW.md`.
3. Classify existing behavior in the spec baseline as `current`, `target`,
   `deprecated`, or `unknown`.
4. Add `WORK_PACKAGES.md` before implementation starts.
5. Give each work package parent IDs and L0/L1/L2 checks.
6. Run `vtrace validate <repo>`.
7. Close the package only after evidence, validation, and review are recorded.

Expected result: `vtrace validate <repo>` reports `VTRACE validation passed` or
prints actionable findings with file and line references.
