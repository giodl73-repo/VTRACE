# Specification Baseline

| Spec ID | Parent REQ IDs | Type | Current / Target / Deprecated / Unknown | Specification Statement | Verification Method | Validation Method | Owner | Risk | Status |
|---|---|---|---|---|---|---|---|---|---|
| SPEC-001 | REQ-001 | product | current | Output uses the exact format `<item>: <state>`. | demonstration | migration scenario | VTRACE | low | accepted |
| SPEC-002 | REQ-002 | software | target | Leading and trailing whitespace is stripped from both fields before formatting. | demonstration | migration scenario | VTRACE | low | accepted |

## Requirement-To-Spec Coverage

| Requirement ID | Spec IDs | Coverage Status | Notes |
|---|---|---|---|
| REQ-001 | SPEC-001 | covered | Existing behavior preserved. |
| REQ-002 | SPEC-002 | covered | Target behavior implemented. |
