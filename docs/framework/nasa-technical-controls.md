# NASA-Inspired Technical Controls

VTRACE derives practical controls from public NASA systems/software engineering
source categories without asserting NASA compliance.

## Derived Control Map

| Control Area | VTRACE Adaptation | Primary Artifacts |
|---|---|---|
| Technical requirements definition | Requirements and specs are baselined before design/implementation. | `REQUIREMENTS.md`, `SPECIFICATION_BASELINE.md` |
| Technical assessment | Reviews are scoped, evidence-based, and decision-oriented. | `REVIEW.md`, `REVIEW_CHECKLISTS.md` |
| Technical data management | Evidence, source basis, trace, and command receipts are retained. | `TRACE.md`, `EVIDENCE.md`, `SOURCE_BASIS.md` |
| Configuration management | Semantics, templates, roles, and public contracts change through DCRs. | `CHANGE_CONTROL.md`, `WORK_PACKAGES.md` |
| Interface management | APIs, CLIs, schemas, config, files, and generated artifacts are controlled surfaces. | `INTERFACES.md`, `PACKAGE_BOUNDARIES.md` |
| Verification | Built-correctly evidence is planned by requirement/spec/work package. | `VERIFICATION.md` |
| Validation | Built-right evidence ties to mission, CONOPS, user workflow, or acceptance scenario. | `VALIDATION.md` |
| Software assurance and safety | Role lanes classify assurance, safety/risk, security/privacy, and source custody triggers. | `REVIEW.md`, `.roles/` |

## Review Depth By Risk

- Low-risk docs-only changes need traceability, source custody when sources are
  affected, and docs sanity checks.
- Public contract changes need specification, design, implementation readiness,
  and release/transition review.
- Generated artifacts need source-of-truth, reproducibility, fixture, and
  downstream consumption evidence.
- Safety, security, privacy, or high-consequence changes require the relevant
  role lanes and cannot close on happy-path tests alone.

## Source Posture

The source registry remains canonical for URLs, rights posture, download policy,
and source use. This file is locally authored derived guidance, not substituted
NASA text.
