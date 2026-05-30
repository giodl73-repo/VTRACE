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
| 08 | Validator candidates | pending | Decide whether lightweight automated checks are warranted. |

## Success Criteria

- README explains the repo purpose and validation command.
- Product plan names users, waves, dependencies, and non-goals.
- Source pass cites public NASA references.
- First reusable skill exists under `skills/`.
- Focused assessment, adoption, and gate-review skills exist.
- Stage templates exist for repo-local `docs/vtrace/` adoption.
- Repo-local wave, pulse, and research skills exist.
- `git diff --check` passes.
