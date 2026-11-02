# Pulse 01: qualify package reads independently of validation rules

## Decision and trace

REQ-BROWSER-001 / SPEC-BROWSER-001 needs the original validator over supplied
Markdown. Add `PackageReader` (`exists`, `read_to_string`) and the original
`FileSystemReader`; native public functions delegate to the same rules through
that reader. Corresponding `*_with_reader` APIs expose checks and all original
extractions. Native `run_checks` retains its original root canonicalization;
the reader variant interprets paths according to the supplied reader.

This is input access separation, not a new parser or validation policy. Native
read errors retain the original empty-text fallback. No production dependencies,
CLI source, manifest, lock or original tests change.

## Evidence and validation

All original nine unit tests and 22 CLI tests pass. Four added independent
filesystem/supplied-text tests cover all native public queries, every check pass,
exact finding location/message/order, native duplicate-ID and substring quirks,
optional omissions, CRLF and absent-directory behavior. Sixteen captured
original native CLI validate/status/plan/adoption outputs are byte-identical
over frozen self/migration/empty/absent packages. Their canonical JSON SHA256 is
67e4618bde1cf48ec07f72c8cc090e4a1462d0c770669387290299d33bcfdcea.

`cargo test --locked`, formatting, strict all-target native Clippy and wasm32
library Clippy pass; the release wasm32 library builds. Native self and migration
validation pass before the extraction, with their original public APIs retained.
Original unit-test module, CLI tests, CLI source, manifest and lock match the
canonical baseline's Git bytes. Native-only orchestration remains unchanged.

Initial extraction editing produced 16 unbound-name compiler errors because a
mechanical path replacement matched the suffix of compound variable names.
Corrected reader calls compile and all complete suites pass. A later test-only
struct cleanup required formatter output before the final formatting gate.
No original assertion, policy condition or test was removed or waived.

## Remaining work

This foundation is not a browser release. Implement the dedicated bounded
companion, actionable own-file UI, native/real-WASM/independent CLI controls,
original/current/accepted export custody, source/legal packaging, outside-Git
replay and hosted/default/fresh live qualification. The canonical checkout and
TRACKER pointer remain unchanged until that complete task qualifies.
