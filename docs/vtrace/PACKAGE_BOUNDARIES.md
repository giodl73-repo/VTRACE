# Package Boundaries

## Scope

Repo: VTRACE

## Boundary Inventory

| ID | Boundary Unit | Language / Toolchain | Owner | Responsibility | Public Interfaces | Downstream Consumers |
|---|---|---|---|---|---|---|
| PKG-001 | `docs/framework/` | docs | VTRACE | Define framework guidance, process rules, review rules, staged execution, source custody, and implementation management. | IF-003 | Target repos, agents, maintainers. |
| PKG-002 | `templates/adoption/` | docs | VTRACE | Provide copy/adapt templates for target repo `docs/vtrace/` packages. | IF-002, IF-003 | Target repos, agents. |
| PKG-003 | `skills/` | Codex skill Markdown | VTRACE | Provide agent workflows for assessment, adoption, gate review, and V-model rigor. | IF-001, IF-003 | Agents. |
| PKG-004 | `.roles/` | role Markdown | VTRACE | Provide review lenses for systems, traceability, V&V, assurance, custody, security, safety, and adoption usability. | IF-003 | Review agents and maintainers. |
| PKG-005 | `examples/` | docs / Python | VTRACE | Demonstrate VTRACE adoption paths and scenario tests. | IF-003 | Future adopters. |
| PKG-006 | `sources/` and `schemas/` | JSON / schema | VTRACE | Track source custody, rights posture, and encoding contracts. | IF-004 | Framework docs, source custody reviewers. |
| PKG-007 | `context/waves/` | docs | VTRACE | Record VTRACE repo execution history and pulse outcomes. | IF-003 | Future agents and maintainers. |
| PKG-008 | `docs/vtrace/` | docs | VTRACE | Prove VTRACE with VTRACE and control DCRs. | IF-003 | VTRACE maintainers and adopters. |
| PKG-009 | `src/`, `Cargo.toml`, `Cargo.lock` | Rust std | VTRACE | Validate VTRACE artifact contracts without network or third-party dependencies. | IF-005 | Maintainers, agents, target repos. |
| PKG-010 | `.github/workflows/` | GitHub Actions YAML | VTRACE | Run repeatable validation for VTRACE pushes and pull requests. | IF-006 | Maintainers, agents, target repos. |

## Dependency Direction

| From | To | Allowed? | Rationale | Verification |
|---|---|---|---|---|
| `docs/vtrace/` | all repo boundaries | yes | Self-trace references the complete VTRACE proof surface. | trace inspection |
| `skills/` | `docs/framework/` | yes | Skills should follow framework behavior. | skill review |
| `templates/adoption/` | `docs/framework/` | yes | Templates implement framework contracts. | template review |
| `docs/framework/` | `templates/adoption/` | limited | Framework may name required artifacts but should avoid copying template details. | docs review |
| `examples/` | `templates/adoption/`, `docs/framework/` | yes | Examples demonstrate adoption. | scenario review |
| `sources/` | external public/citation sources | yes | Source custody points to references. | JSON/source review |
| `src/` | `docs/vtrace/`, `templates/adoption/` | yes | Validator checks current artifact contracts. | cargo test / validator run |
| `.github/workflows/` | `src/`, `docs/vtrace/`, `examples/` | yes | CI executes the same validator commands recorded in evidence. | workflow inspection |

## Boundary Rules

| Boundary ID | Allowed Changes | Forbidden Changes | Change-Control Trigger |
|---|---|---|---|
| PKG-001 | Add/refine framework guidance with trace updates. | Claim NASA endorsement or compliance without source/legal basis. | Any stage semantic change. |
| PKG-002 | Add fields/templates that map to VTRACE stages. | Rename public templates without migration notes. | Any required artifact or field change. |
| PKG-003 | Refine workflows and stop conditions. | Make skills bypass required VTRACE gates. | Any skill behavior change. |
| PKG-004 | Add/revise review lenses. | Remove required role lanes without review. | Any role lane change. |
| PKG-005 | Add scenarios and realistic examples. | Let examples contradict templates or process. | Any public example claim. |
| PKG-006 | Add metadata and schemas. | Commit restricted standards text without custody decision. | Any source rights or schema change. |
| PKG-008 | Update self-trace for VTRACE changes. | Leave DCRs/work packages stale after process changes. | Any VTRACE behavior claim change. |
| PKG-009 | Add deterministic std-only Rust checks and tests. | Add network or third-party dependency requirements for the core validator. | Any validation contract or CLI behavior change. |
| PKG-010 | Add validation jobs that mirror local evidence commands. | Add deploy or release permissions without a DCR and security review. | Any CI permission or validation-scope change. |

## Language Tailoring

| Boundary ID | Code Rigor Profile | L0 | L1 | L2 |
|---|---|---|---|---|
| PKG-001 / PKG-002 / PKG-004 / PKG-007 / PKG-008 | docs-only | `git diff --check` | trace/review inspection | adoption review |
| PKG-003 | skill Markdown | `git diff --check` | frontmatter/workflow inspection | skill execution on example |
| PKG-005 | docs + Python | `git diff --check`; Python compile/run when touched | scenario inspection | realistic adoption scenario |
| PKG-006 | JSON/schema | JSON parse | schema/source custody inspection | source-custody review |
| PKG-009 | Rust std | `cargo fmt --check` | `cargo test`; `cargo run -- .` | `cargo run -- examples\existing-repo-migration` |
| PKG-010 | GitHub Actions | YAML inspection | workflow command parity with `VERIFICATION.md` | first remote CI run after push |

## Open Boundary Questions

- Should validators later be packaged as a CLI, GitHub Action, or reusable skill helper?
- Should schemas become authoritative for all VTRACE templates or only machine-checkable subsets?
