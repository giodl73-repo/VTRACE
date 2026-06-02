# VTRACE In One Page

VTRACE is a lightweight V-model discipline for software repos. It creates a
traceable line from why a repo exists to what it promises, what gets built, how
it is verified, how it is validated, and what evidence proves the claim.

The core idea:

```text
mission -> CONOPS -> requirements -> specs -> design -> work package
work package -> verification -> validation -> evidence -> review
```

For a small repo, start with:

- `docs/vtrace/MISSION.md`
- `docs/vtrace/REQUIREMENTS.md`
- `docs/vtrace/SPECIFICATION_BASELINE.md`
- `docs/vtrace/TRACE.md`
- `docs/vtrace/VERIFICATION.md`
- `docs/vtrace/REVIEW.md`

Add CONOPS, architecture, interfaces, package boundaries, code rigor, detailed
design, implementation planning, validation, evidence, and communications
strategy when the repo needs that level of control.

VTRACE is NASA-inspired, not NASA-authored or NASA-endorsed.
