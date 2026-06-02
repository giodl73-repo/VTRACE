# Requirements

## Scope

Repo: VTRACE

## Requirement Table

| ID | Requirement | Parent Need / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | VTRACE shall define a source-grounded V-model process for software repos. | NEED-001 / NEED-002 | Establishes the operating model. | must | VTRACE | inspection | accepted |
| REQ-002 | VTRACE shall provide adoption templates for each required VTRACE stage. | NEED-001 / CON-001 | Enables repeatable repo adoption. | must | VTRACE | inspection | accepted |
| REQ-003 | VTRACE shall provide skills for assessment, adoption, and gate review. | NEED-003 / CON-002 / CON-003 | Enables agents to apply the process. | must | VTRACE | inspection | accepted |
| REQ-004 | VTRACE shall require specification baselines before non-trivial implementation planning. | NEED-001 / CON-001 | Prevents random implementation slices. | must | VTRACE | inspection | accepted |
| REQ-005 | VTRACE shall map implementation work to work packages with parent IDs, V closure, L0/L1/L2 checks, and review lanes. | NEED-001 / CON-002 | Converts files into procedural execution. | must | VTRACE | inspection | accepted |
| REQ-006 | VTRACE shall define future change requests for missing validators, language profiles, evidence ledgers, gate checklists, and realistic adoption scenarios. | NEED-001 / NEED-003 | Makes remaining gaps explicit. | should | VTRACE | inspection | accepted |
| REQ-VAL-001 | VTRACE shall provide a lightweight local validator for repo-local `docs/vtrace/` packages. | NEED-001 / NEED-003 | Enforces the current artifact contracts without external dependencies. | must | VTRACE | automated test / local command | accepted |
| REQ-PROFILE-001 | VTRACE shall define language/package validation profiles for common repo types. | NEED-001 / NEED-003 | Lets target repos choose concrete L0/L1/L2 commands. | must | VTRACE | inspection / validator | accepted |
| REQ-EXAMPLE-001 | VTRACE shall include a realistic existing-repo migration example with current and target behavior. | NEED-001 / CON-001 | Proves VTRACE retrofit adoption beyond hello-world. | must | VTRACE | example command / validator | accepted |
| REQ-EVIDENCE-001 | VTRACE shall provide reusable evidence ledger artifacts and validation checks. | NEED-001 / NEED-003 | Makes objective evidence durable and traceable. | must | VTRACE | validator tests | accepted |
| REQ-GATE-001 | VTRACE shall provide gate-specific review checklist artifacts. | NEED-001 / CON-003 | Makes review gates repeatable. | must | VTRACE | inspection / validator | accepted |
| REQ-NASA-001 | VTRACE shall encode deeper NASA-inspired technical controls as derived guidance. | NEED-002 | Preserves source-grounded rigor without compliance overclaiming. | should | VTRACE | source-custody inspection | accepted |
| REQ-RUST-001 | VTRACE shall implement the local validator as a Rust CLI. | NEED-001 / NEED-003 | Keeps validator implementation aligned with portfolio language rigor and supports future packaging. | must | VTRACE | cargo test / local command | accepted |
| REQ-CI-001 | VTRACE shall run the Rust validation path in CI on push and pull request. | NEED-001 / NEED-003 | Makes validator evidence repeatable outside a local workstation. | should | VTRACE | workflow inspection / local command parity | accepted |
| REQ-CLI-001 | VTRACE shall define a CLI orchestrator contract for initialization, status, validation, work-package execution, evidence recording, review preparation, and closeout. | NEED-001 / NEED-003 / CON-004 | Converts VTRACE from static artifacts into a procedural adoption runner. | must | VTRACE | design inspection / future CLI tests | accepted |
| REQ-AI-001 | VTRACE shall define optional LLM provider and agent/worktree boundaries that keep generated output advisory until accepted into controlled artifacts. | NEED-003 / CON-002 / CON-004 | Allows AI assistance without weakening traceability, evidence, source custody, or human/agent review gates. | should | VTRACE | design inspection / role review | accepted |
| REQ-INTEGRATION-001 | VTRACE shall provide explicit provider, role, GitHub, adoption-report, and pulse-sync integration commands that preserve canonical artifact control. | NEED-003 / CON-002 / CON-004 | Completes later-boundary orchestration while keeping live actions guarded and advisory output non-canonical. | should | VTRACE | CLI integration tests / local commands | accepted |
| REQ-COMMS-001 | VTRACE shall define how repo-local user-facing docs are derived from mission, CONOPS, requirements, specs, interfaces, work packages, validation, and evidence without replacing controlled specifications. | NEED-001 / NEED-003 / CON-004 | Helps adopters build Concepts, How-To, Tutorials, Examples, Traces, decks, and corpus governance from the VTRACE source chain. | should | VTRACE | validator / CLI command / inspection | accepted |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| none | n/a | n/a |
