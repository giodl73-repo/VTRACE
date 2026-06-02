# Minimum Package Example

Minimum files:

```text
docs/vtrace/MISSION.md
docs/vtrace/REQUIREMENTS.md
docs/vtrace/SPECIFICATION_BASELINE.md
docs/vtrace/TRACE.md
docs/vtrace/VERIFICATION.md
docs/vtrace/REVIEW.md
```

Minimum trace row shape:

```text
REQ-001 -> SPEC-001 -> WP-001 -> EVID-001 -> verified
```

Expected command:

```powershell
vtrace validate <repo>
```

Expected output:

```text
VTRACE validation passed
```
