# Change Control

## Scope

Repo: VTRACE

## DCR Table

| DCR ID | Change Request | Driver | Affected IDs | Decision | Target WP | Notes |
|---|---|---|---|---|---|---|
| DCR-001 | Add lightweight automated VTRACE validators. | Missing enforcement. | REQ-VAL-001, SPEC-001..SPEC-005, CR-001..CR-003 | accepted | WP-001 | Check artifact presence, ID coverage, work-package closure, and review lanes. |
| DCR-002 | Add language/package profiles. | Existing repos need concrete validation profiles. | REQ-PROFILE-001, CR-004 | accepted | WP-002 | Rust, Python, TypeScript/frontend, generated code, docs-only, multi-language. |
| DCR-003 | Add a realistic existing-repo migration example. | Hello-world is too small to prove retrofit adoption. | CON-001, SPEC-004, SPEC-005 | accepted | WP-003 | Use a messy repo scenario with current/target/unknown behavior. |
| DCR-004 | Add evidence ledger template/schema and automation. | Evidence pointers need a reusable durable shape for target repos. | IF-003, SPEC-005 | accepted | WP-004 | VTRACE now has `docs/vtrace/EVIDENCE.md`; target-repo template/schema remain open. |
| DCR-005 | Add gate-specific checklists. | Review gates need sharper execution criteria. | SPEC-001, SPEC-005 | accepted | WP-005 | Spec, design, implementation readiness, WP close, TRR/readiness. |
| DCR-006 | Deepen NASA source specificity encoding. | Framework should capture review, traceability, assurance, and configuration management patterns more explicitly. | NEED-002, SPEC-001 | accepted | WP-006 | Derived guidance only; no NASA compliance claim. |

## Change-Control Rule

Any change that alters VTRACE stage semantics, ID conventions, template
contracts, skills, source custody, or public adoption guidance gets a `DCR-*`
entry or updates an existing one.
