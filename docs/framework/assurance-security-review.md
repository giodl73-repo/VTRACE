# Assurance And Security Review

NASA-style software implementation does not rely on developer self-attestation.
VTRACE adapts that discipline with role-based review gates for assurance,
security, safety, risk, independent verification, and configuration control.

## Source Basis

NASA software requirements and assurance sources establish a few useful
principles for VTRACE:

- software work spans planning, development, testing, operations, maintenance,
  retirement, management, acquisition, and assurance,
- software assurance includes quality, safety, reliability, mission software
  cybersecurity assurance, verification and validation, and IV&V,
- software assurance and software safety apply through the full software life
  cycle,
- lifecycle reviews and independent review boards provide objective judgments,
- cybersecurity policies are accessed through NODIS and NASA standards channels.

VTRACE does not claim NASA compliance. It uses these ideas as a practical review
model for ordinary repos.

## Review Lanes

Every non-trivial work package should classify which lanes apply.

| Lane | Use When | Review Evidence |
|---|---|---|
| Systems engineering | Requirements, architecture, interfaces, or validation claims change. | Systems engineering role finding. |
| Requirements traceability | Any accepted `REQ-*` is added, changed, implemented, waived, or retired. | Traceability role finding. |
| V&V | Verification or validation method, result, or claim changes. | V&V role finding. |
| Software assurance | Code rigor, static analysis, reliability, generated code, or critical implementation changes. | Assurance role finding. |
| Security/privacy | Inputs, auth, secrets, network, files, external services, user data, or supply chain are affected. | Security role finding. |
| Safety/mission impact | Failure can cause physical, operational, financial, civic, legal, or trust harm. | Safety/risk role finding. |
| Source custody | Standards, external sources, license, export, data custody, or provenance changes. | Source custody role finding. |
| Configuration/change control | Baselines, public interfaces, releases, generated artifacts, or downstream consumers change. | Change-control role finding. |

## Role Review Matrix

Add a role review matrix to `REVIEW.md` and work-package close records.

Decision values:

- `not_required`
- `pending`
- `pass`
- `pass_with_risk`
- `blocked`
- `deferred`

No work package can close while a required lane is `pending` or `blocked`.

## Security Questions

Use these questions even for non-security repos:

- Does the package introduce or change input parsing?
- Does it read, write, delete, or move files?
- Does it touch credentials, tokens, secrets, or identity?
- Does it add network access or external service dependency?
- Does it change dependency versions or supply-chain posture?
- Does it expose user, customer, civic, operational, or proprietary data?
- Does it change permissions, sandboxing, auth, audit logs, or retention?
- Could generated output cause unsafe automation or misleading claims?

If any answer is yes, require a security/privacy lane.

## Safety And Risk Questions

Require safety/risk review when failure could affect:

- physical safety,
- operational continuity,
- finance or compliance,
- public/civic trust,
- election or districting outputs,
- medical/legal/security advice,
- downstream automation,
- irreversible user action.

## Independent Review

For high-risk packages, require one independent lens that did not author the
implementation. In VTRACE this can be a simulated role review, but the review
record must say it is simulated and must name the inspected evidence.

## Tailoring

Low-risk docs-only packages may mark security, safety, and IV&V lanes
`not_required`, but the reason should be visible. High-risk packages should not
skip lanes silently.
