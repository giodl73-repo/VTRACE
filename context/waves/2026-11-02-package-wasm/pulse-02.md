# Pulse 02: bounded Rust adapter and executing-WASM probe

## Decision and trace

REQ-BROWSER-001 / SPEC-BROWSER-001 uses the dedicated unpublished
`browser/package-inspector` crate. It depends on the original std-only native
library by an in-repo path and pins serde1.0.228, serde_json1.0.149 and
wasm-bindgen0.2.127 in its separate committed lock. The production manifest,
lock and CLI remain unchanged. No TRACKER-relative build dependency is added.

The adapter admits exact artifact names, text, file/line/package/request limits,
and a whole-result budget; native policy stays in the library. JSON transports
all original findings, summary, work-package, review-lane and communication
views. Unknown Markdown content remains separate from those known-field views.
Input commands/links/status strings are caller text, never external evidence.

## Evidence and validation

Seven adapter tests pass, covering native missing-file findings, own-edit/source
locations, duplicate/path/unknown-field refusal, exact UTF-8/line/package limits,
unknown Markdown/claimed statuses, actual amplified report-budget overflow and
a real native subprocess with an oversized multibyte request. Strict host and
wasm32 Clippy pass; native release and actual WASM release builds pass.

A local-only probe in TRACKER's ignored `.local-tools/probe-vtrace-adapter.py`
generates 33 requests, repeats each native execution, independently compares
14 admitted finding sets against the original `vtrace validate` executable, and
matches every JSON result with executing WASM in Node. The actual instance grows
to exactly64MiB and refuses one more page. Fixtures/receipts are ignored under
`browser/package-inspector/.cache/`. These are local diagnostic probes, not the
final delivered-source recipe or browser-worker/hosted/live certification.

The first Node memory probe assumed wasm-bindgen exported `__wasm`; its Node
wrapper exports only evaluate. The corrected harness captures the actual
WebAssembly.Instance at construction, restores the constructor, then tests that
same executing instance's memory. Generated binding code and compiled memory
limits are unmodified. An earlier local generator had an unmatched bracket,
corrected before producing the request set. No product assertions/limits change.

## Remaining work

Implement the own-file browser task, action-to-file/line navigation, complete
original/current/accepted exports, real read/startup/compute deadlines and stale
result fencing. Add a repo-owned reproducible build/control recipe, independent
browser native/WASM parity, source/legal packaging and outside-Git replay, then
qualify hosted/default/fresh live delivery before advancing TRACKER. The draft
PR and canonical pointer remain deliberately unqualified for release.
