# Requirements

## Scope

Repo: VTRACE

## Requirement Table

| ID | Requirement | Parent Need / Scenario | Rationale | Priority | Owner | Verification Method | Status |
|---|---|---|---|---|---|---|---|
| REQ-001 | VTRACE shall define a source-grounded V-model process for software repos. | NEED-001 / NEED-002 | Establishes the operating model. | must | VTRACE | inspection | accepted |
| REQ-002 | VTRACE shall provide adoption templates for each required VTRACE stage. | NEED-001 / CON-001 | Enables repeatable repo adoption. | must | VTRACE | inspection | accepted |
| REQ-003 | VTRACE shall provide skills for assessment, adoption, and gate review. | NEED-003 / CON-002 / CON-003 | Enables agents to apply the process. | must | VTRACE | inspection | accepted |
| REQ-004 | VTRACE shall require specification baselines before non-trivial implementation planning. | NEED-001 / CON-001 | Prevents random implementation slices. | must | VTRACE | inspection | accepted |
| REQ-005 | VTRACE shall map implementation work to work packages with parent IDs, V closure, L0/L1/L2 checks, and review lanes. | NEED-001 / CON-002 | Converts files into procedural execution. | must | VTRACE | inspection | accepted |
| REQ-006 | VTRACE shall define future change requests for missing validators, language profiles, evidence ledgers, gate checklists, and realistic adoption scenarios. | NEED-001 / NEED-003 | Makes remaining gaps explicit. | should | VTRACE | inspection | accepted |

## Deferred Requirements

| ID | Reason Deferred | Revisit Trigger |
|---|---|---|
| REQ-VAL-001 | Automated validation tooling is planned but not yet implemented. | Start `DCR-001`. |
| REQ-PROFILE-001 | Language/package profiles need evidence from real target repos. | Start `DCR-002`. |
