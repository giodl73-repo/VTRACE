# Pulse 05: qualified implementation trace and source replay isolation

The d8472c5 candidate passed hosted run41483576915 (synthetic checkout
9ba7aeda8fbe4d7832a078ec05522f5d9c55ed8e). Its complete delivered source rebuilt
outside Git:31 original native tests, four reader tests, seven adapter tests,
36 native outcomes,17 original CLI controls and21 browser checks passed. Local
complete packaged delivery passed23 checks. The durable dated receipt is
`docs/vtrace/browser-implementation-qualification.v1.json`.

Promote only the completed software implementation into the controlled self
requirements/spec/change/work/trace/evidence/verification/validation/profile/
communication/review records. Main/default/live publication stays gated in this
wave. Review rows describe agent checks and retained risks, not human approval,
NASA endorsement, authentic external evidence or product acceptance.

## Cache-context correction

A subsequent ordinary native test run reused a CLI test executable from the
removed temporary source extraction: all22 CLI tests failed with missing/invalid
directories. The executable contained the exact deleted extraction path and
not this checkout path. The unchanged tests use compile-time CARGO_MANIFEST_DIR.
Refreshing first-party timestamps and recompiling restored all35 native tests;
self/migration validation and the frozen original source/test/lock guard pass.

Source replay now builds its native tests and original CLI in the named
`vtrace-replay` profile within the same shared Cargo target. Every extraction
refreshes first-party Rust timestamps so repeat replays recompile embedded
paths. It also removes inherited VTRACE_URL before testing its own rebuilt
browser site. No original test, production manifest/lock, policy rule or shared
cache content was deleted or weakened. The failure log remains in ignored
.cache/cache-context-failure.log. Final source replay and a subsequent ordinary
native run must demonstrate that the two contexts remain separate.

## Remaining gates

Repeat clean committed packaging, all23 packaged checks, outside-Git replay,
and ordinary native tests after replay; require fresh hosted qualification,
main/default deployment, fresh live browser behavior and exact served hashes.
The hosted workflow also reruns ordinary checkout tests after replay to guard
against future cache-context regressions. Canonical checkout and TRACKER gitlink
remain unchanged until those gates pass.
