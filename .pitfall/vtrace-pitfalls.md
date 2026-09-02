# VTRACE Pitfalls

## VTRACE-PF-01: NASA Inspiration Becomes NASA Authority

**Status:** MITIGATED

**Pattern:** Public NASA source references are presented as NASA authorship,
endorsement, certification, or proof that a target repo follows a NASA process.

**Domain:** README claims, framework docs, source maps, adoption guidance,
public summaries, and research writeups.

**Detection difficulty:** NASA-derived vocabulary carries authority even when
the repo is explicit that it only adapts public guidance.

**Structural solution:** Keep source custody pointer-first, preserve
NASA-inspired wording, and require source-custody review for source or standards
changes.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/source-custody.md`,
`docs/vtrace/SOURCE_BASIS.md`, and `.roles/parliament/source-custody-counsel.md`.

## VTRACE-PF-02: Process Artifact Becomes Product Feature

**Status:** MITIGATED

**Pattern:** VTRACE work packages, reviews, readiness gates, evidence rows, or
briefs are added to a target product's CLI, API, schema, or UX because they
appear in a VTRACE plan.

**Domain:** Adopter repos, generated templates, agent handoffs, and reusable
framework guidance.

**Detection difficulty:** VTRACE terminology can look implementation-ready
because it is structured, reviewed, and trace-linked.

**Structural solution:** Keep the product-first infrastructure budget and
product-boundary rule visible in docs, templates, skills, and agent handoffs.

**Evidence:** `docs/framework/product-first-infrastructure-budget.md`,
`docs/framework/vtrace-process.md`, `docs/vtrace/PACKAGE_BOUNDARIES.md`, and
`docs/vtrace/REVIEW.md`.

## VTRACE-PF-03: Trace Table Becomes Evidence

**Status:** MITIGATED

**Pattern:** A complete-looking trace row, review checklist, or work-package
record is treated as proof that behavior works, even when the cited command,
artifact, user scenario, or review evidence is missing.

**Domain:** Requirements trace, validation records, verification tables,
reviews, and portfolio adoption reports.

**Detection difficulty:** Bidirectional trace creates visual completeness; the
missing part is often the external evidence receipt rather than the row shape.

**Structural solution:** Require evidence identifiers and validation commands
to point to objective artifacts, generated reports, tests, examples, or review
records.

**Evidence:** `docs/vtrace/TRACE.md`, `docs/vtrace/VERIFICATION.md`,
`docs/vtrace/VALIDATION.md`, and `cargo test`.

## VTRACE-PF-04: Live Helper Output Becomes Deterministic Evidence

**Status:** MITIGATED

**Pattern:** Provider, GitHub, pulse, or remote CI output is treated as
repeatable local evidence without recording the external tool, auth state, run
identifier, commit, or explicit live action.

**Domain:** Provider draft/review helpers, GitHub issue and PR packets, pulse
sync, remote CI receipts, and adoption reports.

**Detection difficulty:** Dry-run packets and live packets share command
surfaces, while remote systems can change independently of the repo.

**Structural solution:** Keep dry-runs deterministic, require explicit live
flags and availability checks, and record durable run metadata whenever live
external systems provide evidence. The Live Evidence Custodian role and
`docs/vtrace/pitfall-integration-boundaries.v1.json` block live helper output
from becoming deterministic local evidence, repeatable validation, canonical
artifact updates, work-package closure, review pass, or provider-authored truth.

**Evidence:** `docs/framework/cli-orchestrator.md`, `docs/how-to/run-the-cli.md`,
`docs/vtrace/TRACE.md`, `docs/vtrace/VERIFICATION.md`,
`docs/vtrace/pitfall-integration-boundaries.v1.json`, `.roles/ROLE.md`,
`.roles/parliament/live-evidence-custodian.md`, and `tests/cli.rs`.

## VTRACE-PF-05: PITFALL Doctrine Is Copied Into Trace Ownership

**Status:** MITIGATED

**Pattern:** VTRACE requirements, validators, or review records copy PITFALL
principle and pitfall prose, then start treating doctrine truth as a VTRACE
validation concern.

**Domain:** PITFALL integration, adopter templates, validator expansion, role
reviews, and research evidence ledgers.

**Detection difficulty:** Reusing the prose feels helpful, but it blurs whether
VTRACE is tracing evidence or PITFALL is preserving decision doctrine.

**Structural solution:** Cite local PITFALL IDs from VTRACE rows and reviews,
keep PITFALL validators responsible for doctrine structure, and keep VTRACE
validators limited to trace/evidence presence and artifact integrity. The
PITFALL Doctrine Boundary Steward role and
`docs/vtrace/pitfall-integration-boundaries.v1.json` block copied PITFALL prose
from becoming VTRACE-owned requirement semantics or validator truth.

**Evidence:** `README.md`, `.roles/ROLE.md`,
`docs/vtrace/ROLE_RECOMMENDATIONS.md`, and
`docs/framework/vtrace-process.md`,
`docs/vtrace/pitfall-integration-boundaries.v1.json`,
`.roles/parliament/pitfall-doctrine-boundary-steward.md`, and `tests/cli.rs`.
