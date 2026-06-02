# Communications Strategy

## Purpose

VTRACE separates controlled engineering artifacts from user-facing docs.
Specifications define what the repo promises to build and prove. Communications
docs explain accepted features, usage paths, examples, and release meaning to
the audiences that need to use or govern the repo.

The communications strategy is a right-side artifact. It is derived from the
left side of the V and updated when verification, validation, and work-package
evidence close.

## Artifact

Use `docs/vtrace/COMMUNICATIONS_STRATEGY.md` when a repo needs a planned docs
package. The artifact is optional for a minimum VTRACE slice, but once present
it should be controlled like other VTRACE artifacts.

Required rows use `COMMS-*` IDs and name:

- source IDs from mission, CONOPS, requirements, specs, work packages, and
  evidence,
- target audience,
- user question,
- generated docs surface,
- update cadence,
- owner,
- status.

## Source Mapping

| VTRACE Source | Communications Output | Rule |
|---|---|---|
| Mission / Need | `docs/README.md`, audience map, promises, non-goals | Every mission gets a docs map that names who the repo serves and what docs exist. |
| CONOPS Scenario | Tutorial, walkthrough, trace, or deck | Every important scenario gets a runnable or inspectable user journey. |
| Requirement | Concept, how-to, reference section, or example | User-visible requirements get an explanation or task page unless explicitly internal. |
| Specification Baseline | Reference page, example, compatibility note, or FAQ | Public behavior and interface specs become user docs after acceptance. |
| Architecture / Package Boundaries | Concept or maintainer guide | Boundary decisions become concepts when users or contributors must understand them. |
| Interface | Reference page, example, expected output, or compatibility note | Public CLIs, APIs, schemas, file formats, and config surfaces need copyable usage docs. |
| Work Package | Release note, trace walkthrough, corpus row, or deck update | Closed implementation slices update the user-facing narrative and docs inventory. |
| Verification / Validation / Evidence | Trace page, benchmark interpretation, example output, or adoption claim | Evidence supports docs claims; it does not get buried in commit history. |

## Recommended Taxonomy

| Surface | Purpose | Typical Trigger |
|---|---|---|
| `docs/README.md` | Map the repo docs package and route readers. | Any public-facing repo docs wave. |
| `docs/concepts/` | Explain mental models, feature boundaries, and design vocabulary. | Mission, architecture, package-boundary, and spec decisions. |
| `docs/how-to/` | Answer focused tasks for users or maintainers. | Repeated user tasks, CLI/API operations, integration steps. |
| `docs/tutorials/` | Teach sequenced learning paths from simple to realistic use. | CONOPS scenarios and onboarding paths. |
| `docs/examples/` | Provide small copyable examples with expected outputs. | Interfaces, specs, and validation scenarios. |
| `docs/traces/` | Show end-to-end walkthroughs tied to VTRACE evidence. | Closed work packages and validation scenarios. |
| `docs/decks/` | Summarize stakeholder, release, or adoption narratives. | Portfolio reviews, adoption campaigns, major releases. |
| `docs/CORPUS.md` | Govern doc ownership, update cadence, and obligations. | Any repo with more than one docs surface. |

## Sizing Guidance

Use the repo mission and CONOPS to size the docs package. A small internal tool
may need only `docs/README.md`, one concept, and one how-to. A reusable public
library or portfolio dependency should usually have:

- 3-6 concept guides for the core mental model,
- 5-8 how-to guides for repeatable tasks,
- 4-6 tutorials that form a learning sequence,
- indexed examples with expected output,
- at least one trace walkthrough for an end-to-end adoption path,
- `docs/CORPUS.md` ownership and update obligations.

## Review

Communications review is not marketing review by default. It is an engineering
review lane that checks whether public explanations match controlled intent.

Minimum review lanes:

- adoption guide/editorial: docs are navigable and audience-appropriate,
- requirements traceability: docs claims trace to mission, CONOPS, requirements,
  specs, work packages, or evidence,
- V&V: examples, tutorials, and walkthroughs do not overclaim unvalidated
  behavior,
- configuration control: docs updates land with the work package or a recorded
  follow-up.

Security, privacy, source custody, or safety review is required when docs
include sensitive behavior, external integrations, operational controls, or
claims that could change risk posture.

## Validation

Run:

```powershell
vtrace comms plan <repo>
vtrace validate <repo>
```

The plan command is advisory. The validator enforces row completeness once the
communications strategy artifact exists.
