# Pulse 08: Specification Baselines

## Goal

Make specification explicit enough that existing repos can move from observed
behavior and requirements into controlled implementation packages.

## Changes

- Add NASA specificity map for translating NASA-style controls into VTRACE
  repo artifacts.
- Add specification-baseline guidance for existing repo retrofit.
- Add `SPECIFICATION_BASELINE.md` adoption template.
- Update process, staged execution, implementation management, trace,
  implementation-plan, and work-package guidance to require `SPEC-*` linkage.
- Update assessment, adoption, and gate skills to inspect and govern
  specifications.
- Add hello-world specification baseline and trace links.

## Validation

- `git diff --check`
- JSON parse check for `sources/source-registry.json`
- Hello-world compile and run checks

## Status

Complete.
