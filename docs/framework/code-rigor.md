# Code Rigor

Code rigor is the VTRACE bridge between detailed design and component
verification. It converts "write good code" into explicit, reviewable
constraints before implementation begins.

## Placement In The V

```text
Left side:
  Requirements -> Architecture -> Interfaces -> Detailed Design -> Code Rigor

Bottom:
  Implementation

Right side:
  Code Rigor Verification -> Component Verification -> Integration/System Verification
```

The left side defines the coding constraints. The right side proves that code
met them.

## Source Basis

VTRACE uses NASA Software Engineering Handbook coding-standard guidance and the
NASA/JPL Power of Ten rule set as source inputs. It does not claim every repo is
safety-critical or must follow the Power of Ten literally.

For ordinary portfolio repos, treat these as verifiability principles:

- keep code units small enough to review,
- keep control flow simple enough to analyze,
- bound or justify complex behavior,
- make error handling explicit,
- use assertions or invariants around critical assumptions,
- use static analysis, linting, compiler warnings, and tests as evidence.

## Left-Side Requirements

Add code-rigor requirements when implementation correctness matters. Example:

| ID | Requirement | Verification |
|---|---|---|
| REQ-CODE-001 | Critical implementation units should be small enough for focused review. | Code review plus size/complexity check. |
| REQ-CODE-002 | Critical control flow should be bounded or justified. | Design inspection plus targeted tests. |
| REQ-CODE-003 | Public interfaces should handle invalid inputs and errors explicitly. | Interface tests and review. |
| REQ-CODE-004 | Critical invariants should have assertions, checks, or property tests. | Unit/property tests and inspection. |
| REQ-CODE-005 | Static analysis, lint, and compiler warnings should be clean or waived. | Tool output and waiver ledger. |

## Right-Side Evidence

Code-rigor verification can include:

- lint or formatter output,
- compiler warning policy,
- static analysis reports,
- complexity report,
- focused code review,
- unit tests,
- property tests,
- invariant/assertion tests,
- waiver records.

## Tailoring

Avoid applying safety-critical C rules blindly to every repo. Tailor by:

- language,
- repo risk,
- public API stability,
- generated vs hand-authored code,
- performance needs,
- existing toolchain,
- review cost.

Every exception should include rationale and evidence. For example, a parser may
need a larger generated function, but the evidence should shift to generated
source provenance, grammar tests, and fixture coverage.
