# Implementation Plan

## Scope

Repo: VTRACE

Implementation baseline: self-adoption and DCR planning

## Baseline Inputs

| Artifact | Status | Notes |
|---|---|---|
| `MISSION.md` | accepted | Self-adoption baseline. |
| `CONOPS.md` | accepted | Maintainer, agent, reviewer scenarios. |
| `REQUIREMENTS.md` | accepted | Core VTRACE requirements and deferred tooling/profile requirements. |
| `SPECIFICATION_BASELINE.md` | accepted | Current and target specs. |
| `ARCHITECTURE.md` | accepted | Repo component allocation. |
| `INTERFACES.md` | accepted | Skill, template, ID, and source registry interfaces. |
| `DESIGN.md` | accepted | Markdown-first, source-grounded, DCR-driven design. |
| `CODE_RIGOR.md` | accepted | Applies to future validators and profiles. |
| `CHANGE_CONTROL.md` | accepted | DCR backlog. |
| `VERIFICATION.md` | accepted | Current validation commands. |
| `VALIDATION.md` | accepted | Self-adoption validation scenario. |

## Source-To-Work-Package Mapping

| Source IDs | Work Package | Disposition | Notes |
|---|---|---|---|
| DCR-001 / REQ-VAL-001 / CR-001..CR-003 | WP-001 | implement later | Validator candidate pulse. |
| DCR-002 / REQ-PROFILE-001 / CR-004 | WP-002 | implement later | Profiles should follow real adoption needs. |
| DCR-003 / CON-001 / SPEC-004 / SPEC-005 | WP-003 | implement later | Realistic migration example. |
| DCR-004 / IF-003 / SPEC-005 | WP-004 | implement later | Evidence ledger. |
| DCR-005 / SPEC-001 / SPEC-005 | WP-005 | implement later | Gate checklists. |
| DCR-006 / NEED-002 / SPEC-001 | WP-006 | implement later | NASA specificity encoding. |

## Branch / Change Control

Branch strategy: `main` for small docs pulses; feature branches for validator code.

Worktree strategy: use worktrees for parallel DCR implementation when multiple
agents work concurrently.

Change-control trigger: update `CHANGE_CONTROL.md` before altering VTRACE stage
semantics, ID conventions, template contracts, source custody, or public skills.

## Validation Levels

| Level | Scope | Required Commands / Evidence | Required Before |
|---|---|---|---|
| L0 | Docs sanity | `git diff --check` | commit |
| L1 | Repo confidence | JSON parse for `sources/source-registry.json`; hello-world compile/run when touched | push |
| L2 | Adoption readiness | Self-adoption trace/review updated; realistic scenario when available | release claim |

## Implementation Readiness Decision

Decision: pass_with_risk

Rationale: VTRACE is ready for DCR-driven follow-on work. Tooling and realistic
adoption evidence remain open risks.
