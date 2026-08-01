# Pulse 18: Evidence-backed capability assessment

## Goal

Preserve the reusable parts of the ROUTE, BASTION, and SHIELD VERDICT pilot
without creating a runtime dependency or treating a repository score as policy
or delivery authority.

## Engineering decision

VTRACE owns the generic assessment recipe. Domain repositories own their
service definitions and evidence. TAXLANE owns fiscal admission, allocation,
and rate consequences.

## Trace links expected

Each adoption identifies the scored object, its exact version, mission or
service promise, dimension evidence, applicable hard floors, review decision,
and next gate.

## Evidence produced

- `docs/framework/capability-assessment.md`
- `templates/adoption/CAPABILITY_ASSESSMENT.md`
- `docs/reviews/2026-07-31-capability-assessment-role-review.md`
- discoverability from the VTRACE and adoption-template READMEs

## Validation

- `git diff --check`
- `cargo run -- validate .`

## Outcome

Complete when both commands pass. No validator or shared crate is added until
repeated repo-local use proves stable common semantics.
