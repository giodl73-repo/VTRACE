# Existing Repo Migration Example

This example models a small repo that already has behavior before VTRACE is
introduced. The migration baselines current behavior, records a target change,
implements one work package, and closes the V with evidence.

## Current Behavior

`src/report.py` formats a status line from an item name and state.

## Target Change

Preserve the current status format and add deterministic normalization of
leading/trailing whitespace before formatting.

## Validation

```powershell
py -m py_compile examples\existing-repo-migration\src\report.py
py examples\existing-repo-migration\src\report.py
cargo run -- examples\existing-repo-migration
```
