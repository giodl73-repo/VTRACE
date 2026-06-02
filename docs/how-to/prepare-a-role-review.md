# Prepare A Role Review

1. Open `docs/vtrace/REVIEW.md`.
2. Confirm required lanes for the scoped change.
3. Run:

```powershell
vtrace roles review WP-001 <repo>
vtrace roles run WP-001 <repo>
```

4. Ask each lane to review against its concern:
   systems engineering, requirements traceability, V&V, software assurance,
   security/privacy, safety/risk, source custody, configuration control,
   adoption guidance, and stakeholder impact.
5. Record decisions and evidence in `REVIEW.md`.

Expected result: required lanes are `pass`, `pass_with_risk`, or explicitly
blocked/deferred before public readiness claims are made.
