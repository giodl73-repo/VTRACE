# Architecture

## Scope

Repo or feature: `examples/hello-world`

## Architecture Summary

The example uses one Python script with one pure function and one entrypoint.
This is intentionally minimal so the trace model is the focus.

## Components

| Component | Responsibility | Requirement IDs | Interfaces | Evidence |
|---|---|---|---|---|
| `message()` | Return the expected output string. | REQ-001 | IF-001 | EVID-002 |
| `main()` | Print the message to standard output. | REQ-001 | IF-001 | EVID-001 |

## Data Flow

```text
operator -> python script -> stdout
```

## Dependencies

| Dependency | Purpose | Boundary / Risk | Verification |
|---|---|---|---|
| Python runtime | Execute the script. | Operator environment must provide Python. | EVID-001 |

## Failure Modes

| Failure Mode | Impact | Mitigation | Evidence |
|---|---|---|---|
| Python unavailable | Program cannot run. | Scope assumes Python is available. | REVIEW.md accepted risk |
| Syntax error | Program cannot run. | Compile check. | EVID-002 |

## Open Risks

- Python availability is an operator environment assumption.
