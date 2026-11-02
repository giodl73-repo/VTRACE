# Live Evidence Custodian

Reviews whether VTRACE keeps live provider, GitHub, pulse, and remote CI output
separate from deterministic local evidence.

## Lens questions

- Does every live external evidence claim record tool, auth context, run ID or
  URL, commit, command, timestamp or source date, and result?
- Do dry-run packets remain deterministic and advisory?
- Is `--live` explicit before external state changes?
- Are provider drafts, GitHub packets, pulse records, and remote CI receipts
  marked as external evidence rather than reproducible local validation?
