# Scenario 03: Generated Code Or Parser

## Situation

A repo generates code from a grammar or schema. Generated files may be large and
violate normal function-size expectations.

## Expected VTRACE Emphasis

- Requirements for generated artifact correctness
- Interface control for grammar/schema input and generated output
- Detailed design for generator invariants
- Code rigor tailoring for generated vs hand-authored code
- Verification by golden fixtures, round-trip tests, and generator provenance
- Review gate that accepts justified size exceptions

## Code Rigor

Required with tailoring:

- hand-authored generator code follows size/complexity constraints,
- generated code can exceed size thresholds,
- generated code requires provenance, deterministic regeneration, and fixture
  evidence.

## Expected Decision

`pass_with_risk` if generated code has deterministic regeneration and fixtures
but no independent consumer validation yet.

`blocked` if generated code is committed without provenance or regeneration
steps.

## Model Check

The process should allow justified exceptions without weakening traceability.
