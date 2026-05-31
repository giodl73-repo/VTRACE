# Gate Checklists

Gate checklists make VTRACE review decisions repeatable. They are not a second
review system; they are the decision record for the existing VTRACE gates.

## Required Gates

| Gate | When | Minimum Output |
|---|---|---|
| Specification Review | Before accepting requirements/specs for non-trivial work. | Requirements, specs, unknowns, validation path, and role lanes are clear. |
| Design Review | Before locking architecture, interfaces, package boundaries, or code-rigor constraints. | Boundaries, interfaces, tradeoffs, and verification approach are acceptable. |
| Implementation Readiness Review | Before starting controlled work packages. | Work packages have parents, entry/exit criteria, profiles, and L0/L1/L2 checks. |
| Work Package Close Review | Before marking a `WP-*` complete. | Implementation, evidence, trace, validation impact, and role lanes are closed. |
| Test Readiness Review | Before formal verification/validation or release evidence run. | Procedures, fixtures, expected results, and environment assumptions are ready. |
| Release/Transition Readiness Review | Before merge, release, adoption, or public readiness claim. | Evidence supports the claim and open risks are accepted or blocked. |

## Checklist Rules

- Required checklist rows cannot remain `pending` or `blocked` at a passing
  gate.
- `pass_with_risk` is acceptable only when the risk and follow-up are explicit.
- A missing checklist is acceptable for very small adoption slices, but a
  checklist is required for VTRACE self-trace and high-risk target repos.
