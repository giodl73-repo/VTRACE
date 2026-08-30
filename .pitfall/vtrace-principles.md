# VTRACE Principles

## VTRACE-PR-01: NASA-Inspired, Not NASA-Endorsed

**Decision rule:** Adapt public NASA systems and software engineering guidance
only as locally authored framework guidance, never as NASA authorship,
endorsement, certification, or compliance.

**Rationale:** VTRACE gets value from public systems-engineering rigor while
remaining honest about authority and source custody.

**Test:** Public docs, source maps, and adoption guidance retain
NASA-inspired language and avoid compliance or endorsement claims.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/source-custody.md`, and
`docs/vtrace/SOURCE_BASIS.md`.

## VTRACE-PR-02: Trace Exists To Support Decisions

**Decision rule:** Keep trace rows tied to the decision being made and the
objective evidence that supports or limits that decision.

**Rationale:** Traceability is useful when it exposes reasoning and evidence,
not when it adds paperwork around an untested claim.

**Test:** Requirements, specs, work packages, validation rows, and review rows
link to concrete artifacts, commands, or evidence receipts.

**Evidence:** `docs/vtrace/TRACE.md`, `docs/vtrace/VERIFICATION.md`, and
`docs/vtrace/VALIDATION.md`.

## VTRACE-PR-03: Product Boundaries Override Process Convenience

**Decision rule:** Use VTRACE artifacts to coordinate the target product, but
do not add work-package, review, readiness, or evidence machinery to the
product surface unless the product explicitly accepts that behavior.

**Rationale:** Process scaffolding can crowd out the actual product if agents
mistake VTRACE coordination terms for implementation requirements.

**Test:** New CLI/API/schema/UX behavior is justified by product requirements,
not by the presence of VTRACE rows alone.

**Evidence:** `docs/framework/product-first-infrastructure-budget.md`,
`docs/framework/vtrace-process.md`, and `docs/vtrace/PACKAGE_BOUNDARIES.md`.

## VTRACE-PR-04: Validation Is Risk-Tailored Evidence

**Decision rule:** Use tests as evidence where they match the claim, and add
scenario, operator, review, source-custody, or customer-facing validation when
the claim depends on real use.

**Rationale:** Passing code can still fail a product or adoption claim if the
claim depends on source rights, user workflow, scenario realism, or review
judgment.

**Test:** Validation records state the command or review evidence and the
claim level that evidence is allowed to support.

**Evidence:** `docs/vtrace/LANGUAGE_PROFILES.md`,
`docs/vtrace/VALIDATION.md`, and `docs/framework/code-rigor.md`.

## VTRACE-PR-05: Integration Keeps Ownership Separate

**Decision rule:** Cite PITFALL and ROLES artifacts from VTRACE trace and
review records instead of copying their doctrine into VTRACE-owned
requirements or validator semantics.

**Rationale:** PITFALL, ROLES, and VTRACE are stronger when they have distinct
ownership: doctrine, review lenses, and mission-to-evidence trace.

**Test:** VTRACE integration guidance references local PITFALL IDs and role
files while VTRACE validators stay limited to trace/evidence/artifact checks.

**Evidence:** `README.md`, `.roles/ROLE.md`, and
`docs/vtrace/ROLE_RECOMMENDATIONS.md`.
