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
| DCR-008 | Add CI for the Rust validator path. | Validator evidence should run outside local agent sessions. | REQ-CI-001, REQ-RUST-001, SPEC-014, IF-006 | implemented | WP-008 | Added GitHub Actions workflow for formatting, clippy, tests, self-validation, and example validation. |
| DCR-009 | Design and implement the first VTRACE CLI orchestrator slice with optional provider/agent boundaries. | Users need help running the VTRACE process, not just validating finished files. | REQ-CLI-001, REQ-AI-001, SPEC-015, SPEC-016, IF-007, IF-008 | implemented | WP-009 | CLI guides init/status/validate/work-package execution, role-review preparation, and agent briefs; provider integrations remain optional advisory helpers. |
| DCR-010 | Implement later-boundary integrations for providers, roles, reports, GitHub helpers, and pulse sync. | Users need VTRACE to coordinate agent/provider/review execution while preserving canonical artifact control. | REQ-INTEGRATION-001, REQ-AI-001, REQ-CLI-001, SPEC-017 | implemented | WP-010 | Added provider-neutral Codex/Claude/Copilot command surfaces, advisory role packets, adoption reports, guarded GitHub helpers, and pulse sync dry-run/live boundaries. |
| DCR-011 | Add communications strategy orchestration for user-facing docs. | Adopters need VTRACE to derive Concepts, How-To, Tutorials, Examples, Traces, decks, and corpus governance from controlled artifacts without confusing docs with specs. | REQ-COMMS-001, REQ-CLI-001, SPEC-018, IF-009 | implemented | WP-011 | Added framework guidance, adoption template, self strategy, validator checks, and `vtrace comms plan`. |
| DCR-012 | Build the VTRACE user-facing docs package. | VTRACE should prove the communications strategy by applying it to its own docs. | REQ-COMMS-001, SPEC-018, VAL-011, COMMS-README-001, COMMS-CONCEPTS-001, COMMS-CLI-001, COMMS-RUNE-PATTERN-001 | implemented | WP-012 | Added docs map, concepts, how-to guides, tutorials, examples, traces, deck outline, and corpus governance. |
| DCR-013 | Add deep spec model, contract-boundary, and scenario model templates to the stage order. | Complex platform adoptions need reusable controls for spec depth, durable interfaces, and scenario findings before implementation. | REQ-SPEC-MODEL-001, REQ-CONTRACT-BOUNDARY-001, REQ-SCENARIO-001, SPEC-019, SPEC-020, SPEC-021, VAL-012 | implemented | WP-013 | Added adoption templates, self-trace copies, stage-order placement, and trace/evidence rows. |
| DCR-014 | Add diagnostic model templates to the stage order. | Complex adoptions need reusable controls for durable diagnostics, locations, severities, fixtures, rendering, and allocation before implementation. | REQ-DIAGNOSTIC-MODEL-001, SPEC-022, VAL-013 | implemented | WP-014 | Added adoption template, self-trace copy, stage-order placement, and trace/evidence rows. |
| DCR-015 | Add problem-space map, domain backlog, and fixture model templates to the stage order. | Large-domain adoptions need reusable controls for world mapping, backlog discovery, and fixture promotion. | REQ-PROBLEM-SPACE-001, REQ-DOMAIN-BACKLOG-001, REQ-FIXTURE-MODEL-001, SPEC-023, SPEC-024, SPEC-025, VAL-014 | implemented | WP-015 | Added adoption templates, self-trace copies, stage-order placement, and trace/evidence rows. |
| DCR-016 | Add research-plan templates to the stage order. | Source-heavy and large-domain adoptions need a generic path for research tracks and papers to drive requirements before implementation. | REQ-RESEARCH-001, SPEC-026, VAL-015 | implemented | WP-016 | Added adoption template, self-trace copy, S1 placement, and trace/evidence rows. |
| DCR-017 | Add a hard product-boundary rule to stop VTRACE process terms from becoming target-product UX. | CRAFT adoption showed agents building process commands such as work-package/proof/readiness surfaces instead of product features. | REQ-PRODUCT-BOUNDARY-001, SPEC-027, VAL-016 | implemented | WP-017 | Added boundary guidance to README, process docs, adoption templates, skills, CLI init output, plan output, and agent briefs. |

## Change-Control Rule

Any change that alters VTRACE stage semantics, ID conventions, template
contracts, skills, source custody, or public adoption guidance gets a `DCR-*`
entry or updates an existing one.
