# VTRACE Invariants

## VTRACE-INV-01: Self-Trace And Example Packages Validate

**Status:** MITIGATED

**Claim:** The root VTRACE package and the existing-repo migration example
validate with the local CLI.

**Why it matters:** VTRACE is a framework repo; if its self-trace or migration
example fails, adopter guidance loses credibility.

**Enforcement:** Run the root validator and migration-example validator after
framework, template, CLI, or evidence changes.

**Evidence:** `cargo run -- .` and
`cargo run -- examples\existing-repo-migration`.

## VTRACE-INV-02: Source Custody Stays Pointer-First

**Status:** MITIGATED

**Claim:** Public source material may be cited and summarized, but source
registry, framework, and review artifacts do not vendor restricted standard
text or convert public guidance into a compliance claim.

**Why it matters:** Source-grounded rigor depends on provenance without
redistribution or authority overclaim.

**Enforcement:** Source-custody docs and role review govern source, standards,
and provenance changes.

**Evidence:** `docs/source-custody.md`, `sources/source-registry.json`,
`docs/framework/standards-map.md`, and `docs/vtrace/SOURCE_BASIS.md`.

## VTRACE-INV-03: Trace Rows Remain Bidirectional

**Status:** MITIGATED

**Claim:** Accepted requirements and work packages keep visible links from need
to spec, design, implementation, verification, validation, evidence, and
review status.

**Why it matters:** Missing links let incomplete work look complete and make it
hard for later agents to know what evidence supports a claim.

**Enforcement:** VTRACE trace, verification, validation, and work-package
records preserve bidirectional references.

**Evidence:** `docs/vtrace/TRACE.md`, `docs/vtrace/WORK_PACKAGES.md`,
`docs/vtrace/VERIFICATION.md`, and `docs/vtrace/VALIDATION.md`.

## VTRACE-INV-04: External Side Effects Stay Explicit

**Status:** MITIGATED

**Claim:** Provider, GitHub, pulse, and worktree commands default to
deterministic dry-run/advisory behavior and require explicit flags or commands
before live external side effects occur.

**Why it matters:** External state can change outside the repo, so it must not
be confused with repeatable local validation.

**Enforcement:** CLI tests cover deterministic provider, role, GitHub, pulse,
and guarded worktree command surfaces.

**Evidence:** `docs/framework/cli-orchestrator.md`, `docs/how-to/run-the-cli.md`,
`src/main.rs`, and `tests/cli.rs`.

## VTRACE-INV-05: Product-Boundary Language Remains Visible

**Status:** MITIGATED

**Claim:** Templates, skills, framework docs, and CLI handoffs keep the
distinction between VTRACE process artifacts and target product behavior
visible to future agents.

**Why it matters:** Future agents can otherwise add process machinery to a
product while believing they are satisfying product requirements.

**Enforcement:** Product-first framework docs, package-boundary rules, and
future-agent role review preserve the boundary.

**Evidence:** `docs/framework/product-first-infrastructure-budget.md`,
`docs/framework/vtrace-process.md`, and `.roles/stakeholders/future-agent.md`.
