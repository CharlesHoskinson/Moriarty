---
title: "APSS permission: tutorial"
diataxis: tutorial
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# Tutorial: derive a bounded payment permission

This is a guided paper exercise. You will build and challenge a small permission envelope. It uses hypothetical exact-integer token units and a fictional ledger, not runnable Moriarty syntax, deployed Midnight behavior or a real signing scheme. No wallet or funds are needed.

By the end, you will have a grant, two valid uses and rejection cases that distinguish developer freedom from owner authority. The concepts come from structured authorizations, contextual delegation and explicit replay handling. [P07](https://www.rfc-editor.org/rfc/rfc9396.html#section-2), [P09](https://research.google.com/pubs/archive/41892.pdf), [P02](https://eips.ethereum.org/EIPS/eip-712).

## 1. Write Alice's grant

Alice owns TOKEN-X. Bob must be the recipient of any payment under this grant. Alice authorizes an agent to make up to two separate payments before fictional ledger time 500.

Copy this conceptual record:

```text
grant: G1
owner: Alice
agent: Agent-K
program_semantics: PaymentProfile-v1
domain: DemoLedger / Contract-C
asset: TOKEN-X, integer base units
allowed invoice slots: I1, I2 (each usable once)
recipient: Bob
minimum Bob credit per used slot: 45
maximum fee per used slot: 2
maximum cumulative gross debit: 100
new liabilities: forbidden
expiry: ledger time < 500
revocation epoch: 7
```

Assume the profile fixes an authorized fee recipient and has no hidden effects or other charges. In a real design, those assumptions must be encoded and checked. The exact signed representation is not specified in this exercise.

Observe that no field asks which developer wrote the program or which reviewer approved it. Alice's assets remain protected because every attempted use must satisfy her grant.

## 2. Check the first payment

At time 100, the agent proposes:

```text
slot: I1
Bob credit: 50
fee: 2
Alice gross debit: 52
new debt: 0
```

Compute `50 + 2 = 52`. Bob receives at least 45, the fee is at most 2, and cumulative debit becomes 52, below 100. If signature, domain, current epoch, slot freshness and all other profile conditions hold, this is an authorized transition in the exercise.

Record the resulting state:

```text
consumed slots: {I1}
cumulative debit: 52
remaining gross authority: 48
```

A signature alone did not perform this accounting. That distinction is the point of EIP-712's explicit exclusion of replay protection. [P02](https://eips.ethereum.org/EIPS/eip-712).

## 3. Check the second payment

At time 110, propose `I2`, Bob credit 46, fee 2, gross debit 48. Compute `52 + 48 = 100`. The proposal satisfies the remaining budget and the per-payment minimum. Record both slots consumed and zero remaining authority.

Now change Bob credit to 47 while retaining fee 2. The gross debit becomes 49 and cumulative debit 101. Reject it, even though each amount looks individually small and the agent has a valid key.

## 4. Try replay and concurrent delegation

Resubmit the first payment with the original signed bytes. Reject it because I1 is consumed; changing only an outer transport request ID must not make it fresh.

Reset to the original exercise state. Give one child permission only I1 with maximum 52 and another only I2 with maximum 48. These children preserve the common limit when their scopes and semantics are enforced. Try instead giving two children limits of 60 without shared consumption checks: they could each pass a local 60-unit test while jointly spending 120. Write down why a shared root identity or a sound budget partition is necessary.

Do not resolve this by asking a maintainer to approve each payment. Resolve it in the authority relation and durable ledger state. The exercise borrows contextual narrowing from P09; it does not adopt its HMAC credential format. [P09](https://research.google.com/pubs/archive/41892.pdf).

## 5. Add revocation

Reset again. Accept the first 52-unit payment, then confirm Alice's revocation advancing epoch 7 to 8. A later use of the epoch-7 grant fails. Bob keeps the already accepted 50-unit credit, and its fee remains incurred. The revocation does not restore spent funds or erase an unrelated debt.

Now put the second payment and revocation in a race. Specify two ordered histories: payment then revocation, and revocation then payment. They can have different valid outcomes because current authority is state-dependent. A wallet's local “revoked” label is not the ledger ordering. [P03](https://eips.ethereum.org/EIPS/eip-1271).

## 6. Separate observation, signing and settlement

Give Auditor-V permission to view the exercise's payment history. Do not give Auditor-V Agent-K's spend authority. Suppose two signers jointly create a threshold signature for an allowed payment: record “signature produced,” not “Bob paid.” Submission, verification and finalized effects remain separate events. Zcash's key components and FROST's application-specific validation discussion illustrate why these distinctions matter. [P10 pp.14–15](https://zips.z.cash/protocol/protocol.pdf), [P12 §7.7](https://www.rfc-editor.org/rfc/rfc9591.html#section-7.7).

You have completed the exercise when you can explain all four facts: anyone may develop the payment program; only Alice's applicable grant authorizes her assets; authorization must account for current state and cumulative use; and possession of a view key or a produced signature is not proof of a finalized payment.


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
