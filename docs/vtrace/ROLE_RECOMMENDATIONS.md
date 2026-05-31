# Role Recommendations

## Scope

Repo: VTRACE

ROLES conformance target: panel

## Recommended Panel

| Role | Tier | Required | Trigger | Local File |
|---|---|---|---|---|
| Systems Engineering Steward | parliament | yes | VTRACE process, stage, or V-shape changes. | `.roles/parliament/systems-engineering-steward.md` |
| Requirements Traceability Auditor | parliament | yes | Requirement, spec, trace, or DCR changes. | `.roles/parliament/requirements-traceability-auditor.md` |
| Verification and Validation Lead | parliament | yes | Verification, validation, evidence, example, or gate changes. | `.roles/parliament/verification-validation-lead.md` |
| Software Assurance Guardian | parliament | yes | Validator, code-rigor, language-profile, or implementation-risk changes. | `.roles/parliament/software-assurance-guardian.md` |
| Security Privacy Guardian | parliament | conditional | Security/privacy/supply-chain/data trigger. | `.roles/parliament/security-privacy-guardian.md` |
| Safety Risk Officer | parliament | conditional | Safety, high-consequence, or mission-impacting adoption trigger. | `.roles/parliament/safety-risk-officer.md` |
| Source Custody Counsel | parliament | yes | Source, license, standards, provenance, or encoding trigger. | `.roles/parliament/source-custody-counsel.md` |
| Adoption Guide Editor | editorial | yes | Public docs, templates, examples, or user-facing guidance. | `.roles/editorial/adoption-guide-editor.md` |
| Template Minimalism Editor | editorial | yes | Template expansion or process-weight change. | `.roles/editorial/template-minimalism-editor.md` |
| Repo Maintainer | stakeholders | yes | Usability and maintenance impact. | `.roles/stakeholders/repo-maintainer.md` |
| Future Agent | stakeholders | yes | Resumability and trace continuity. | `.roles/stakeholders/future-agent.md` |

## Review Order

1. Systems Engineering Steward checks whether the change belongs in VTRACE.
2. Requirements Traceability Auditor checks IDs and coverage.
3. Source Custody Counsel checks source and rights posture when source claims change.
4. V&V Lead checks evidence, validation levels, and gate posture.
5. Software Assurance Guardian checks code-rigor and validator/profile impact.
6. Conditional security/safety roles review only when triggered.
7. Editorial and stakeholder roles check usability, template weight, and future-agent continuity.

## Current Role Decision

Self-adoption gate requires systems engineering, traceability, V&V, software
assurance, source custody, configuration/change control, adoption guide, repo
maintainer, and future-agent review. Security/privacy and safety/risk are not
required for this docs-only baseline, but become required for affected future
DCRs.
