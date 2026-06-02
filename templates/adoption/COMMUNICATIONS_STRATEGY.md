# Communications Strategy

## Purpose

This artifact plans user-facing docs separately from controlled specifications.
Specs define the engineering baseline. Communications docs explain accepted
features, examples, scenarios, and release meaning to users, maintainers, and
stakeholders.

## Surface Plan

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
|---|---|---|---|---|---|---|---|
| COMMS-README-001 | NEED-001 / CON-001 | repo users / maintainers | Where do I start and which docs exist? | `docs/README.md` | every docs wave |  | proposed |
| COMMS-CONCEPTS-001 | NEED-001 / REQ-001 / SPEC-001 | users / contributors | What mental model explains this repo? | `docs/concepts/` | when mission, architecture, or specs change |  | proposed |
| COMMS-HOWTO-001 | CON-001 / REQ-001 / IF-001 | task users | How do I complete a focused task? | `docs/how-to/` | when interface or workflow behavior changes |  | proposed |
| COMMS-TUTORIALS-001 | CON-001 / VAL-001 | new adopters | How do I learn this end to end? | `docs/tutorials/` | when onboarding scenarios change |  | proposed |
| COMMS-EXAMPLES-001 | SPEC-001 / IF-001 / EVID-001 | implementers / integrators | What can I copy and what output should I expect? | `docs/examples/` | when public behavior changes |  | proposed |
| COMMS-TRACES-001 | WP-001 / EVID-001 / VAL-001 | reviewers / maintainers | How does this feature work end to end? | `docs/traces/` | when work packages close |  | proposed |
| COMMS-CORPUS-001 | WP-001 / REVIEW.md | docs owners | Who owns each doc surface and when is it updated? | `docs/CORPUS.md` | every docs wave |  | proposed |

## Derivation Rules

- Every mission gets a docs map or an explicit no-user-docs rationale.
- Every important CONOPS scenario gets a tutorial, walkthrough, trace, or deck.
- Every user-visible requirement or specification gets a concept, how-to,
  reference section, example, or explicit internal-only disposition.
- Every public interface gets copyable usage and expected-output documentation.
- Every closed work package updates release notes, trace walkthroughs, corpus
  rows, or records that no communications change is needed.
- User docs must not replace specs, verification, validation, or evidence rows.

## Review Checklist

| Item | Required | Decision | Evidence / Rationale |
|---|---|---|---|
| Docs claims trace to controlled source IDs. | yes | pending |  |
| Concepts/tutorials/examples do not overclaim unvalidated behavior. | yes | pending |  |
| Public interfaces have expected usage or expected output docs. | if applicable | pending |  |
| `docs/CORPUS.md` names ownership and update obligations. | if multiple surfaces exist | pending |  |
