# Interfaces

## Scope

Repo or feature: `examples/hello-world`

## Interface Inventory

| ID | Interface | Type | Owner | Consumers | Compatibility Rule | Verification |
|---|---|---|---|---|---|---|
| IF-001 | Standard output line | CLI output | VTRACE | Operator, reviewer | Output text stays exactly `Hello, VTRACE!` for this example. | EVID-001 |

## Interface Details

### IF-001: Standard output line

Purpose: give the operator one visible validation signal.

Inputs: none.

Outputs: `Hello, VTRACE!`

Errors: Python runtime or syntax errors are outside the output contract.

Versioning or compatibility: any output text change requires updating REQ-001,
TRACE.md, and validation evidence.

Evidence: EVID-001.

## Open Questions

- None.
