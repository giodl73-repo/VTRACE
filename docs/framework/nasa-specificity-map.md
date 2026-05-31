# NASA Specificity Map

VTRACE is not a NASA compliance framework. It translates public NASA systems
engineering, software engineering, assurance, and review practices into a
software-repo operating model with enough specificity to stop random slicing.

NASA projects use tailoring. The practical lesson for VTRACE is not "create
every possible document." The lesson is: define the controlled technical
products, baseline them, assign responsible roles, trace them, verify them,
review them, and manage changes.

## NASA Pattern To VTRACE Control

| NASA Pattern | VTRACE Control | Required Repo Evidence |
|---|---|---|
| Stakeholder expectations and CONOPS | `MISSION.md`, `CONOPS.md` | Users, scenarios, constraints, success criteria, validation scenarios. |
| Technical requirements definition | `REQUIREMENTS.md` | Clear `REQ-*` statements with rationale, owner, priority, verification method, and parent need. |
| Specification generation | `SPECIFICATION_BASELINE.md` plus derived specs when needed | `SPEC-*` rows for accepted behavior, public contracts, constraints, assumptions, expected results, and acceptance basis. |
| Logical decomposition | `ARCHITECTURE.md`, `PACKAGE_BOUNDARIES.md` | Components, crate/package/module ownership, dependency direction, allocation of `REQ-*` and `SPEC-*`. |
| Design solution definition | `DESIGN.md`, `CODE_RIGOR.md` | Design decisions, invariants, algorithms, safety/security considerations, coding constraints. |
| Interface control | `INTERFACES.md` or a derived interface spec | `IF-*` contracts, compatibility rules, fixtures, schemas, versioning, owners. |
| Product realization | `IMPLEMENTATION_PLAN.md`, `WORK_PACKAGES.md` | Work packages with parent IDs, entry/exit criteria, branch/worktree rules, L0/L1/L2 checks. |
| Technical assessment | `STAGE_EXECUTION.md`, `REVIEW.md` | Stage gate decisions, review findings, metrics, evidence pointers. |
| Verification and validation | `VERIFICATION.md`, `VALIDATION.md` | Procedures, commands, expected results, evidence, user acceptance scenarios. |
| Configuration and change control | `CHANGE_CONTROL.md` | Change IDs, affected controlled items, reviewer, decision, trace updates. |
| Technical data management | `TRACE.md`, evidence ledger, source custody | Trace rows, artifacts, command receipts, source provenance, retained reports. |
| Software assurance and safety | Role lanes and `.roles/` | Assurance, safety/risk, security/privacy, source-custody, and V&V review decisions. |

## Required Specificity For Existing Repos

Existing repos usually already have behavior before VTRACE arrives. Start by
baselining what exists, not by writing a fantasy target state.

Every VTRACE adoption should classify behavior into these states:

- `current`: observed in docs, tests, examples, releases, CLI/API behavior, or
  downstream use,
- `target`: accepted for this scope but not yet implemented,
- `deprecated`: still present but planned for removal or compatibility control,
- `unknown`: needs discovery before implementation can start.

Every controlled item should identify:

- ID,
- source,
- owner,
- risk,
- verification method,
- validation method when user-facing,
- change-control trigger,
- status.

## What Must Be Specific Before Coding

For non-trivial implementation, do not start work packages until these are
known or explicitly deferred:

- which `REQ-*` items are in scope,
- which `SPEC-*` behavior or contracts are controlled,
- which `IF-*` interfaces may change,
- which `PKG-*` package/crate/module boundaries may be touched,
- which `DES-*` decisions constrain implementation,
- which `CR-*` code-rigor rules apply,
- which L0/L1/L2 commands prove the work,
- which review lanes are required,
- which pulse or wave records will hold execution evidence.

## Review Depth

Use review depth proportional to risk:

| Scope | Minimum Gate | Required Review Lanes |
|---|---|---|
| Documentation-only clarification | Specification or readiness review | Systems engineering, traceability. |
| Private implementation with no contract change | Implementation readiness and work-package close | Systems engineering, V&V, software assurance as needed. |
| Public API/CLI/schema/config change | Specification, design, integration, readiness | Systems engineering, traceability, V&V, software assurance, configuration/change control. |
| Security/privacy-sensitive change | Same as public contract change | Add security/privacy. |
| Safety/mission-impacting change | Full staged gates | Add safety/risk and independent review when possible. |
| Source/standard encoding change | Specification/readiness | Add source custody and traceability. |

## NASA Source Pointers

This map is derived from public NASA source categories already tracked in
`sources/source-registry.json`: the NASA Systems Engineering Handbook, its
appendix material, NPR 7123.1, NPR 7120.5, the NASA Software Engineering
Handbook, NASA-STD-8739.8B, and NASA cybersecurity policy pointers.
