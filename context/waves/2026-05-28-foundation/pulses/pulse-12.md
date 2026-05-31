# Pulse 12: Existing Repo Migration Example

## Goal

Close `WP-003` by adding a deterministic realistic migration example.

## Changes

- Add `examples/existing-repo-migration/`.
- Baseline current behavior and target behavior in `docs/vtrace/`.
- Include one closed work package, evidence, review, and validator-compatible
  trace.

## Validation

- `py -m py_compile examples\existing-repo-migration\src\report.py`
- `py examples\existing-repo-migration\src\report.py`
- `py tools\vtrace_check.py examples\existing-repo-migration`

## Status

Complete.
