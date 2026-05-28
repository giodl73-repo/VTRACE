# Mission

## Scope

Repo or feature: `examples/hello-world`

VTRACE adoption scope: demonstrate an end-to-end VTRACE chain on the smallest
possible runnable program.

## Mission Need

VTRACE needs a trivial example that proves the stage model, numbering scheme,
trace matrix, code-rigor placement, verification evidence, validation evidence,
and review gate can all be followed without ambiguity.

## Users

| User | Need | Success Signal |
|---|---|---|
| VTRACE maintainer | See the whole process in one tiny artifact. | Trace rows connect every stage to evidence. |
| Repo adopter | Copy the pattern into a real repo. | The example is small enough to inspect in minutes. |

## Operating Context

The example is read by maintainers and agents before applying VTRACE to larger
repos.

## Constraints

- Keep the program intentionally trivial.
- Avoid dependencies.
- Keep evidence local and reproducible.

## Non-Goals

- Do not demonstrate complex architecture.
- Do not claim safety-critical readiness.
- Do not add a full package manager project.

## Success Criteria

| Criterion | Validation Method | Evidence Pointer |
|---|---|---|
| The program prints the expected message. | Run command. | EVID-001 |
| The source compiles under Python syntax checks. | Static compile check. | EVID-002 |
| The VTRACE chain is complete for the tiny scope. | Readiness review. | REVIEW.md |

## Source Links

- VTRACE process: `docs/framework/vtrace-process.md`
