# Communications Strategy

## Scope

Repo: VTRACE

This artifact defines how VTRACE turns controlled process artifacts into
user-facing docs guidance for target repos. It is separate from VTRACE specs:
specs control the framework and CLI behavior; communications docs explain how
adopters should teach, demonstrate, and govern their own repo documentation.

## Surface Plan

| Surface ID | Source IDs | Audience | User Question | Generated Docs | Cadence | Owner | Status |
|---|---|---|---|---|---|---|---|
| COMMS-README-001 | NEED-001 / REQ-COMMS-001 / SPEC-018 | repo maintainers | Where does communications strategy fit in VTRACE? | `README.md`, `docs/vtrace/README.md` | when adoption guidance changes | VTRACE | accepted |
| COMMS-CONCEPTS-001 | REQ-COMMS-001 / SPEC-018 | adopters / docs owners | How do VTRACE source artifacts become user-facing concepts? | `docs/framework/communications-strategy.md` | when stage semantics change | VTRACE | accepted |
| COMMS-TEMPLATE-001 | REQ-COMMS-001 / SPEC-018 / IF-009 | target repo maintainers | What file do I copy into a repo to plan docs? | `templates/adoption/COMMUNICATIONS_STRATEGY.md` | when template contracts change | VTRACE | accepted |
| COMMS-CLI-001 | REQ-COMMS-001 / REQ-CLI-001 / SPEC-018 | CLI operators / agents | How do I inspect the docs package a repo should build? | `vtrace comms plan <repo>` | when CLI command surface changes | VTRACE | accepted |
| COMMS-RUNE-PATTERN-001 | CON-004 / REQ-COMMS-001 / VAL-011 | portfolio adopters | What does a realistic adoption docs package look like? | Concepts, how-to, tutorials, examples, traces, decks, and corpus governance pattern | when target repo docs waves are planned | VTRACE | accepted |

## Derivation Rules

- Mission and needs define the docs map, audiences, promises, and non-goals.
- CONOPS scenarios become tutorials, walkthroughs, trace pages, or stakeholder
  decks.
- Requirements and specification baselines become concepts, how-to guides,
  reference sections, examples, or internal-only dispositions.
- Interfaces become examples with expected inputs and outputs.
- Closed work packages become release notes, trace walkthroughs, corpus rows,
  or a recorded no-docs-impact decision.
- Validation evidence supports user-facing docs claims; it does not get treated
  as a substitute for docs.

## RUNE Adoption Pattern

For a repo like RUNE, VTRACE would recommend an adoption docs package:

- `docs/README.md` to map every docs surface,
- `docs/concepts/` for descriptor model, profiles versus adapters, and
  discovery boundaries,
- `docs/how-to/` for focused tasks such as deriving contracts, registering
  contracts, generating evidence, and using discovery,
- `docs/tutorials/` for a sequence from first struct to collection, profile,
  and adapter,
- `docs/examples/` for small copyable examples with expected JSON outputs,
- `docs/traces/` for end-to-end walkthroughs and bakeoff evidence,
- `docs/decks/` for stakeholder/adoption walkthroughs when needed,
- `docs/CORPUS.md` for document ownership and update obligations.

## Review Posture

The communications strategy requires adoption-guide/editorial,
requirements-traceability, V&V, and configuration-control review. Security,
privacy, source-custody, or safety review is added when docs expose sensitive
operations, external integrations, protected data, or risk-affecting claims.
