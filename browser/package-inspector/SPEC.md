# Supplied Markdown package inspection

REQ-BROWSER-001 (target): a maintainer can supply the eleven Markdown artifact
types consumed by the existing Rust validator, inspect its complete findings
with source file/line locations and its package/work/review/communication rows,
edit a supplied artifact and export original inputs, current inputs and reports.
An original synthetic missing-trace example demonstrates a finding disappearing
when the matching reference is supplied. Original text stays distinct from
editor normalization and accepted input/report state.

SPEC-BROWSER-001 (target): a dedicated Rust/WASM companion invokes all original
`run_checks` rules and all original public extraction functions through the
same library reader interface. The native filesystem APIs, CLI behavior and
std-only production dependency posture remain. Browser input is a logical
`docs/vtrace` package; file presence refers to supplied names, not a repository
or fetched evidence. Native missing-directory handling stays in the shared core.

This is VTRACE's own user-facing inspection product, satisfying the product
package destination of the 100-line tripwire. The consequential user question is
"Which structural gaps does this supplied package contain, and where can I edit
them?" Sources are owned caller Markdown or clearly synthetic examples. The
observable result is an original native finding/line and its disappearance after
the corresponding edit. The shortest demonstration opens the example and adds
its missing requirement reference to TRACE.md. Product code is estimated at
400–800 lines plus qualification/build tooling. Reuse existing Rust checks,
wasm-bindgen, browser workers and Playwright; add no orchestration engine.

The current validator is deliberately coarse. File presence does not validate
content, ID visibility uses existing string/token matching, malformed Markdown
rows can be skipped, duplicate IDs can collapse, optional absent documents can
be skipped and caller status strings are not authenticated. No-findings does not
prove complete traceability, evidence execution, completed reviews, requirements
quality, product readiness, compliance, safety or NASA endorsement. Commands,
URLs and evidence references are displayed as inert caller text, never executed.

Evidence: this wave's native controls, adapter/native/WASM outputs, original CLI
comparisons, browser/recovery cases, outside-Git source replay and exact live
artifact receipts. Gate commands and sequencing are in
`context/waves/2026-11-02-package-wasm/WAVE.md`. No publication is claimed yet.
