# VTRACE Self-Trace Source Of Truth

This directory is the authoritative VTRACE package for the VTRACE repo itself.
When VTRACE guidance, templates, skills, roles, examples, or source custody
rules change, this package is the proof surface that shows why the change
exists, what it controls, how it is verified, and which open risks remain.

## Operating Rule

The framework docs explain VTRACE. This directory proves VTRACE with VTRACE.

Use this package as the first place to answer:

- what VTRACE is required to do,
- which behavior is currently controlled,
- which gaps are accepted as design change requests,
- which work packages are allowed next,
- which commands and inspections prove the current claim,
- which risks prevent a stronger readiness claim.

## Source Of Truth Map

| Question | File |
|---|---|
| Why does VTRACE exist? | `MISSION.md` |
| How is VTRACE used? | `CONOPS.md` |
| What must VTRACE provide? | `REQUIREMENTS.md` |
| What controlled behavior/specs prove those requirements? | `SPECIFICATION_BASELINE.md` |
| How is the repo organized? | `ARCHITECTURE.md`, `PACKAGE_BOUNDARIES.md` |
| Which public adoption contracts must stay stable? | `INTERFACES.md` |
| Which design decisions govern VTRACE? | `DESIGN.md` |
| What rigor applies to future validators and profiles? | `CODE_RIGOR.md` |
| How should VTRACE derive user-facing docs from controlled artifacts? | `COMMUNICATIONS_STRATEGY.md` |
| What changes are requested? | `CHANGE_CONTROL.md` |
| What shape must deep product specs follow? | `SPEC_MODEL.md` |
| Which durable contracts need boundary controls? | `CONTRACT_BOUNDARIES.md` |
| How should scenarios prove specs and error paths? | `SCENARIO_MODEL.md` |
| How should durable diagnostics be shaped and allocated? | `DIAGNOSTIC_MODEL.md` |
| How is the large problem space mapped? | `PROBLEM_SPACE_MAP.md` |
| Where do domain discoveries wait before becoming specs or packages? | `DOMAIN_BACKLOG.md` |
| How does research become requirements input? | `RESEARCH_PLAN.md` |
| How do scenario artifacts become controlled fixtures? | `FIXTURE_MODEL.md` |
| Which DCRs become work packages? | `IMPLEMENTATION_PLAN.md`, `WORK_PACKAGES.md` |
| What proves the current state? | `VERIFICATION.md`, `VALIDATION.md`, `TRACE.md` |
| What source basis supports the framework? | `SOURCE_BASIS.md` |
| What objective evidence supports the claim? | `EVIDENCE.md` |
| Which roles govern the review? | `ROLE_RECOMMENDATIONS.md`, `REVIEW.md` |
| Which stage is VTRACE in? | `STAGE_EXECUTION.md` |

## Readiness Claim

Current claim: VTRACE is ready as a documentation-first adoption framework with
local validator enforcement and a defined hardening path.

Current gate: `pass`.

The remaining risk is operational packaging: CI integration and broader live
repo adoption can harden the validator, but the docs-first process is no longer
blocked on missing profiles, examples, evidence ledgers, or gate checklists.

## Update Rule

Update this directory whenever a change affects:

- stage semantics,
- required target-repo artifacts,
- ID conventions,
- skill behavior,
- source custody rules,
- role-review lanes,
- work-package closure rules,
- validation commands,
- public adoption claims.
