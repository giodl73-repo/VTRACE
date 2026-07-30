# Product-first delivery and infrastructure budget

## Engineering decision

VTRACE adopters must treat project-management code as a constrained exception.
The default is to reuse existing portfolio or ecosystem capability. A product
repo should spend its implementation budget on behavior that a user can
exercise, observe, or depend upon.

This rule prevents traceability, automation, agent coordination, review, and
reporting machinery from becoming a shadow product.

## The 100-line tripwire

A coherent capability that reaches **100 handwritten executable lines** must be
classified before more code is added. The threshold is a review tripwire, not a
target and not permission to create 99-line tools casually.

Count the capability across files, scripts, embedded CI code, source, and tests;
do not evade the rule by splitting files. Exclude blank lines, comments,
generated or vendored code, lockfiles, and purely declarative configuration.

Beyond the tripwire, only two destinations are permitted:

1. **Product feature package or crate.** The code implements named,
   user-observable behavior owned by a product requirement. It has an
   executable example or acceptance scenario and lives in a package or crate
   dedicated to that feature or a coherent product domain.
2. **Shared infrastructure package.** The capability is not product behavior,
   cannot be satisfied by an existing tool, has a stable interface, lives in a
   shared tools or standards repository, and names at least one second adopter
   before implementation continues.

A product repo must not grow a project-management crate, workflow engine,
agent supervisor, evidence database, report generator, or bespoke test
orchestrator beyond this threshold merely to manage itself.

## Reuse-before-build order

Before implementing coordination or assurance infrastructure, record the first
applicable choice:

1. Use an existing command, library, GitHub capability, or portfolio tool.
2. Configure or compose that capability declaratively.
3. Add no more than 100 lines of disposable repo-local glue.
4. Extend an existing shared tool for multiple adopters.
5. Create a new shared tool only with a named second adopter and a versioned
   interface.

“Not invented here,” convenience, or a desire for a cleaner abstraction is not
sufficient justification to move down the list.

## Product-feature test

Code is product-bearing only when all of these are true:

- its requirement describes behavior or an outcome for an intended user;
- a user, operator, or downstream product can exercise the behavior;
- an acceptance scenario demonstrates a meaningful before/after result;
- the README can explain the result without referring first to VTRACE, work
  packages, agents, schemas, review machinery, or test counts; and
- removing the code would remove product capability, not merely make project
  administration less convenient.

If these conditions fail, classify the code as infrastructure.

## Local glue allowance

Repo-local project glue below the tripwire is acceptable only when it:

- delegates substantive behavior to an existing tool;
- contains no product or policy logic;
- has one clear owner and invocation point;
- is cheaper to delete than to maintain as a product; and
- names the shared replacement or deletion condition when one is foreseeable.

The allowance is a ceiling, not an expectation.

## Outcome-first work-package gate

Before an implementation work package enters:

1. State the consequential user question in one sentence.
2. Name the authoritative source or clearly synthetic scenario.
3. State the observable result the slice will produce.
4. Name the shortest command, example, or interaction that demonstrates it.
5. Estimate new product code and new infrastructure code separately.
6. Complete a reuse search for any proposed infrastructure.
7. Identify where the result will appear in the README or product guide.

A package that cannot answer the first four items is discovery or
infrastructure work, not a product-feature package.

After every two implementation packages, stop and show the integrated result.
Do not authorize another infrastructure package until the product demonstration
is understandable and the previous infrastructure has a current consumer.

## Shared-infrastructure admission

New shared infrastructure requires:

- a capability gap showing why existing tools are insufficient;
- one primary and at least one named second adopter;
- a versioned public contract;
- a minimal integration example for both adopters;
- an owner and maintenance boundary;
- evidence that the shared tool removes, rather than duplicates, local code;
  and
- an archive or consolidation decision if the second adoption does not occur
  within two portfolio waves.

A hypothetical future consumer does not satisfy the second-adopter rule.

## Exceptions

Safety, security, legal, migration, and incident-response work may temporarily
cross the tripwire without becoming a product feature. The exception must be
recorded as a time-bounded decision with:

- the risk that requires it;
- why reuse is unavailable;
- the maximum scope;
- the deletion, migration, or sharing date; and
- the reviewer accepting the maintenance cost.

An exception cannot silently become permanent infrastructure.

## Required trace links

A product-feature package links:

```text
NEED -> REQ -> product package/crate -> acceptance scenario -> evidence
```

A shared-infrastructure package links:

```text
capability gap -> reuse search -> shared contract -> adopter A -> adopter B
  -> integration evidence
```

Project-management code without one of these trace shapes is held.

## Evidence produced

The implementation-readiness or work-package record should contain:

- classification: `product-feature`, `local-glue`,
  `shared-infrastructure`, or `time-bounded-exception`;
- estimated and actual handwritten executable lines;
- reuse candidates considered and dispositioned;
- feature crate/package or shared-tool owner;
- product demonstration or two-adopter evidence; and
- deletion, consolidation, or exception review date where applicable.

## Validation

For each affected repository:

1. Inspect the changed executable files and count the coherent capability, not
   isolated files.
2. Confirm every capability at or above the tripwire has an allowed
   classification and trace shape.
3. Run the product acceptance scenario before framework validation.
4. Run the repository's normal verification commands.
5. Confirm the README describes the resulting product behavior.
6. Run `git diff --check`.

The review fails when project-management code is split to evade the threshold,
when a shared tool has only hypothetical adopters, or when infrastructure is
presented as a product feature.

