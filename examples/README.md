# VTRACE Examples

Examples prove the VTRACE model before it is applied to larger repos.

## Included Examples

- `hello-world/` - a complete tiny implementation traced from mission through
  review.
- `scenario-tests/` - scenario cards that stress the model against different
  repo types and risk levels.

## How To Read The Hello World Example

Start with `hello-world/docs/vtrace/MISSION.md`, then follow the trace:

```text
MISSION -> CONOPS -> REQUIREMENTS -> DESIGN -> CODE_RIGOR
  -> src/hello_world.py -> VERIFICATION -> VALIDATION -> TRACE -> REVIEW
```

The example is intentionally small. Its value is showing how IDs, evidence, and
review outcomes connect.
