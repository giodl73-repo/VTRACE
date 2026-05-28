# Scenario 04: High-Risk Control Logic

## Situation

A repo implements control logic where failure could cause safety, financial, or
operational harm.

## Expected VTRACE Emphasis

- Mission and hazard/impact context
- Full CONOPS including failure and degraded modes
- Requirements with risk classification
- Architecture and interface review
- Detailed design before implementation
- Strict code rigor
- Verification by tests, static analysis, inspections, property tests, and
  negative fixtures
- Validation by realistic acceptance scenarios and operator review
- Readiness review with critical findings blocking claims

## Code Rigor

Required and stricter than default:

- recursion forbidden unless formally bounded,
- dynamic allocation in critical runtime path justified or avoided,
- `unsafe` forbidden unless isolated and independently reviewed,
- explicit error handling,
- assertions or invariant checks,
- clean warnings/static analysis,
- branch/complexity limits.

## Expected Decision

`blocked` until verification and validation evidence both exist for critical
requirements.

## Model Check

The process should prevent a high-risk repo from passing on unit tests alone.
