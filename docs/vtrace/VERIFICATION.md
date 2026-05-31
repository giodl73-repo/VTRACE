# Verification

## Scope

Repo: VTRACE

## Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Result | Status | Evidence |
|---|---|---|---|---|---|
| REQ-001 | inspection | Inspect `docs/framework/vtrace-process.md` | V-model process and stage rules exist. | passed | EVID-001 |
| REQ-002 | inspection | Inspect `templates/adoption/` | Required adoption templates exist. | passed | EVID-002 |
| REQ-003 | inspection | Inspect `skills/vtrace-*` | Assessment, adoption, and gate skills exist. | passed | EVID-003 |
| REQ-004 | inspection | Inspect spec baseline guidance and template | Spec baseline required before non-trivial implementation. | passed | EVID-004 |
| REQ-005 | inspection | Inspect implementation management and work-package template | Work packages require parent IDs, V closure, validation levels, and role lanes. | passed | EVID-005 |
| REQ-006 | inspection | Inspect `CHANGE_CONTROL.md` and `WORK_PACKAGES.md` | DCRs map to future work packages. | passed | EVID-006 |
| REQ-VAL-001 | automated test / local command | `py -m unittest discover -s tests -p "test_*.py"`; `py tools\vtrace_check.py .` | Validator tests pass and VTRACE self-package validates. | passed | EVID-013, EVID-014 |

## Validation Commands

| Level | Command | Status |
|---|---|---|
| L0 | `git diff --check` | passed |
| L1 | `py -m json.tool sources\source-registry.json` | passed |
| L1 | `py -m py_compile examples\hello-world\src\hello_world.py` | passed |
| L1 | `py examples\hello-world\src\hello_world.py` | passed |
| L1 | `py -m unittest discover -s tests -p "test_*.py"` | passed |
| L1 | `py tools\vtrace_check.py .` | passed |

## Evidence Ledger

| Evidence ID | Type | Pointer | Result |
|---|---|---|---|
| EVID-001 | inspection | `docs/framework/vtrace-process.md` | passed |
| EVID-002 | inspection | `templates/adoption/` | passed |
| EVID-003 | inspection | `skills/vtrace-assess`, `skills/vtrace-adopt`, `skills/vtrace-gate` | passed |
| EVID-004 | inspection | `docs/framework/specification-baselines.md`, `templates/adoption/SPECIFICATION_BASELINE.md` | passed |
| EVID-005 | inspection | `docs/framework/implementation-management.md`, `templates/adoption/WORK_PACKAGES.md` | passed |
| EVID-006 | inspection | `docs/vtrace/CHANGE_CONTROL.md`, `docs/vtrace/WORK_PACKAGES.md` | passed |
| EVID-007 | source custody | `docs/vtrace/SOURCE_BASIS.md`, `sources/source-registry.json` | passed |
| EVID-008 | command | `git diff --check` | passed |
| EVID-009 | command | `py -m json.tool sources\source-registry.json` | passed |
| EVID-010 | command | `py -m py_compile examples\hello-world\src\hello_world.py`; `py examples\hello-world\src\hello_world.py` | passed |
| EVID-013 | command | `py -m unittest discover -s tests -p "test_*.py"` | passed |
| EVID-014 | command | `py tools\vtrace_check.py .` | passed |
