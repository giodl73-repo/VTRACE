# Plan A Docs Package

Use this when a repo needs user-facing docs in addition to VTRACE specs.

1. Copy `templates/adoption/COMMUNICATIONS_STRATEGY.md` into
   `docs/vtrace/COMMUNICATIONS_STRATEGY.md`.
2. Map mission needs to `docs/README.md` and audience promises.
3. Map each important CONOPS scenario to a tutorial, walkthrough, trace, or
   deck.
4. Map user-visible requirements and specs to concepts, how-to guides,
   reference sections, or examples.
5. Map public interfaces to copyable examples and expected outputs.
6. Map closed work packages to release notes, trace walkthroughs, corpus rows,
   or a recorded no-docs-impact decision.
7. Run `vtrace comms plan <repo>`.
8. Run `vtrace validate <repo>`.

Expected result: the comms plan lists declared `COMMS-*` rows and recommended
docs surfaces.
