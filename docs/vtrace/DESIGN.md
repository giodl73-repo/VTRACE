# Design

## Scope

Repo: VTRACE

## Design Decisions

| ID | Decision | Rationale | Related IDs | Status |
|---|---|---|---|---|
| DES-001 | VTRACE uses Markdown artifacts before executable tooling. | Contracts need to stabilize before validators become useful. | REQ-001, REQ-002, DCR-001 | accepted |
| DES-002 | Self-adoption links to existing framework docs instead of duplicating them. | Keeps VTRACE maintainable and avoids conflicting process copies. | SPEC-001..SPEC-005 | accepted |
| DES-003 | Missing capabilities are represented as DCRs and future WPs. | Converts gaps into controlled implementation candidates. | REQ-006, DCR-001..DCR-006 | accepted |
| DES-004 | VTRACE treats NASA references as source categories, not compliance claims. | Keeps guidance honest and portable to ordinary software repos. | NEED-002 | accepted |
| DES-005 | The CLI orchestrator extends the validator without making generated output canonical. | Maintainers need procedural help, but Markdown trace/evidence artifacts remain the source of truth. | REQ-CLI-001, SPEC-015, IF-007, DCR-009 | accepted |
| DES-006 | LLM providers and agents are optional bounded adapters. | AI can draft, review, and summarize, but closure requires deterministic evidence and explicit artifact updates. | REQ-AI-001, SPEC-016, IF-008, DCR-009 | accepted |
