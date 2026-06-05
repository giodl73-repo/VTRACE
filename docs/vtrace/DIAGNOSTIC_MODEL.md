# Diagnostic Model

## Scope

Reusable VTRACE template for diagnostics as a first-class product surface.

Use this file when a repo has diagnostics, errors, warnings, validation
messages, lint findings, CLI output, LSP/TUI messages, CI annotations, or other
machine-readable feedback that users or downstream tools rely on.

Diagnostics are not just logs. Durable diagnostics are contracts that explain why
work is blocked, degraded, accepted with risk, or missing evidence.

## Authority Boundary

Record the diagnostic authorities for this adoption:

| Surface | Authority |
|---|---|
| Diagnostic registry |  |
| Symbolic diagnostic names |  |
| Stable diagnostic IDs |  |
| Rendering surfaces | CLI / API / LSP / TUI / CI / docs / custom |
| Fixture root |  |
| Redaction/custody rules |  |

## Diagnostic Contract

Every durable diagnostic must define or explicitly mark out of scope:

1. stable ID or symbolic allocation name,
2. owning spec area,
3. trigger condition,
4. severity,
5. subject kind,
6. primary location,
7. path/address location when file content is addressable,
8. model, graph, receipt, evidence, provider, package, policy, or docs links
   when relevant,
9. redaction and custody behavior,
10. remediation hint or blocked-next-action text,
11. positive fixture that avoids the diagnostic,
12. negative fixture that asserts the diagnostic,
13. rendering impact for CLI, API, LSP, TUI, CI, inspect, or docs,
14. compatibility and migration behavior if the ID changes.

## Severity Model

| Severity | Meaning | Package rule |
|---|---|---|
| `error` | Blocks command, package, publish, run, proof, readiness, or claim. | Requires negative fixture before L2 close. |
| `warning` | Allows progress but records risk, degraded evidence, or future incompatibility. | Requires visibility before public claim. |
| `info` | Explains derived state, missing optional evidence, or advisory output. | Must not look like a readiness claim. |
| `trace` | Debug-level causal detail for graph, cache, receipt, provider, or scenario replay. | Can be hidden by default but must remain inspectable when emitted. |

## Location Model

Diagnostics should locate the problem at the most actionable address:

1. source file span or path/address expression,
2. manifest/config/descriptor/profile field,
3. graph/model node or edge,
4. receipt/evidence record,
5. cache key or invalidation edge,
6. package/publication/release record,
7. scenario fixture path,
8. docs/corpus claim.

If a diagnostic references generated, captured, provider, redacted, private, or
external content, it must define redaction behavior and avoid leaking secrets in
any rendering surface.

## Diagnostic Families

| Family | Examples | Required controls |
|---|---|---|
| Source/config | invalid config, path escape, missing source custody, generated-source drift | source location, path/address, negative fixture |
| Model/graph | missing node, invalid edge, stale mapping, skipped validation step | model node/edge, inspect/report output |
| Provider/API | missing provider, incompatible descriptor, stale capture, secret exposure | descriptor/schema, custody, fixture |
| Policy/security | missing authority, policy conflict, guard conflict, downgrade by cache | policy link, blocked action |
| Receipt/evidence | failed receipt advances state, hash drift, missing proof, stale evidence | receipt/evidence link, inspect/report output |
| Cache/build/check | stale cache, weaker-policy cache hit, invalidation miss, build/check mismatch | cache key, invalidation edge, receipt |
| Runtime/proof | unchecked run, dry-run misuse, unsupported claim, eval mismatch | runtime target, proof/eval evidence |
| Package/publish/release | missing signing, unknown trust root, stale package, blocked readiness | package/publication/release record |
| Path/address | invalid address, mixed roots, unsafe insertion, lossy mutation | adapter/profile, mutation safety |
| Docs/corpus | public claim exceeds evidence, stale corpus row, example lacks fixture | evidence pointer, claim boundary |

## Allocation Rule

Symbolic diagnostics in draft specs must be promoted before implementation
claims depend on them.

Promotion requires:

1. stable ID in the diagnostic registry,
2. owning work package,
3. linked spec,
4. negative fixture,
5. expected rendered output or machine-readable envelope,
6. compatibility note if replacing an older symbolic name.

## Package Closeout Requirements

Diagnostic-affecting work packages must report:

1. diagnostic family,
2. severity,
3. location model,
4. redaction/custody behavior,
5. stable ID or allocation status,
6. negative fixture path or explicit deferral,
7. rendering surfaces affected,
8. public claim allowed on close,
9. public claim still blocked.