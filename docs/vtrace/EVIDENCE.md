# Evidence Ledger

## Scope

Repo: VTRACE self-adoption

This ledger records objective evidence for the current VTRACE readiness claim.
Command receipts should be updated when validation is run.

## Evidence Records

| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status |
|---|---|---|---|---|---|
| EVID-001 | inspection | `docs/framework/vtrace-process.md` | V-model process and stage rules exist. | Present and traced by `REQ-001`. | passed |
| EVID-002 | inspection | `templates/adoption/` | Adoption templates exist for required stages. | Present and traced by `REQ-002`. | passed |
| EVID-003 | inspection | `skills/vtrace-assess`, `skills/vtrace-adopt`, `skills/vtrace-gate` | Assessment, adoption, and gate skills exist. | Present and traced by `REQ-003`. | passed |
| EVID-004 | inspection | `docs/framework/specification-baselines.md`, `templates/adoption/SPECIFICATION_BASELINE.md` | Specification baseline guidance and template exist. | Present and traced by `REQ-004`. | passed |
| EVID-005 | inspection | `docs/framework/implementation-management.md`, `templates/adoption/WORK_PACKAGES.md` | Work-package execution rules exist. | Present and traced by `REQ-005`. | passed |
| EVID-006 | inspection | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | DCRs map to proposed work packages. | Present and traced by `REQ-006`. | passed |
| EVID-007 | source custody | `py -m json.tool sources\source-registry.json` | Source registry parses as JSON. | Passed on 2026-05-31. | passed |
| EVID-008 | docs sanity | `git diff --check` | No whitespace errors. | Passed on 2026-05-31. | passed |
| EVID-009 | executable example | `py -m py_compile examples\hello-world\src\hello_world.py` | Hello-world compiles. | Passed on 2026-05-31. | passed |
| EVID-010 | executable example | `py examples\hello-world\src\hello_world.py` | Prints `Hello, VTRACE!`. | Printed `Hello, VTRACE!` on 2026-05-31. | passed |
| EVID-011 | review | `docs/vtrace/REVIEW.md` | Self-adoption gate records decision and findings. | `pass_with_risk`. | passed |
| EVID-012 | role review | `.roles/` against `docs/vtrace/` | Required review lanes are represented and deferred requirements remain trace-visible. | Role-lane fixes recorded in `REVIEW.md` and `TRACE.md` on 2026-05-31. | passed |
| EVID-013 | automated test | `cargo test` | Validator unit tests pass. | 8 tests passed on 2026-05-31. | passed |
| EVID-014 | local command | `cargo run -- .` | VTRACE self-package validates. | Printed `VTRACE validation passed` on 2026-05-31. | passed |
| EVID-015 | inspection / validator | `docs/vtrace/LANGUAGE_PROFILES.md`; `cargo run -- .` | Profiles declare applicability and L0/L1/L2 levels. | Passed on 2026-05-31. | passed |
| EVID-016 | command | `py -m py_compile examples\existing-repo-migration\src\report.py` | Migration example compiles. | Passed on 2026-05-31. | passed |
| EVID-017 | command | `py examples\existing-repo-migration\src\report.py` | Prints `actuator: ready`. | Passed on 2026-05-31. | passed |
| EVID-018 | command | `cargo run -- examples\existing-repo-migration` | Migration example VTRACE package validates. | Passed on 2026-05-31. | passed |
| EVID-019 | automated test / schema inspection | `cargo test`; `schemas/evidence-ledger.schema.json` | Evidence ledger checks are covered and row schema exists. | Passed on 2026-05-31. | passed |
| EVID-020 | automated test / inspection | `docs/vtrace/REVIEW_CHECKLISTS.md`; validator checklist tests | Gate checklist rows close. | Passed on 2026-05-31. | passed |
| EVID-021 | source-custody inspection | `docs/framework/nasa-technical-controls.md`; `docs/vtrace/SOURCE_BASIS.md` | NASA-inspired controls are derived guidance, not compliance claims. | Passed on 2026-05-31. | passed |
| EVID-022 | automated test | `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; `cargo test` | Rust validator is formatted, lint-clean, and tested. | Passed on 2026-05-31. | passed |
| EVID-023 | local command | `cargo run -- .` | Rust validator validates VTRACE self-package. | Printed `VTRACE validation passed` on 2026-05-31. | passed |
| EVID-024 | local command | `cargo run -- examples\existing-repo-migration` | Rust validator validates migration example package. | Printed `VTRACE validation passed` on 2026-05-31. | passed |
| EVID-025 | workflow inspection | `.github/workflows/ci.yml` | CI runs Rust formatting, clippy, tests, self-validation, and migration-example validation. | Workflow file added on 2026-05-31. | passed |
| EVID-026 | local command parity | `git diff --check`; `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; `cargo test`; `cargo run -- .`; `cargo run -- examples\existing-repo-migration` | Local validation commands mirror CI command intent. | Passed on 2026-05-31. | passed |
| EVID-027 | remote CI run | <https://github.com/giodl73-repo/VTRACE/actions/runs/26728521454> | GitHub Actions CI completes successfully for the workflow-introducing commit. | Run `26728521454` completed with conclusion `success` for commit `15b73be0439c3daf7fadde0f181e33894bcd3a22` on 2026-06-01 UTC. | passed |
| EVID-028 | design inspection | `docs/framework/cli-orchestrator.md`, `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | CLI orchestrator DCR is traceable to requirements, specs, work package, and validation scenario. | `DCR-009` / `WP-009` added with deterministic CLI and advisory-provider boundaries. | passed |
| EVID-029 | CLI command receipts | `cargo fmt --check`; `cargo clippy --all-targets -- -D warnings`; `cargo test`; `cargo run -- validate .`; `cargo run -- status .`; `cargo run -- work start WP-009 .`; `cargo run -- work check WP-009 .`; `cargo run -- roles review WP-009 .`; `cargo run -- agent brief WP-009 .`; `cargo run -- init $env:TEMP\vtrace-cli-init-codex` | First CLI orchestration slice works locally and preserves deterministic validation. | Commands passed on 2026-06-01; initial `C:\tmp` init target failed because the directory root was unavailable, then temp-directory init passed. | passed |
| EVID-030 | CLI integration tests | `tests/cli.rs`; `cargo test`; `cargo clippy --all-targets -- -D warnings` | CLI commands are regression-tested for validate, status, work start, roles review, agent brief, and init overwrite protection. | 8 library tests and 11 CLI integration tests passed on 2026-06-01; clippy passed with warnings denied. | passed |
| EVID-031 | CLI evidence receipt | `cargo run -- evidence receipt WP-009 .`; `tests/cli.rs` | CLI can generate a structured work-package evidence receipt draft without mutating canonical artifacts. | Command passed on 2026-06-01 with validator findings: 0; CLI integration tests cover the receipt output. | passed |
| EVID-032 | CLI plan command | `cargo run -- plan .`; `tests/cli.rs` | CLI can summarize next procedural action from validator and work-package state. | Command passed on 2026-06-01 and reported no open self work packages; CLI integration tests cover the plan output. | passed |
| EVID-033 | CLI worktree plan command | `cargo run -- worktree plan WP-009 .`; `tests/cli.rs` | CLI can derive an isolated worktree branch/path/command for a work package without mutating the repo. | Command passed on 2026-06-01 and printed branch `vtrace/wp-009`; CLI integration tests cover the worktree plan output. | passed |
| EVID-034 | CLI worktree create command | `tests/cli.rs`; `cargo test` | CLI can create an isolated worktree for a work package from a clean git repo and refuse unsafe targets through guard checks. | Integration test created a temporary git repo, committed a minimal work package, ran `vtrace worktree create WP-001 <repo> <target>`, verified the worktree files, and removed the worktree. | passed |
| EVID-035 | Worktree ownership record | `tests/cli.rs`; `cargo test` | Created worktrees receive a local `.vtrace/worktree.md` record with WP, branch, source repo, worktree path, and closeout commands. | Integration test verified the generated record contains `Work package: WP-001` and `Closeout commands:`. | passed |
| EVID-036 | CLI worktree status command | `tests/cli.rs`; `cargo test` | CLI can list git worktrees and identify whether a VTRACE ownership record is present. | Integration test creates a temporary worktree and verifies `vtrace worktree status <repo>` reports `record: present`. | passed |
| EVID-037 | CLI worktree remove command | `tests/cli.rs`; `cargo test` | CLI can remove a VTRACE-owned worktree while refusing unowned targets unless forced. | Integration test removes the temporary worktree through `vtrace worktree remove <target>` and verifies the target no longer exists. | passed |
| EVID-038 | Worktree agent brief record | `tests/cli.rs`; `cargo test` | Created worktrees receive a local `.vtrace/agent-brief.md` handoff brief with WP context and stop conditions. | Integration test verified the generated brief contains `# VTRACE Agent Brief: WP-001` and `Stop conditions:`. | passed |

## Evidence Rules

- Evidence must point to a file, command, CI run, review record, artifact, or
  explicit inspection target.
- Evidence for public claims must be reproducible or reviewable.
- A future validator may generate this ledger, but this Markdown file is the
  current source of truth.
