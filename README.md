# VTRACE

VTRACE is a standards and skills repository for applying V-model systems
engineering rigor to software codebases.

The repo is not an official NASA project. It adapts public NASA systems
engineering guidance into practical codebase practices: requirements,
architecture, interfaces, implementation evidence, verification, validation,
review gates, and traceability records.

## Purpose

Modern software repos often have tests, docs, and issue history, but they do
not always maintain an auditable line from mission need to requirement, design,
implementation, verification evidence, and validation evidence. VTRACE defines
that line as a lightweight development framework and packages reusable skills
that coding agents can apply repo by repo.

## First Skill

The first reusable skill is:

- `skills/v-model-rigor/SKILL.md` - assess or establish a V-model traceability
  spine in an existing codebase.

Use it when a repo needs stronger rigor around:

- stakeholder needs and mission objectives,
- requirements quality and allocation,
- architecture and interface ownership,
- verification planning,
- validation against intended use,
- objective evidence,
- review gates and readiness criteria.

## Source Basis

VTRACE starts from these public NASA references:

- NASA Systems Engineering Handbook:
  <https://www.nasa.gov/reference/systems-engineering-handbook/>
- NASA Systems Engineering Handbook appendix material:
  <https://www.nasa.gov/reference/system-engineering-handbook-appendix/>
- NASA Software Engineering Handbook requirement guidance:
  <https://swehb.nasa.gov/>

The repo translates those ideas for ordinary software engineering work. It does
not copy NASA process wholesale, assert NASA endorsement, or require a
spaceflight lifecycle.

## Repository Shape

```text
docs/research/        Source-grounded research notes.
skills/               Reusable public skills for codebase rigor.
context/waves/        Local development wave and pulse records.
.claude/skills/       Repo-local execution skills.
```

## Validation

The foundation repo is documentation-first. Until executable tooling exists,
the required validation command is:

```powershell
git diff --check
```

Future waves may add schemas, checklists, trace matrix validators, and generated
evidence reports.

## License

MIT. See `LICENSE`.
