# Change Control

## Scope

Repo: VTRACE

## DCR Table

| DCR ID | Change Request | Driver | Affected IDs | Decision | Target WP | Notes |
|---|---|---|---|---|---|---|
| DCR-001 | Add lightweight automated VTRACE validators. | Missing enforcement. | REQ-VAL-001, SPEC-001..SPEC-005, SPEC-007, CR-001..CR-003 | implemented | WP-001 | The Rust `vtrace` CLI checks artifact presence, trace visibility, evidence pointers, work-package shape, and review lanes. |
| DCR-002 | Add language/package profiles. | Existing repos need concrete validation profiles. | REQ-PROFILE-001, SPEC-008, CR-004 | implemented | WP-002 | Profiles cover Rust, Python, TypeScript/frontend, generated code, docs-only, and multi-language repos. |
| DCR-003 | Add a realistic existing-repo migration example. | Hello-world is too small to prove retrofit adoption. | REQ-EXAMPLE-001, CON-001, SPEC-004, SPEC-005, SPEC-009 | implemented | WP-003 | `examples/existing-repo-migration/` shows current/target behavior and one closed work package. |
| DCR-004 | Add evidence ledger template/schema and automation. | Evidence pointers need a reusable durable shape for target repos. | REQ-EVIDENCE-001, IF-003, SPEC-005, SPEC-010 | implemented | WP-004 | Added `templates/adoption/EVIDENCE.md`, `schemas/evidence-ledger.schema.json`, and validator evidence-ledger checks. |
| DCR-005 | Add gate-specific checklists. | Review gates need sharper execution criteria. | REQ-GATE-001, SPEC-001, SPEC-005, SPEC-011 | implemented | WP-005 | Added gate checklist framework/template and validator checklist checks. |
| DCR-006 | Deepen NASA source specificity encoding. | Framework should capture review, traceability, assurance, and configuration management patterns more explicitly. | REQ-NASA-001, NEED-002, SPEC-001, SPEC-012 | implemented | WP-006 | Added locally authored NASA-inspired technical control map without compliance claim. |
| DCR-007 | Port the local validator from Python to Rust. | User requested Rust implementation for stronger tooling posture. | REQ-RUST-001, REQ-VAL-001, SPEC-007, SPEC-013, IF-005 | implemented | WP-007 | Added std-only Rust crate and removed the Python validator/test implementation. |

## Change-Control Rule

Any change that alters VTRACE stage semantics, ID conventions, template
contracts, skills, source custody, or public adoption guidance gets a `DCR-*`
entry or updates an existing one.
