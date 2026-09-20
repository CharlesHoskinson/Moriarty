---
id: apss.applications.tutorial
title: Separate an application's rule from a user's intent
status: draft
documentation_type: tutorial
---

# Separate an application's rule from a user's intent

This is a conceptual paper-and-pencil exercise. It is not `.mori` syntax, executable software, a cryptographic proof or a deployed application. You will examine three proposed trades and see why program correctness and user intent need separate predicates.

## 1. Set up a small exchange

Write these initial pool reserves in exact integer units:

```text
Pool A = 1,000
Pool B = 2,000
Pool rule: the final reserve product must not be smaller than 2,000,000.
```

The reserve-product rule is inspired by the technical example in [APP10](notes/APP10.md). We intentionally omit real pricing, fee and liquidity-provider mechanics. These numbers describe a learning model only.

## 2. Write Alice's intent

Alice permits the following outcome:

```text
Asset A and asset B have distinct fixed identities.
At most 100 A may leave Alice in total, including any fee in A.
Only this pool and the named fee collector may receive that A.
Alice must receive at least 170 B net.
No new debt may be created.
This authorization may be consumed only once.
```

Notice that Alice has not specified how a proposal engine finds a trade. Her limits are conditions on the result and authority used.

## 3. Check a candidate that satisfies both

Candidate One sends 99 A to the pool, pays a 1 A fee to the named collector and sends 180 B from the pool to Alice.

Calculate:

```text
Final pool: 1,099 A and 1,820 B
Product: 1,099 × 1,820 = 2,000,180
Alice gross A debit: 99 + 1 = 100
Alice net B credit: 180
```

The pool rule and these numeric user constraints pass. This does **not** establish a real valid transaction: the actual system must also check signatures, funding, exact asset identities, time, history, proof validity and ledger consumption.

## 4. Change only the recipient

Candidate Two makes the same pool movements but sends 180 B to Mallory.

The reserve product remains 2,000,180. Alice receives no B. Mark the application arithmetic check as passing and the user-outcome check as failing. A proof of only the pool invariant would be insufficient.

## 5. Try a refund trick

Candidate Three sends 110 A from Alice to the pool, returns 11 A from the pool to Alice, charges a 1 A fee and sends 180 B to Alice.

The final pool is again 1,099 A and 1,820 B. Alice's net A decrease is 100, but her gross outgoing A is 111. Mark the signed gross-debit check as failing. A later refund does not authorize the earlier excess debit.

This is our constructed counterexample, not a behavior reproduced from CoW or Moriarty.

## 6. Name what a real proof would bind

Write a final list: program/specification identity, Alice's signed constraints, current predecessor, input assets, full effects, resulting state, authority consumption and required history. Add explicit finite arithmetic and work bounds. Keeping these bindings together prevents a solver from substituting a different recipient, program or predecessor after a useful calculation.

You have now separated a program rule, a user intent and a proposed execution. You have not needed a project administrator to approve Alice's application; objective proof and authorization checks still apply. For auditing an actual design, continue with the [how-to guide](how-to.md). For why asynchronous applications and privacy need further distinctions, read the [explanation](explanation.md).
