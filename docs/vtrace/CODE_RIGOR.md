# Code Rigor

## Scope

Repo: VTRACE

VTRACE is currently documentation-first. Code-rigor constraints apply when
scripts, validators, schemas, or generated checks are added.

## Constraints

| ID | Constraint | Verification Method | Status |
|---|---|---|---|
| CR-001 | Validators should be small, deterministic, and runnable locally without network access. | inspection / local command | accepted |
| CR-002 | Validators should emit actionable findings tied to file paths and VTRACE IDs. | test / inspection | accepted |
| CR-003 | Source-derived checks must not embed copyrighted standard text. | source custody review | accepted |
| CR-004 | Language-specific profiles must declare their validation commands and applicability. | inspection | accepted |
