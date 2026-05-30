# Trace Matrix

| Requirement ID | Parent Need | CONOPS | Requirement | Design Element | Code Rigor Constraint | Work Package | Implementation Surface | Verification Method | Validation Method | Evidence Pointer | Status |
|---|---|---|---|---|---|---|---|---|---|---|---|
| REQ-001 | NEED-001 | CON-001 | Print exactly `Hello, VTRACE!`. | ARCH-001, IF-001, DES-001 | n/a | WP-001 | `src/hello_world.py` | VER-001 demonstration | VAL-001 | EVID-001 | validated |
| REQ-002 | NEED-001 | CON-001 | Stay dependency-free and reviewable. | ARCH-001, DES-001 | CR-001, CR-003, CR-005 | WP-001 | `src/hello_world.py` | VER-002 inspection/static check | VAL-001 | EVID-002, EVID-CR-001 | verified |
| REQ-CODE-001 | REQ-002 | CON-001 | Satisfy code-rigor constraints. | DES-001 | CR-001, CR-002, CR-003, CR-005 | WP-001 | `src/hello_world.py` | VER-003 inspection/static check | VAL-001 | EVID-CR-001, EVID-002 | verified |

## ID Legend

- NEED-001: mission need in `MISSION.md`.
- CON-001: run-program scenario in `CONOPS.md`.
- ARCH-001: one-script architecture in `ARCHITECTURE.md`.
- IF-001: standard output interface in `INTERFACES.md`.
- DES-001: `message()` plus `main()` design in `DESIGN.md`.
- CR-* constraints: `CODE_RIGOR.md`.
- WP-001: implementation package in `WORK_PACKAGES.md`.
- EVID-* evidence: `VERIFICATION.md`.
