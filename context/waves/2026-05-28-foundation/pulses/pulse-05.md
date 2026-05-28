# Pulse 05: End-To-End Example And Scenarios

## Goal

Prove the VTRACE model on a tiny complete example and add scenario cards that
stress docs-only, Rust CLI, generated-code, and high-risk-control cases.

## Changes

- Add hello-world example with complete `docs/vtrace/` stage artifacts.
- Add a tiny runnable implementation.
- Add scenario-test cards for different adoption shapes and risk levels.

## Validation

- `git diff --check`
- `python src\hello_world.py` from `examples/hello-world`
- `python -m py_compile src\hello_world.py` from `examples/hello-world`

## Status

Complete.
