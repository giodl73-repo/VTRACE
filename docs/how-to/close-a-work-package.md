# Close A Work Package

1. Confirm the work package parent IDs still match the change.
2. Run the listed L0/L1/L2 commands.
3. Update `VERIFICATION.md` with command or inspection results.
4. Update `VALIDATION.md` with scenario impact.
5. Add evidence rows in `EVIDENCE.md`.
6. Update `TRACE.md` evidence pointers.
7. Record review lane decisions in `REVIEW.md`.
8. Update any user docs listed in `COMMUNICATIONS_STRATEGY.md`.
9. Run `vtrace work close <WP-ID> <repo>`.

Expected result: close passes only when validator findings are resolved,
work-package status is complete, review lanes are recorded, and git scope is
clean.
