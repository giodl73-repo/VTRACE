# Tutorial 02: Add CONOPS And Specs

Goal: turn user scenarios into controlled behavior.

1. Add `CONOPS.md`.
2. Write one concrete scenario with actor, trigger, inputs, normal path,
   degraded path, output, and handoff.
3. Link requirements to the scenario.
4. Add current and target behavior rows in `SPECIFICATION_BASELINE.md`.
5. Mark unknown behavior as `unknown` instead of pretending it is accepted.
6. Run `vtrace validate <repo>`.

Expected result: non-trivial implementation has a scenario and a specification
baseline before work packages start.
