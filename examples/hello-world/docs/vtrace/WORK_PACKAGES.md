# Work Packages

## Scope

Repo or feature: `examples/hello-world`

## Work Package Table

| ID | Objective | Parent IDs | Affected Surfaces | Entry Criteria | Exit Criteria | Verification | Status |
|---|---|---|---|---|---|---|---|
| WP-001 | Implement and verify the hello-world example. | REQ-001 / REQ-002 / REQ-CODE-001 / DES-001 / IF-001 / CR-001 / CR-005 | `src/hello_world.py`, `docs/vtrace/*` | Baseline artifacts accepted. | Program runs, compiles, trace and review updated. | `python src\hello_world.py`; `python -m py_compile src\hello_world.py` | complete |

## Work Package Details

### WP-001: Implement and verify the hello-world example

Objective: provide one runnable end-to-end VTRACE example.

Parent requirement IDs: REQ-001, REQ-002, REQ-CODE-001.

Design/interface/code-rigor IDs: DES-001, IF-001, CR-001, CR-002, CR-003, CR-005.

Validation scenario IDs: VAL-001.

Affected files/modules:

- `src/hello_world.py`
- `docs/vtrace/*`

Entry criteria:

- Requirements are accepted.
- Design and interface are accepted.
- Code-rigor constraints are accepted.

Exit criteria:

- Program prints expected output.
- Static compile check passes.
- Trace matrix includes WP-001.
- Review decision is recorded.

Verification commands:

```powershell
python src\hello_world.py
python -m py_compile src\hello_world.py
```

Validation levels:

| Level | Required | Commands / Evidence | Result |
|---|---|---|---|
| L0 | yes | `python src\hello_world.py`; `python -m py_compile src\hello_world.py` | pass |
| L1 | no | Same as L0 for this tiny example. | n/a |
| L2 | no | Readiness review only; no integration surface exists. | n/a |

V closure:

| V Area | IDs / Evidence | Status | Notes |
|---|---|---|---|
| Need / CONOPS | NEED-001, CON-001 | closed | Tiny run-program scenario. |
| Requirements | REQ-001, REQ-002, REQ-CODE-001 | closed | All assigned to WP-001. |
| Architecture / Interface | ARCH-001, IF-001 | closed | One-script architecture and stdout contract. |
| Design / Code Rigor | DES-001, CR-001, CR-002, CR-003, CR-005 | closed | Low-risk Python tailoring. |
| Implementation | `src/hello_world.py` | closed | One implementation file. |
| Verification | EVID-001, EVID-002, EVID-CR-001 | closed | Run, compile, and inspection evidence. |
| Validation | VAL-001 | closed | Adopter can inspect full trace chain. |
| Trace | `TRACE.md` rows include WP-001 | closed | No orphan accepted requirements. |
| Gate | `REVIEW.md` decision `pass_with_risk` | closed | Scale risk accepted. |

Validation impact: supports VAL-001.

Risks: Python availability remains an accepted environment assumption.

Review gate: readiness review in `REVIEW.md`.

Status: complete.

## Orphan Check

Before implementation starts, confirm:

- [x] Every accepted `REQ-*` is assigned to a work package or dispositioned.
- [x] Every interface-changing work package names `IF-*` IDs.
- [x] Every critical-code work package names `CR-*` IDs.
- [x] Every work package has exit criteria and verification commands.
- [x] Every work package has V closure rows completed or marked `n/a` with rationale.
- [x] No work package is only "cleanup" without parent IDs or discovery status.
