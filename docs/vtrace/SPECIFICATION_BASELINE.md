# Specification Baseline

## Scope

Repo: VTRACE

Baseline type: mixed

Baseline date: 2026-05-31

## Controlled Specification Items

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-001 | product | current | `docs/framework/vtrace-process.md` defines the V shape, stage deliverables, stage rules, adoption flow, and definition of done. | inspection | self-adoption scenario | VTRACE | medium | accepted |
| SPEC-002 | REQ-002 | product | current | `templates/adoption/` contains stage templates including mission, requirements, specification baseline, architecture, implementation plan, work packages, verification, validation, trace, and review. | inspection | self-adoption scenario | VTRACE | medium | accepted |
| SPEC-003 | REQ-003 | product | current | `skills/vtrace-assess`, `skills/vtrace-adopt`, and `skills/vtrace-gate` guide assessment, adoption, and review gates. | inspection | agent scenario | VTRACE | medium | accepted |
| SPEC-004 | REQ-004 | product | current | Specification baselines classify current, target, deprecated, and unknown behavior before non-trivial implementation planning. | inspection | existing-repo scenario | VTRACE | medium | accepted |
| SPEC-005 | REQ-005 | product | current | Work packages require parent IDs, V closure rows, L0/L1/L2 validation levels, review lanes, Git execution, and pulse linkage. | inspection | work-package scenario | VTRACE | medium | accepted |
| SPEC-006 | REQ-006 | roadmap | target | Remaining gaps are recorded as `DCR-*` change requests and mapped to future work packages. | inspection | roadmap review | VTRACE | low | accepted |
| SPEC-007 | REQ-VAL-001 | software | current | The Rust `vtrace` validator validates required VTRACE artifacts, requirement/spec trace visibility, evidence pointers, work-package shape, language profiles, review checklists, and required review-lane closure. | automated test / local command | self-adoption scenario | VTRACE | medium | accepted |
| SPEC-008 | REQ-PROFILE-001 | product | current | `docs/framework/language-profiles.md`, `templates/adoption/LANGUAGE_PROFILES.md`, and `docs/vtrace/LANGUAGE_PROFILES.md` define profile applicability and L0/L1/L2 expectations. | inspection / validator | profile scenario | VTRACE | medium | accepted |
| SPEC-009 | REQ-EXAMPLE-001 | product | current | `examples/existing-repo-migration/` demonstrates current/target spec baselining and one closed work package. | example command / validator | existing-repo scenario | VTRACE | medium | accepted |
| SPEC-010 | REQ-EVIDENCE-001 | product | current | `templates/adoption/EVIDENCE.md` and validator evidence checks define reusable evidence ledger behavior. | validator tests | evidence scenario | VTRACE | medium | accepted |
| SPEC-011 | REQ-GATE-001 | product | current | `docs/framework/gate-checklists.md`, `templates/adoption/REVIEW_CHECKLISTS.md`, and `docs/vtrace/REVIEW_CHECKLISTS.md` define gate-specific checklist closure. | inspection / validator | gate scenario | VTRACE | medium | accepted |
| SPEC-012 | REQ-NASA-001 | product | current | `docs/framework/nasa-technical-controls.md` encodes NASA-inspired technical controls as locally authored derived guidance. | source-custody inspection | source review | VTRACE | low | accepted |
| SPEC-013 | REQ-RUST-001 | software | current | The validator is provided as a std-only Rust crate with `vtrace` binary, unit tests, and no runtime network or third-party dependency requirement. | cargo test / local command | validator packaging scenario | VTRACE | medium | accepted |
| SPEC-014 | REQ-CI-001 | workflow | current | `.github/workflows/ci.yml` runs Cargo formatting, clippy, tests, self-validation, and migration-example validation on push and pull request. | workflow inspection / local command parity | CI validation scenario | VTRACE | medium | accepted |
| SPEC-015 | REQ-CLI-001 | software | current | `docs/framework/cli-orchestrator.md` defines the CLI command surface and `src/` implements the first local commands for `init`, `status`, `validate`, `work start/check/close`, role-review preparation, and agent briefs. | design inspection / CLI commands | CLI operator scenario | VTRACE | medium | accepted |
| SPEC-016 | REQ-AI-001 | software / assurance | current | Provider and agent integrations remain optional adapter layers; generated output is advisory until accepted into canonical VTRACE artifacts with trace IDs, evidence, and review status. | design inspection / role review | provider-assisted adoption scenario | VTRACE | high | accepted |
| SPEC-017 | REQ-INTEGRATION-001 / REQ-AI-001 / REQ-CLI-001 | software / assurance | current | The CLI exposes provider list/check/draft/review, roles run, adoption report, GitHub issue/PR dry-run/live helpers, and pulse sync commands with explicit live flags and advisory output boundaries. | CLI integration tests / local commands | integration operator scenario | VTRACE | high | accepted |
| SPEC-018 | REQ-COMMS-001 / REQ-CLI-001 | product / software | current | `docs/framework/communications-strategy.md`, `templates/adoption/COMMUNICATIONS_STRATEGY.md`, `docs/vtrace/COMMUNICATIONS_STRATEGY.md`, the validator, and `vtrace comms plan` define source-to-user-docs orchestration for target repos. | validator / CLI integration tests / inspection | communications adoption scenario | VTRACE | medium | accepted |
| SPEC-019 | REQ-SPEC-MODEL-001 | product | current | `templates/adoption/SPEC_MODEL.md` and `docs/vtrace/SPEC_MODEL.md` define required spec classes, spec shape, scenario coupling, descriptor impact, and promotion rules for deep implementation input. | inspection | deep-spec adoption scenario | VTRACE | medium | accepted |
| SPEC-020 | REQ-CONTRACT-BOUNDARY-001 | product | current | `templates/adoption/CONTRACT_BOUNDARIES.md` and `docs/vtrace/CONTRACT_BOUNDARIES.md` define durable contract-boundary classes and package closeout requirements. | inspection | contract-boundary adoption scenario | VTRACE | medium | accepted |
| SPEC-021 | REQ-SCENARIO-001 | product | current | `templates/adoption/SCENARIO_MODEL.md` and `docs/vtrace/SCENARIO_MODEL.md` define scenario package shape, findings rules, fixture candidates, and validation coupling. | inspection | scenario adoption scenario | VTRACE | medium | accepted |
| SPEC-022 | REQ-DIAGNOSTIC-MODEL-001 | product | current | `templates/adoption/DIAGNOSTIC_MODEL.md` and `docs/vtrace/DIAGNOSTIC_MODEL.md` define diagnostic contract shape, severity, location, redaction/custody, diagnostic families, fixture expectations, rendering impact, and allocation rules. | inspection | diagnostic adoption scenario | VTRACE | medium | accepted |
| SPEC-023 | REQ-PROBLEM-SPACE-001 | product | current | `templates/adoption/PROBLEM_SPACE_MAP.md` and `docs/vtrace/PROBLEM_SPACE_MAP.md` define world regions, traversal order, and cross-region risks for large-domain adoption. | inspection | problem-space adoption scenario | VTRACE | medium | accepted |
| SPEC-024 | REQ-DOMAIN-BACKLOG-001 | product | current | `templates/adoption/DOMAIN_BACKLOG.md` and `docs/vtrace/DOMAIN_BACKLOG.md` define backlog item types, fields, dispositions, and exit paths for domain discoveries. | inspection | backlog adoption scenario | VTRACE | medium | accepted |
| SPEC-025 | REQ-FIXTURE-MODEL-001 | product | current | `templates/adoption/FIXTURE_MODEL.md` and `docs/vtrace/FIXTURE_MODEL.md` define fixture classes, required fixture shape, and scenario-to-fixture promotion rules. | inspection | fixture adoption scenario | VTRACE | medium | accepted |
| SPEC-026 | REQ-RESEARCH-001 | product | current | `templates/adoption/RESEARCH_PLAN.md` and `docs/vtrace/RESEARCH_PLAN.md` define research tracks, item types, required research shape, and research-to-requirement promotion rules. | inspection | research-plan adoption scenario | VTRACE | medium | accepted |
| SPEC-027 | REQ-PRODUCT-BOUNDARY-001 | product / assurance | current | VTRACE guidance, adoption templates, skills, CLI init output, plan output, and agent briefs state that VTRACE/work-package/review/readiness/proof/validation/fixture/package-status concepts are not target-product features unless explicitly restated as customer-facing behavior. | inspection / CLI tests | product-boundary adoption scenario | VTRACE | high | accepted |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-001 | covered | Framework process exists. |
| REQ-002 | SPEC-002 | covered | Templates exist. |
| REQ-003 | SPEC-003 | covered | Skills exist. |
| REQ-004 | SPEC-004 | covered | Spec baseline process exists. |
| REQ-005 | SPEC-005 | covered | Implementation management exists. |
| REQ-006 | SPEC-006 | covered | DCRs defined in change control. |
| REQ-VAL-001 | SPEC-007 | covered | Lightweight validator exists. |
| REQ-PROFILE-001 | SPEC-008 | covered | Language/package profiles exist. |
| REQ-EXAMPLE-001 | SPEC-009 | covered | Realistic migration example exists. |
| REQ-EVIDENCE-001 | SPEC-010 | covered | Evidence template and validator checks exist. |
| REQ-GATE-001 | SPEC-011 | covered | Gate checklist guidance and template exist. |
| REQ-NASA-001 | SPEC-012 | covered | NASA-inspired technical control guidance exists. |
| REQ-RUST-001 | SPEC-013 | covered | Rust validator crate exists. |
| REQ-CI-001 | SPEC-014 | covered | GitHub Actions workflow exists. |
| REQ-CLI-001 | SPEC-015 | covered | First CLI orchestrator slice is implemented. |
| REQ-AI-001 | SPEC-016 / SPEC-017 | covered | Provider and agent boundaries are implemented as advisory command surfaces with explicit live guards. |
| REQ-INTEGRATION-001 | SPEC-017 | covered | Later-boundary integration commands are implemented with guarded live paths and deterministic dry-run tests. |
| REQ-COMMS-001 | SPEC-018 | covered | Communications strategy artifacts and CLI planning command distinguish user docs from controlled specs. |
| REQ-SPEC-MODEL-001 | SPEC-019 | covered | Deep spec model templates and self-trace copies exist. |
| REQ-CONTRACT-BOUNDARY-001 | SPEC-020 | covered | Contract-boundary templates and self-trace copies exist. |
| REQ-SCENARIO-001 | SPEC-021 | covered | Scenario model templates and self-trace copies exist. |
| REQ-DIAGNOSTIC-MODEL-001 | SPEC-022 | covered | Diagnostic model templates and self-trace copies exist. |
| REQ-PROBLEM-SPACE-001 | SPEC-023 | covered | Problem-space map templates and self-trace copies exist. |
| REQ-DOMAIN-BACKLOG-001 | SPEC-024 | covered | Domain backlog templates and self-trace copies exist. |
| REQ-FIXTURE-MODEL-001 | SPEC-025 | covered | Fixture model templates and self-trace copies exist. |
| REQ-RESEARCH-001 | SPEC-026 | covered | Research plan templates and self-trace copies exist. |
| REQ-PRODUCT-BOUNDARY-001 | SPEC-027 | covered | Product/process/verification/VTRACE-only boundaries are explicit in core guidance, templates, skills, CLI init output, plan output, and agent briefs. |

## Specification Gate

Decision: pass_with_risk

Rationale: VTRACE now has a self-baseline, local validator, profiles, evidence
ledger template, gate checklists, realistic migration example, and deeper
source-grounded controls.
