# S02 candidate A semantic intake

Status: independent read-only architectural advice. No implementation or model result.
The reviewed S02 designs and frozen `moriarty/core.py` and `swap.py` govern this work.

## Representation

Use a finite indexed constructor table and a complete agreement state.
Nodes represent `Close`, `Pay`, `If`, and `When`. Cases contain `Deposit` or `Choice` actions.
Each explicit successor index must be lower than its source. Node zero is `Close`.
Keep the complete node table in every structural artifact binding.
Reconstruct the recursive Python AST independently when comparing continuations.
Equal phase labels or node numbers alone do not establish correspondence.

Accounts form a total map over six principal/asset combinations.
Zero represents absence from the sparse Core state.
Choices form a total map into `NoInt | IntValue(int)`.
An absent choice is different from a choice of zero.
The agreement state retains accounts, choices, continuation, and minimum time.
The financial ledger remains separate and covers wallets and escrows.

Results retain accepted status, exact error, ordered warnings with requested/paid
values, ordered Core payments, complete agreement successor, and reduction count.
Complete transfer effects are separate from Core payments.
Every rejected transaction returns its entire original agreement state.
Rejection clears payments, warnings, complete effects, and reduction count.

## Candidate tables

All timeouts are 100 and target node zero. AA is Alice/TokenA, BB is Bob/TokenB.

| Swap node | Constructor |
| --- | --- |
| 0 | Close |
| 1 | Pay BB to Alice, 20, next 0 |
| 2 | Pay AA to Bob, 10, next 1 |
| 3 | If Settle equals 1, then 2, else 0 |
| 4 | When Bob Settle choice 0..1, next 3 |
| 5 | When Bob deposits 20 into BB, next 4 |
| 6 | When Alice deposits 10 into AA, next 5 |

Swap starts at node 6 with empty accounts and choices, minimum time zero.

| Installment node | Constructor |
| --- | --- |
| 0 | Close |
| 1 | Pay AA to Bob, 5, next 0 |
| 2 | When Bob Slot2 choice 1, next 1; or Alice Refund choice 0, next 0 |
| 3 | Pay AA to Bob, 5, next 2 |
| 4 | When Bob Slot1 choice 1, next 3; or Alice Refund choice 0, next 0 |

Installments start at node 4 with AA pre-funded to ten and other accounts zero.
The first payment stops at node 2, so the second requires a separate transaction.
Refund choices are legal Core transitions without cancellation state.
The later envelope must enforce cancellation and separate nonce-1 recovery authority.
Bob fill/Alice refund ownership and the new choice strings need explicit plan decisions.

## Exact transaction rules

1. Save the original complete agreement. Reject time below its minimum time.
2. Tentatively set minimum time to the supplied time.
3. Reduce to quiescence before input matching.
4. Without input, reject empty unchanged Close as `contract_closed`.
   Reject quiescent When as `input_required`. Otherwise accept reductions.
5. With input, reject quiescent Close as `contract_closed`.
6. At When, scan cases in order. Deposits match account, depositor, and quantity exactly.
   A matched nonpositive deposit rejects immediately.
7. A matching choice with invalid bounds records mismatch and continues scanning.
   A later matching valid case can succeed.
   If none succeeds, return `choice_out_of_bounds` after any bounds mismatch.
   Otherwise return `no_matching_input`.
8. Successful deposits credit the account and add one wallet-to-escrow transfer.
   They add no Core payment. Successful choices update the optional choice value.
9. Advance to the matched continuation and reduce again.
10. Concatenate before/input/after complete effects in order.
    Concatenate before/after Core payments and warnings without deposit payments.
11. Any rejection restores the original state and clears all observations.

Each Pay, If, timeout advance, or single-account Close refund counts one reduction.
Input matching counts zero. Close refunds the first positive account in pinned order.
Pay uses `min(max(requested, 0), balance)`.
Nonpositive requested amounts produce `non_positive_payment(requested, 0)`.
Positive underfunded requests produce `partial_payment(requested, paid)`.
Only positive payments transfer value.
An unset choice makes If false, even when the expected value is zero.

## Bounds and independent checks

The rank `continuation + positiveAccountCount` decreases on every reduction.
Six accounts give conservative per-phase bounds 12 for swap and 10 for installments.
Check quiescence after each bounded reduction fold.
An exceeded budget or invalid node table is a model diagnostic, not a Core error.
Never accept a truncated result.

Preserve funded deadline commit and supplied-input rollback as separate witnesses.
Also test a no-input Pay that stops at When and rolls back with `input_required`.
Canonical workloads cannot reach every warning or nonpositive-deposit error.
Use a separate finite semantic-test artifact catalog for those checks.
Never alter canonical tables to generate such cases.

A Core-accepted result can lack executable wallet funds.
Preserve Core acceptance and mark financial execution unavailable separately.
Do not invent an insufficient-wallet Core error.

The plan must pin choice strings/order, test/input/state catalogs, artifact mutations,
and source hashes. Times remain 1, 2, 100, 101, with initial minimum time zero.
Candidate A's interpreter cannot become B or D's execution implementation.
Core-result correspondence and complete-effect extraction need independent corruption checks.
