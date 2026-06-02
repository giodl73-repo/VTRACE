# Validation

## Scope

Repo: VTRACE

## Validation Scenarios

| ID | User / Actor | Scenario | Success Criteria | Evidence | Status |
|---|---|---|---|---|---|
| VAL-001 | VTRACE maintainer | Apply VTRACE to VTRACE itself. | Repo has mission, requirements, spec baseline, architecture, interfaces, design, DCRs, work packages, trace, verification, validation, and review. | `docs/vtrace/` | passed |
| VAL-002 | Future adopter | Use VTRACE DCRs to choose next implementation pulse. | DCRs map to proposed work packages with L0/L1/L2 expectations. | `CHANGE_CONTROL.md`, `WORK_PACKAGES.md` | passed |
| VAL-003 | Future agent | Pick a VTRACE work package and know when to stop. | Work package has parent IDs, affected surfaces, entry/exit criteria, and validation levels. | `WORK_PACKAGES.md` | passed_with_risk |
| VAL-004 | VTRACE maintainer | Run the validator against VTRACE itself. | Validator returns success and reports actionable findings when contracts are broken in tests. | `src/`, `Cargo.toml`, `EVID-013`, `EVID-014` | passed |
| VAL-005 | Existing repo maintainer | Apply VTRACE to a small existing behavior with a target change. | Current and target behavior are separated, implemented, verified, and reviewed. | `examples/existing-repo-migration/` | passed |
| VAL-006 | Evidence reviewer | Adopt an evidence ledger in a target repo. | Trace evidence IDs resolve to ledger rows with complete status. | `templates/adoption/EVIDENCE.md`, validator tests | passed |
| VAL-007 | Gate reviewer | Use gate-specific checklist rows for a review. | Required checklist rows cannot remain pending. | `REVIEW_CHECKLISTS.md`, validator tests | passed |
| VAL-008 | Source custody reviewer | Review deeper NASA-inspired control guidance. | Guidance is derived and pointer-first, without compliance claim. | `docs/framework/nasa-technical-controls.md` | passed |
| VAL-009 | CLI operator / agent operator | Use VTRACE procedurally instead of filling disconnected files. | CLI commands cover init/status/validate/work start/check/close/review/agent brief boundaries, with provider output advisory until accepted. | `docs/framework/cli-orchestrator.md`, `DCR-009`, `WP-009`, `EVID-029` | passed |
| VAL-010 | Integration operator | Coordinate providers, roles, GitHub packets, reports, and pulse records from one work package. | Integration commands produce deterministic dry-run packets, guard live actions behind explicit flags, and keep provider output advisory. | `DCR-010`, `WP-010`, `EVID-049` | passed |
| VAL-011 | Docs owner / adoption guide | Turn a repo's mission, CONOPS scenarios, requirements, specs, interfaces, work packages, and evidence into a user-facing docs package. | VTRACE provides a strategy artifact, template, validator check, and CLI plan command that produce Concepts, How-To, Tutorials, Examples, Traces, decks, and corpus governance without replacing specs. | `COMMUNICATIONS_STRATEGY.md`, `docs/framework/communications-strategy.md`, `EVID-051`, `EVID-052` | passed |

## Open Validation Gaps

| Gap | Disposition |
|---|---|
| Live provider and GitHub execution requires local auth/tooling. | Default to deterministic dry-run packets; require explicit `--live` plus availability checks for external side effects. |
