# Wave: Foundation

## Goal

Create the VTRACE repository foundation: source-grounded positioning, product
plan, wave/pulse records, repo-local skills, and the first reusable V-model
rigor skill.

## Thesis

VTRACE should begin as a standards/protocols repo because its value is not a
runtime library. Its first job is to make traceability, V&V planning, and
objective evidence repeatable across codebases.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Repository foundation | complete | Create repo skeleton, docs, skills, and source-grounded first contract. |
| 02 | Source custody and encoding model | complete | Add source registry, rights posture, encoding schema, and first trace template. |
| 03 | Usable adoption process | complete | Add process docs, review gates, templates, and focused skills. |
| 04 | Code rigor in the V | complete | Add pre-code rigor constraints and right-side verification evidence. |
| 05 | End-to-end example and scenarios | complete | Add hello-world VTRACE package and scenario tests. |
| 06 | Role panel | complete | Add VTRACE review lenses for process, traceability, V&V, assurance, custody, adoption, and agent continuity. |
| 07 | Implementation management | complete | Add baseline, work-package, change-control, and integration planning procedure. |
| 08 | Specification baselines | complete | Add NASA specificity map and spec-baseline controls for existing repos. |
| 09 | VTRACE self-trace | complete | Add `docs/vtrace/` as VTRACE's source-of-truth proof package. |
| 10 | Validator candidates | complete | Add a lightweight local VTRACE artifact checker and tests. |
| 11 | Language/package profiles | complete | Add concrete validation profiles for common repo types. |
| 12 | Existing repo migration example | complete | Add deterministic retrofit example with current/target behavior and closed WP. |
| 13 | Evidence ledger | complete | Add reusable evidence ledger template and validator checks. |
| 14 | Gate checklists | complete | Add gate checklist guidance, template, self-checklist, and validator checks. |
| 15 | NASA technical controls | complete | Add deeper derived technical control map with source-custody posture. |
| 16 | Rust validator | complete | Port local validator to std-only Rust crate and update self-trace evidence. |

## Success Criteria

- README explains the repo purpose and validation command.
- Product plan names users, waves, dependencies, and non-goals.
- Source pass cites public NASA references.
- First reusable skill exists under `skills/`.
- Focused assessment, adoption, and gate-review skills exist.
- Stage templates exist for repo-local `docs/vtrace/` adoption.
- Specification baselines are explicit enough to connect existing behavior,
  requirements, and work packages.
- `docs/vtrace/` proves VTRACE itself with requirements, specs, DCRs, work
  packages, trace, evidence, validation, and review posture.
- Lightweight validator checks the self-trace package.
- Language profiles, evidence ledger template, review checklist template,
  realistic migration example, and NASA-inspired technical control map exist.
- Rust validator crate exists and validates VTRACE plus the migration example.
- Repo-local wave, pulse, and research skills exist.
- `git diff --check` passes.
