---
title: "APSS settlement: tutorial"
diataxis: tutorial
status: research-draft
created: 2026-09-19
updated: 2026-09-19
type: research
tags: [moriarty, apss, research]
---

# Tutorial: classify a payment that failed after paying a fee

This is a conceptual paper exercise. The identifiers and numbers are invented; no transaction, proof, simulator or Midnight deployment was executed. You will learn to distinguish a valid candidate, accepted ledger effects and successful financial fulfillment. This is not executable Moriarty syntax.

The exercise is motivated by the official [Midnight transaction-fallibility rules](https://docs.midnight.network/concepts/how-midnight-works/semantics#transaction-fallibility), which allow guaranteed effects to survive fallible-phase failure. The actual compiler phase mapping still needs verification.

## Step 1: write the intention

For this exercise only, use one abstract accounting unit for the payment and fee so the arithmetic stays visible. This is not a claim that Midnight network fees are paid in test asset A. A real intention must represent the actual fee resource and each asset separately, with any conversion bound explicitly authorized. Assume a user authorizes one payment attempt with these terms:

| Field | Conceptual value |
|---|---|
| Asset | Exact test asset A on one named Midnight deployment |
| Recipient | Creditor C |
| Payment on success | 97 atomic units |
| Maximum network fee | 3 units, charged to this intention |
| Maximum gross debit | 100 units |
| Liability discharged | 97 units only if the creditor payment completes |
| Fee-only failure | Explicitly permitted, at most 3 units; debt remains |
| Cumulative authority | One payment, no reuse after fulfillment; attempts share the stated cumulative budget |

A permissive routing choice does not allow asset B with the same display ticker. The user's signature commits to the exact asset and the fee-only failure policy.

## Step 2: separate candidate arithmetic from execution

The expression `97 + 3 <= 100` is true. That fact alone says nothing about whether C is paid, whether debt is discharged or whether a second attempt can reuse the authority. Write those as separate predicates.

For example, successful fulfillment requires both `credit(C, A) = 97` and the matching liability reduction. Every accepted outcome must satisfy the cumulative gross limit. A permitted fee-only partial result must leave the debt intact.

## Step 3: assign hypothetical phases

For learning only, suppose the backend places the fee in guaranteed execution and the creditor payment/debt update in fallible execution. We are not asserting that Moriarty currently emits this layout.

Now let the payment's state precondition become false before ledger execution. The proof might still establish the candidate's stated computation, but ledger application has to check the current state and phase rules.

## Step 4: classify four observations

| Observation | Classification | What remains to check |
|---|---|---|
| Guaranteed phase rejects | No included transaction under the documented rule | Retained rejection, no falsely attributed fee |
| Guaranteed fee succeeds; fallible payment fails | Partial ledger success, financial objective unfulfilled | Actual fee ≤3, no creditor credit, unchanged debt, remaining budget/authority |
| Fee and payment both succeed | Candidate for financial success | Finality, exact recipient/asset, debt discharge, complete effects and unique consumption |
| Broadcast occurred; acknowledgement lost | Unresolved | Reconcile ledger state before a retry |

For the partial result with fee 3, cumulative spending leaves only 97 of the original 100 gross allowance. A retry requiring another 3 fee plus 97 payment no longer fits. A refund or hoped-for compensation cannot erase the original gross debit. Obtain fresh appropriately bounded authorization or use a preauthorized alternative that fits; do not silently reset the budget.

## Step 5: reject two tempting conclusions

“A proof verified, so payment succeeded” ignores ledger application, phase outcome and finality. “The payment failed, so nothing changed” ignores the retained fee. Both descriptions mislead the user and can corrupt successor accounting.

An attestor's signature saying “paid” is also not a substitute for the required evidence policy. If the intention requires verified ledger effects, a proof that merely checks that signature does not meet it.

## Step 6: carry the lesson into an asynchronous application

Imagine an optional remote leg after the first ledger effect. A missing remote reply cannot prove that nothing happened. Keep the attempt unresolved, preserve residual debt and prevent duplicate authority consumption until an admissible receipt or non-execution proof arrives. Recovery can itself need a reachable chain and a witness.

You have completed the exercise when you can state which predicate failed, which effects survived, what budget remains, and what evidence permits the next transition. To audit real artifacts, continue with [how-to.md](how-to.md).


Evidence archive: [captured source manifest and reading records](evidence.md). Source claims and Moriarty proposals retain the stated evidence limits.
