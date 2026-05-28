# Verification Plan

## Scope

Repo or feature: `examples/hello-world`

## Verification Matrix

| Requirement ID | Method | Command / Inspection | Expected Evidence | Result | Evidence Pointer |
|---|---|---|---|---|---|
| REQ-001 | demonstration | `python src\hello_world.py` | Expected output appears. | pass | EVID-001 |
| REQ-002 | inspection / static check | inspect `src\hello_world.py`; `python -m py_compile src\hello_world.py` | Small dependency-free source compiles. | pass | EVID-002 |
| REQ-CODE-001 | inspection / static check | inspect `CODE_RIGOR.md`; `python -m py_compile src\hello_world.py` | Code-rigor constraints are satisfied. | pass | EVID-CR-001, EVID-002 |

## Commands

```powershell
python src\hello_world.py
python -m py_compile src\hello_world.py
```

## Evidence Ledger

| Evidence ID | Type | Path / URL / Command | Covers | Result |
|---|---|---|---|---|
| EVID-001 | command output | `python src\hello_world.py` | REQ-001, VAL-001 | pass: `Hello, VTRACE!` |
| EVID-002 | static check | `python -m py_compile src\hello_world.py` | REQ-002, REQ-CODE-001, CR-005 | pass |
| EVID-CR-001 | inspection | `docs/vtrace/CODE_RIGOR.md` and `src/hello_world.py` | CR-001, CR-002, CR-003 | pass |

## Code Rigor Verification

| Constraint ID | Method | Command / Review | Result | Evidence Pointer |
|---|---|---|---|---|
| CR-001 | size/complexity inspection | inspect `src\hello_world.py` | pass | EVID-CR-001 |
| CR-005 | static syntax check | `python -m py_compile src\hello_world.py` | pass | EVID-002 |

## Gaps

| Gap | Impact | Disposition |
|---|---|---|
| Python availability not checked automatically | Operator without Python cannot run example. | accept risk |
