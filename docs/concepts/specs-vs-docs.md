# Specs Versus Docs

VTRACE treats specifications and user docs as different products.

Specifications control engineering intent:

- accepted behavior,
- interface contracts,
- compatibility rules,
- requirements allocation,
- verification and validation methods,
- status such as `current`, `target`, `deprecated`, or `unknown`.

User docs explain accepted behavior:

- concepts,
- how-to guides,
- tutorials,
- examples,
- walkthrough traces,
- decks,
- release and adoption narratives.

The communications strategy connects the two. A user doc can explain a spec,
but it should not silently create a new spec. When a user doc makes a claim
about repo behavior, that claim should trace to mission, CONOPS, requirements,
specs, work packages, validation, or evidence.
