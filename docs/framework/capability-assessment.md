# Evidence-backed capability assessment

## Decision supported

This profile supports one decision: whether a named repository program,
current system, or intervention candidate has enough evidence to make a
specific capability claim and proceed to its next review gate.

It does not select policy, authorize delivery, approve spending, infer savings,
or replace domain review. A score summarizes evidence maturity; it never
overrides a failed safety, rights, continuity, or access floor.

## Scored-object rule

Every assessment names exactly one scored object and one version or observation
date. Keep these objects separate:

- **program capability** — what the repository can model, test, and govern;
- **current-system performance** — what the operating system achieves now; and
- **candidate performance** — what one bounded intervention is expected or
  observed to change.

Evidence about one object cannot silently score another. In particular, a
capable repository is not evidence that the current system works or that a
candidate should be adopted.

## Maturity scale

| Score | Meaning | Minimum evidence posture |
|---:|---|---|
| 0 | Missing. | No usable contract or evidence. |
| 1 | Designed or partially evidenced. | Named mechanism, source, or gap; material execution evidence is absent. |
| 2 | Executable and bounded. | Reproducible machinery and material evidence exist, but an outcome, price, delivery, or real-world gate remains held. |
| 3 | Demonstrated. | Current observed evidence supports the claim for the named object, perimeter, and time. |

Do not average uncertainty away. Each dimension records its score, evidence
pointers, demonstrated strength, and principal hold. Missing values remain
missing rather than becoming zero or a synthetic pass.

## VERDICT profile

VERDICT is the first reusable dimension profile:

| ID | Dimension | Assessment question |
|---|---|---|
| V | Value | Is whole-system lifecycle price competitive per useful outcome? |
| E | Effectiveness | Does the named object measurably improve the intended service? |
| R | Resilience | Can service continue, recover, and perform under relevant stress? |
| D | Deliverability | Are authority, capacity, workforce, procurement, transition, and timing credible? |
| I | Iteration | Can the responsible system observe, learn, adjust, scale, roll back, or retire? |
| C | Coverage and fair access | Who is reached, excluded, delayed, or burdened? |
| T | Trust | Are sources, assumptions, accountability, custody, and correction reviewable? |

The profile is optional. Repositories may use another dimension set when their
mission requires it, but must preserve the scored-object, evidence, floor, and
claim-boundary rules.

## Iteration distinction

Record which loop the evidence demonstrates:

1. analytical refresh — the model or corpus updates;
2. operational response — the service system changes;
3. outcome learning — post-change service and burden are measured; and
4. fiscal rebalancing — cost, financing, and allocation are reconsidered.

An analytical refresh alone cannot receive demonstrated maturity for the full
operational learning loop.

## Trace and evidence contract

An assessment records:

- assessment ID, date, assessor, object class, object ID, and exact version;
- dimension profile and scale version;
- one row per dimension with score, evidence pointers, strength, and hold;
- applicable hard floors and their pass, fail, or unresolved state;
- total as a descriptive rollup only;
- allowed and blocked claims;
- the next evidence-producing action; and
- validation commands and results.

Expected trace links are mission or stakeholder need, requirement or service
promise, candidate or system boundary, evidence record, review decision, and
next gate. The assessment produces a reviewable scorecard, not implementation
authority.

## Validation

For a repo-local adoption, reviewers should verify that every score has an
evidence pointer, totals reproduce arithmetically, exact versions are named,
failed or unresolved hard floors block promotion, and claim boundaries are
explicit. Run the target repository's normal validation plus:

```powershell
git diff --check
```

VTRACE intentionally provides no shared runtime crate for this profile. Add a
validator only after repeated repo-local use proves stable common semantics.
