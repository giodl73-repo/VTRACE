# Pulse 04: exact source delivery and hosted qualification

## Decision and trace

REQ-BROWSER-001 / SPEC-BROWSER-001 needs a reproducible complete native/browser
package. The source ZIP includes every tracked Git blob, its mode/SHA256 and
the original split-grant LICENSE. This preserves the templates, role files,
examples and self-trace docs required by the unchanged native CLI tests. No
external source is fetched or mirrored. Rust dependencies and runtime notices
are delivered separately, with software metadata distinguished from content terms.

The Pages workflow repeats native and adapter tests/lint, real-worker checks,
exact packaging, complete delivery and outside-Git source replay before uploading
the tested static artifact. Qualification receipts include hidden .cache files;
deployment runs only from main with least-privilege Pages permissions.

## Retained local evidence and candidate follow-up

At cb495c4, all23 packaged checks passed over71 served files, with244 tracked
source files,21 dependency notice groups and12 Rust notices. Runtime was273156
bytes (512KiB budget). Delivered source rebuilt outside Git in a path with spaces:
all31 original native tests, four reader tests, seven adapter tests,35 native
outcomes,16 independent CLI finding controls and21 task/recovery checks passed.
No cross-directory binary identity is claimed. A desktop/mobile visual pass
reported no page errors; its single local startup observation was247ms, not a
general performance claim.

The current candidate adds one Unicode/lone-CR work-package identifier request,
now36 native cases and17 independent CLI controls. The CLI comparator splits
only native LF record delimiters, preserving CR and U+2028 as message data rather
than treating them as extra records. All36/17 native controls pass. The example
walkthrough stays visible after acceptance; an explicit current-file edit action
keeps accepted-source navigation from moving mobile keyboard focus invisibly.
The candidate must repeat all packaged/source/hosted/default/live gates.

## Boundaries and remaining gates

The native CLI, original31 tests, root manifest and lock stay frozen. Original
PITFALL integration commits remain preserved remotely. Current caller Markdown,
review/evidence status and native known-field views confer no actual execution,
human review, product readiness or NASA endorsement. Do not advance the canonical
checkout or TRACKER pointer before fresh main/live artifact qualification.
