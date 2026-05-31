# Source Basis

## Scope

Repo: VTRACE

This file records which source categories justify VTRACE's framework claims.
The canonical source metadata remains `sources/source-registry.json`.

## Source Category Map

| Source ID | VTRACE Use | Controlled Claims | Verification |
|---|---|---|---|
| `nasa-se-handbook` | Systems engineering lifecycle, common technical processes, system design and product realization framing. | VTRACE uses a V-model-style left/right proof structure and staged technical processes. | Source registry inspection. |
| `nasa-se-handbook-appendix` | Requirement quality, requirement verification matrix, validation matrix, interface requirements outline, CONOPS outline, peer review guidance. | VTRACE templates include requirements, trace, verification, validation, interfaces, CONOPS, and review. | Source registry inspection. |
| `npr-7123-1d` | NASA systems engineering process and requirements management framing. | VTRACE treats requirements, verification, validation, technical assessment, and technical data as controlled processes. | Source registry inspection. |
| `npr-7120-5e` | Life-cycle phases, life-cycle reviews, key decision points, formulation/implementation framing. | VTRACE uses stage gates and implementation-readiness/work-package closure reviews. | Source registry inspection. |
| `nasa-swe-handbook` | Software requirements, objective evidence, software architecture, peer review evidence. | VTRACE maps software repo work to requirements, architecture, evidence, and review gates. | Source registry inspection. |
| `nasa-std-8739-8b` | Software assurance and software safety framing. | VTRACE includes assurance, safety/risk, V&V, and independent review lanes where required. | Source registry inspection. |
| `nasa-cybersecurity-policies` | Cybersecurity policy pointer and security/privacy review trigger. | VTRACE includes security/privacy review lanes for affected changes. | Source registry inspection. |
| `jpl-power-of-ten` | Coding-verifiability constraints, small functions, bounded control flow, assertions, static analysis. | VTRACE code-rigor guidance includes reviewable size, static checks, assertion/error-handling expectations. | Source registry inspection. |
| `iso-iec-ieee-15288` / `iso-iec-ieee-29148` | Citation-only comparison points for lifecycle and requirements engineering language. | VTRACE records these as background comparison only, not source text. | Source custody inspection. |

## Derived Control Evidence

`docs/framework/nasa-technical-controls.md` is the current derived control map
for technical requirements, technical assessment, technical data management,
configuration management, interface management, V&V, assurance, and
safety/security review lanes.

## Source Custody Rules

- Public NASA web references may be cited and summarized.
- Source bytes are not committed unless the source registry permits it.
- Copyrighted industry standards stay citation-only.
- VTRACE does not claim NASA endorsement or compliance.
- Any new source-derived rule requires source-custody review.

## Current Source Decision

Decision: pass

Rationale: VTRACE's self-trace stays derived and pointer-first. The source
registry is parseable JSON and remains the canonical source custody artifact.
