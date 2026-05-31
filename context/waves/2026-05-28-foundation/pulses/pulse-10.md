# Pulse 10: Validator Candidates

## Goal

Implement the first lightweight local validator for repo-local VTRACE packages.

## Work Package

`WP-001: Add lightweight validators`

Parent IDs: `DCR-001`, `REQ-VAL-001`, `SPEC-007`, `CR-001..CR-003`

## Changes

- Add `tools/vtrace_check.py`.
- Add unittest coverage in `tests/test_vtrace_check.py`.
- Validate VTRACE's own `docs/vtrace/` package.
- Update self-trace requirements, specs, trace, work packages, verification,
  validation, evidence, review, and pulse execution.

## Validation

- `git diff --check`
- `py -m json.tool sources\source-registry.json`
- `py -m py_compile examples\hello-world\src\hello_world.py`
- `py examples\hello-world\src\hello_world.py`
- `py -m py_compile tools\vtrace_check.py`
- `py -m unittest discover -s tests -p "test_*.py"`
- `py tools\vtrace_check.py .`

## Status

Complete.
