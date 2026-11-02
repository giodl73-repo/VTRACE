# Input, report and claim boundary

The companion mounts the original VTRACE structural validator over supplied
Markdown. A request has `files`, each with exact `name` and UTF-8 `text`; all
eleven artifact names consumed by the native library are admitted. The logical
`docs/vtrace` package is present even when its supplied file set is empty. File
presence means a supplied name, not repository availability or evidence custody.

Browser resource admission rejects whole operations: at most eleven unique
artifact filenames, filename 64 UTF-8 bytes, request 1 MiB, each document 64 KiB,
all text 256 KiB, each line 2048 UTF-8 bytes, each document 2048 native `.lines()`
entries and report 2 MiB. These are named browser limits, not native validator
policy or general repository limits. Empty lines and CRLF use Rust's native
line semantics. No text/rows/findings are truncated. Oversized native-control
stdin is refused before decoding its potentially split UTF-8 prefix.

The core retains its original ten check passes and all native query functions.
The report transports original findings, summary, work packages, review lanes
and communication surfaces. Report paths normalize separators for cross-platform
presentation; supplied text is not normalized by Rust. Extracted rows are native
known-field views, not lossless Markdown serialization. Original/current input
exports must retain unknown columns and text independently of those views.

Zero findings is not a readiness certificate. Required-file presence can coexist
with empty documents; ID visibility uses existing substring/token heuristics;
duplicate IDs can collapse; malformed rows and absent optional artifacts can
be skipped. Status strings, commands, links and review decisions are caller
assertions. The inspector does not execute evidence, run providers/GitHub
helpers, authenticate reviews, assess requirements quality or establish product
acceptance, safety, compliance or NASA endorsement. References remain inert.

Original filesystem APIs and their error fallback remain native-compatible.
The browser UI must reject undecodable UTF-8 instead of silently replacing bytes.
Real worker/read/startup/compute deadlines and compiled memory limits still need
full runtime/browser qualification; native adapter tests alone do not prove them.
