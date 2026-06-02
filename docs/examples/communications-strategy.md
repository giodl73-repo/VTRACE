# Communications Strategy Example

Small repos can start with three rows:

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
|---|---|---|---|---|---|---|---|
| COMMS-README-001 | NEED-001 | users | Where do I start? | `docs/README.md` | every docs wave | maintainer | proposed |
| COMMS-CONCEPTS-001 | REQ-001 / SPEC-001 | users / contributors | What model explains this repo? | `docs/concepts/` | when specs change | maintainer | proposed |
| COMMS-TRACES-001 | WP-001 / EVID-001 | reviewers | How was this feature proven? | `docs/traces/` | when work closes | maintainer | proposed |

Expected command:

```powershell
vtrace comms plan <repo>
```

Expected result: the command prints the docs/spec boundary, source mapping, and
declared surfaces.
