# Evidence Ledger

| Evidence ID | Type | Source / Command | Expected Result | Actual Result | Status |
|---|---|---|---|---|---|
| EVID-001 | command | `py src\report.py` | `actuator: ready` | `actuator: ready` | passed |
| EVID-002 | inspection | `src/report.py` | `strip()` is applied to item and state. | `format_status` strips both fields. | passed |
