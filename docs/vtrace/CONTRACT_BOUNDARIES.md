# Contract Boundaries

## Scope

Reusable VTRACE template for durable contract boundaries.

Use this file when a repo has contracts that cross package, command, provider,
file-format, evidence, docs, scenario, or downstream consumer boundaries.

## Boundary Rule

A work package must update this file or explicitly mark the boundary out of
scope when it creates, changes, removes, or claims any durable interface.

Durable interface means:

- public command input or output,
- API, provider, service, or plugin input/output behavior,
- typed graph/state consumed by another layer,
- persisted manifest, lock, receipt, package record, profile, evidence record,
  proof record, registry record, or report,
- file-format adapter or path/address behavior,
- fixture lifecycle state,
- scenario scorecard,
- docs/corpus claim that external users may rely on.

## Boundary Classes

| Boundary class | Trigger | Owning spec home | Required controls |
|---|---|---|---|
| Command contract | Public command, flag, output, or transcript changes. |  | command-promotion status, diagnostics, docs impact, scenario |
| API/provider contract | Provider, service, plugin, or API IO changes. |  | descriptor/schema, fixture, security/custody review when external IO appears |
| Data/model contract | Manifest, graph, state, workflow, policy, target, or typed model changes. |  | schema/profile, fixture, inspect/report output |
| Receipt/evidence contract | Build, check, test, proof, run, publish, capture, or evidence record changes. |  | stable schema, retention behavior, inspect/report output |
| Diagnostic contract | Symbolic or stable diagnostic behavior changes. |  | stable ID allocation or queued allocation, negative fixture |
| Descriptor contract | Durable schema/profile requires an inventory. |  | descriptor name, version, compatibility, migration behavior |
| Path/address contract | Content must be resolved, found, set, inserted, or inspected by address. |  | adapter kind, mutation safety, fidelity rule, negative cases |
| Adapter/profile contract | File AST, semantic AST, artifact profile, or binary profile behavior changes. |  | profile schema, lossiness rule, fixture |
| Custody/rights contract | Source, generated, captured, package, or publication data crosses custody or sensitivity boundaries. |  | custody profile, redaction rule, trust review |
| Registry/trust contract | Signing, trust roots, registry, promotion, rollback, or pruning changes. |  | signing/trust fixture, rollback scenario |
| Scenario contract | Scenario package becomes reusable proof point or fixture seed. |  | scenario scorecard, findings file, spec links |
| Docs/corpus contract | User-facing docs or corpus rows make a behavioral claim. |  | evidence pointer, claim boundary, corpus status |

## Boundary File Inventory

| File | Boundary responsibility |
|---|---|
| `PACKAGE_BOUNDARIES.md` | Package, module, command, dependency, and documentation ownership. |
| `SPEC_MODEL.md` | Required shape for product specs, contract specs, and scenario coupling. |
| `CONTRACT_BOUNDARIES.md` | Durable contract classes and trigger conditions. |
| `WORK_PACKAGES.md` | Package discipline, closeout fields, and claim limits. |
| `COMMUNICATIONS_STRATEGY.md` | Audience-specific communication and claim-control plan. |
| scenario files | Workflow contracts and fixture seeds. |

## Package Closeout Requirements

Every future package must report:

1. affected boundary classes,
2. contract specs added or changed,
3. descriptor impact,
4. diagnostic allocation impact,
5. path/address impact,
6. scenario packages added or updated,
7. fixture candidates and negative cases,
8. docs/corpus impact,
9. public claim allowed on close,
10. public claim still blocked.