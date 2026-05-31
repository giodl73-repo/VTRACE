# Pulse 14: Gate Checklists

## Goal

Close `WP-005` by adding gate-specific checklist guidance, templates, and
validator support.

## Changes

- Add `docs/framework/gate-checklists.md`.
- Add `templates/adoption/REVIEW_CHECKLISTS.md`.
- Add `docs/vtrace/REVIEW_CHECKLISTS.md`.
- Extend validator tests for required checklist row closure.

## Validation

- `py -m unittest discover -s tests -p "test_*.py"`
- `py tools\vtrace_check.py .`

## Status

Complete.
