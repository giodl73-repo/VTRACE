# Fixture Model

## Scope

Reusable VTRACE template for controlled fixtures.

Fixtures bridge specs and scenarios to implementation evidence. They are not
scratch examples. They are controlled proof inputs that show how behavior should
succeed, fail, render diagnostics, emit evidence, and remain compatible.

## Fixture Authority

Record fixture authority for this adoption:

| Surface | Authority |
|---|---|
| Scenario workspaces |  |
| Product specs |  |
| Fixture root |  |
| Test/verification commands |  |
| Evidence ledger |  |
| Docs examples |  |

## Fixture Classes

| Class | Purpose | Promotion gate |
|---|---|---|
| Golden fixture | Proves the expected happy path. | Product spec, expected outputs, verification command. |
| Negative fixture | Proves a stable failure mode and diagnostic. | Diagnostic allocation, expected error, no hidden success path. |
| Adversarial fixture | Probes custody, policy, trust, path escape, stale state, or secret exposure. | Security/custody review and expected block. |
| Regression fixture | Captures a previously found issue or scenario finding. | Finding ID, before/after behavior, stable assertion. |
| Compatibility fixture | Proves schema/profile/descriptor compatibility or migration. | Version matrix and migration expectation. |
| Corpus fixture | Represents corpus, artifact profile, source custody, capture, or scorecard records. | Contract descriptor and custody profile. |
| Inspect/report fixture | Proves state, diagnostics, evidence, or missing evidence are visible. | Expected machine-readable or human-readable envelope. |
| Docs fixture | Supports a user-facing docs example. | Evidence pointer and claim boundary. |

## Required Fixture Shape

Every promoted fixture must define:

1. fixture ID,
2. owning work package,
3. linked specs,
4. linked scenario or finding source,
5. fixture class,
6. input files and path/address references,
7. expected command or inspection,
8. expected outputs,
9. expected diagnostics,
10. expected evidence/report state,
11. custody/redaction behavior,
12. cache or stale-state behavior when relevant,
13. docs/corpus impact,
14. update rule when specs change.

## Promotion Rule

A scenario artifact becomes a fixture only when:

1. the product spec is accepted for the behavior,
2. positive or negative expected result is explicit,
3. diagnostics are allocated or queued,
4. path/address references are canonical when used,
5. descriptor/schema impact is recorded for durable records,
6. fixture ownership is named in a work package,
7. verification command is known,
8. stale or secret-bearing data has custody/redaction rules.