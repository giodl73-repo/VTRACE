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

The main reusable skills are:

- `skills/v-model-rigor/SKILL.md` - general V-model rigor guidance.
- `skills/vtrace-assess/SKILL.md` - inspect an existing repo and find gaps.
- `skills/vtrace-adopt/SKILL.md` - create or update repo-local VTRACE
  deliverables.
- `skills/vtrace-gate/SKILL.md` - run specification, design, or readiness
  review gates.

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

VTRACE also includes a NASA specificity map:
`docs/framework/nasa-specificity-map.md`. Use it to translate NASA-style
baselines, technical reviews, configuration control, technical data management,
software assurance, and V&V into repo-local controls.

## Repository Shape

```text
docs/research/        Source-grounded research notes.
docs/framework/       VTRACE framework maps and encoding guidance.
schemas/              Machine-readable local artifact schemas.
src/                  Rust validator library and CLI.
sources/              Source registries and rights posture metadata.
skills/               Reusable public skills for codebase rigor.
templates/            Adoption templates for codebases.
examples/             End-to-end examples and process scenario tests.
docs/vtrace/          VTRACE's own source-of-truth self-trace package.
.roles/               Review lenses for VTRACE process and adoption quality.
context/waves/        Local development wave and pulse records.
.claude/skills/       Repo-local execution skills.
```

## Source Custody

VTRACE is pointer-first. Public NASA sources can inform derived guidance and may
be downloaded into ignored local caches when appropriate. Copyrighted industry
standards stay citation-only unless a separate rights review says otherwise.
See `docs/source-custody.md` and `sources/source-registry.json`.

## Applying VTRACE

Use `docs/framework/vtrace-process.md` as the operating process. For a target
repo, start with the minimum first slice under `docs/vtrace/`:

- `MISSION.md`
- `REQUIREMENTS.md`
- `SPECIFICATION_BASELINE.md`
- `TRACE.md`
- `VERIFICATION.md`
- `REVIEW.md`

Add CONOPS, architecture, interfaces, detailed design, and validation artifacts
as the repo's risk and maturity require. For existing repos, the spec baseline
should classify observed behavior as `current`, `target`, `deprecated`, or
`unknown` before non-trivial implementation starts.

Use `CODE_RIGOR.md` before implementation when a repo needs explicit coding
constraints for function size, complexity, assertions, error handling, static
analysis, or warnings. Code rigor is a left-side design constraint with
right-side verification evidence.

For non-trivial work, use `IMPLEMENTATION_PLAN.md` and `WORK_PACKAGES.md` before
coding. VTRACE implementation slices should have parent IDs, entry criteria,
exit criteria, verification commands, evidence pointers, and review gates.
Use execution control guidance for Git branches/worktrees, commits, pushes,
agent handoffs, and L0/L1/L2 validation levels.
Use assurance/security review guidance for role lanes covering systems
engineering, traceability, V&V, software assurance, security/privacy,
safety/risk, source custody, and configuration control.
Use staged execution and role recommendation guidance to drive completeness and
create ROLES-conformant review panels in target repos.
For portfolio repos that already use waves and pulses, record each VTRACE work
package inside the active repo-local pulse using `PULSE_EXECUTION.md`.

VTRACE also applies itself to itself. Use `docs/vtrace/README.md` as the
source-of-truth proof package for current VTRACE requirements, specifications,
DCRs, work packages, evidence, validation, and review posture.

## Examples

Start with `examples/hello-world/` to see the complete process on a tiny
runnable program. Use `examples/existing-repo-migration/` to see current
behavior baselined, target behavior implemented, and one work package closed.
Use `examples/scenario-tests/` to sanity-check the model against docs-only,
Rust CLI, generated-code, and high-risk-control cases.

## Validation

The foundation repo is documentation-first with a lightweight Rust validator.
Run:

```powershell
git diff --check
cargo run -- .
cargo run -- examples\existing-repo-migration
```

When touching the validator, run:

```powershell
cargo fmt --check
cargo test
```

Future waves may add schemas, checklists, CI packaging, and generated evidence
reports.

## License

MIT. See `LICENSE`.
