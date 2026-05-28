# Scenario 02: Rust CLI With Public Output Contract

## Situation

A Rust CLI emits JSON reports consumed by another repo.

## Expected VTRACE Emphasis

- Mission
- CONOPS
- Requirements
- Architecture
- Interfaces for CLI flags and JSON schema
- Design for report generation
- Code rigor with Rust tailoring
- Verification by `cargo fmt`, `cargo clippy`, `cargo test`, schema fixtures
- Validation by consumer workflow
- Readiness review before downstream adoption

## Code Rigor

Required. The repo should define constraints for:

- no uncontrolled `unsafe`,
- no recursion in critical report traversal unless bounded,
- no panics in library/report-generation paths,
- clean warnings and clippy,
- focused function sizes for critical logic,
- property or fixture tests for JSON compatibility.

## Expected Decision

`blocked` if the JSON interface lacks schema fixtures or if downstream claims
exist without validation evidence.

`pass_with_risk` if schema fixtures pass but real downstream validation is still
pending.

## Model Check

The process should put Rust code constraints on the left side before the CLI
implementation is changed, then require right-side evidence from tools and
fixtures.
