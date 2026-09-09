# Independent repayment branch expectations

Specification-derived expectations, authored September 9, 2026 by the independent
GPT-6 fixture author. Status at freeze: **specified-only**. No evaluator, source
preparation, codec, or K execution was used to derive these expected results.
The existing first fixture was inspected only to establish the JSON record shape.
The 16 existing fixtures remain unchanged.

The deliverable is
[`branches.json`](../../experiments/moriarty-language/formal/k/fixtures/branches.json):
exactly 16 complete `{id, input, expected}` records, with five `Prepared` and eleven
`Rejected` expectations. Each input has two balance rows, one allowance, one
obligation, empty used-ID lists, identity conversion, and exactly `Transfer` then
`Repay`. Missing receivers can produce a third post-state balance row. Amounts
and counters are decimal strings. These cases stay within the existing bounded
`moriarty-funded-repayment/0` projection; they do not extend its admission domain.

The normative repository sources for these expectations are
[`repayment-kernel.md`](../../experiments/moriarty-language/spec/successor/repayment-kernel.md),
particularly Input closed shape, Transition semantics, and Stable emitted error
codes, and
[`funded-source.md`](../../experiments/moriarty-language/spec/successor/funded-source.md),
particularly Financial behavior and rejection precedence. This document records
specification-derived arithmetic and expected behavior, not execution evidence,
proof, source/K correspondence, authorization, or ledger acceptance.

## Common input and complete success accounting

Unless a case explicitly changes a field, the input has Payer/Cash 73 and
Lender/Cash 11, allowance Payer/Cash with remaining 61 and spent 7, and work
remaining 9, spent 4, closureReserve 13. Obligation `Debt59` has debtor `Payer`,
creditor `Lender`, denomination and settlementAsset `Cash`, principal 50,
accrued 9, outstanding 59, `AccrualFirst`, conversion
`{mantissa: "1", scale: "0", rounding: "none"}`, and status `Outstanding`.
Its identifier is an opaque identifier, including in the PrincipalFirst case.

Action 0 is Transfer `BranchTransfer`, Payer to Lender, asset Cash, amount 17.
Action 1 is Repay `BranchAllocation`, transferId `BranchTransfer`, obligationId
`Debt59`, payer Payer, nominalAmount 17. In the third-party cases, Sponsor replaces
Payer as balance owner, allowance owner, Transfer sender, and Repay payer only;
the obligation's debtor stays Payer.

For each of the first four successes, identity conversion requires settlement
`17 * 1 / 1 = 17`, leaving zero unallocated transfer funding. AccrualFirst gives
`dA = min(17, 9) = 9`, `dP = 17 - 9 = 8`, principal `50 - 8 = 42`, accrued
`9 - 9 = 0`, and outstanding `59 - 17 = 42 = 42 + 0`. The gross allowance debit
is 17: remaining `61 - 17 = 44`, spent `7 + 17 = 24`, with total 68 preserved.
Two actions consume two ordinary work units: remaining `9 - 2 = 7`, spent
`4 + 2 = 6`, and closureReserve remains 13. The total `7 + 6 + 13 = 26`
equals input `9 + 4 + 13 = 26`.

Every successful post preserves the obligation's id, debtor, creditor,
denomination, settlementAsset, allocation rule and complete conversion record.
It remains `Outstanding`, since the residual is positive. The single allowance
stays in place with the exact sender and Cash asset. The complete used-ID lists
become `["BranchTransfer"]` and `["BranchAllocation"]`, respectively. No
obligation or unrelated balance is removed. No additional records are inserted
except an absent receiver balance.

The complete ordered effects for each success are:

1. The exact input Transfer record, including its id, sender, recipient, asset
   and full gross amount.
2. A Repayment record with allocationId `BranchAllocation`, transferId
   `BranchTransfer`, obligationId `Debt59`, that case's payer, creditor `Lender`,
   denomination `Cash`, settlementAsset `Cash`, and the case's exact nominal,
   settlement, principalDischarged, accruedDischarged and remainingOutstanding
   values below.

Each success has top-level status `Prepared`, schemaVersion
`moriarty-funded-repayment/0`, complete `post`, and those two effects only.
The JSON contains all these fields explicitly; nothing is an expected-output
patch or a partial comparison.

## Five successful branches

Balance lists below are in exact array order; every listed row has asset Cash.

| Case | Complete balance arithmetic and order | Financial branch and complete Repayment quantities |
| --- | --- | --- |
| `missing-receiver-append` | Input `[Payer 73, Custodian 19]`; output `[Payer 56, Custodian 19, Lender 17]`: `73 - 17 = 56`, `0 + 17 = 17`. Total `73 + 19 = 56 + 19 + 17 = 92`. | Existing unrelated row 1 is preserved and Lender appends at index 2. Payer is Payer. Nominal 17, settlement 17, principalDischarged 8, accruedDischarged 9, remainingOutstanding 42. Post P42/A0; allowance Payer/Cash 44/24; work 7/6/reserve13. |
| `swapped-balance-rows` | Input `[Lender 11, Payer 73]`; output `[Lender 28, Payer 56]`: `11 + 17 = 28`, `73 - 17 = 56`. Total `11 + 73 = 28 + 56 = 84`. | Receiver is index 0, sender index 1. No append or sorting occurs. Payer is Payer. Nominal 17, settlement 17, principalDischarged 8, accruedDischarged 9, remainingOutstanding 42. Post P42/A0; allowance Payer/Cash 44/24; work 7/6/reserve13. |
| `third-party-payer` | Input `[Sponsor 73, Lender 11]`; output `[Sponsor 56, Lender 28]`: `73 - 17 = 56`, `11 + 17 = 28`. Total 84 is preserved. | Sponsor differs from debtor Payer, and has its own exact allowance. Transfer sender and Repay payer both equal Sponsor. Nominal 17, settlement 17, principalDischarged 8, accruedDischarged 9, remainingOutstanding 42. Post P42/A0; allowance Sponsor/Cash 44/24; work 7/6/reserve13. |
| `third-party-row1-missing-receiver` | Input `[Custodian 19, Sponsor 73]`; output `[Custodian 19, Sponsor 56, Lender 17]`: `73 - 17 = 56`, `0 + 17 = 17`. Total `19 + 73 = 19 + 56 + 17 = 92`. | Combines sender index 1, missing receiver, unchanged unrelated index 0 and a payer distinct from debtor. Payer is Sponsor. Nominal 17, settlement 17, principalDischarged 8, accruedDischarged 9, remainingOutstanding 42. Post P42/A0; allowance Sponsor/Cash 44/24; work 7/6/reserve13. |
| `principal-first-crossing` | Input `[Payer 73, Lender 11]`; output `[Payer 65, Lender 19]`: `73 - 8 = 65`, `11 + 8 = 19`. Total `73 + 11 = 65 + 19 = 84`. | Input obligation P5/A10/O15 uses PrincipalFirst; Transfer and nominal amounts are 8. `dP = min(8,5) = 5`, `dA = 8 - 5 = 3`; P `5 - 5 = 0`, A `10 - 3 = 7`, O `15 - 8 = 7`. Payer is Payer. Nominal 8, settlement `8 * 1 / 1 = 8`, principalDischarged 5, accruedDischarged 3, remainingOutstanding 7. Allowance remaining `61 - 8 = 53`, spent `7 + 8 = 15`, total 68. Work 7/6/reserve13. |

All five also have exactly the preserved metadata, full effects, used-ID lists,
status and schemaVersion described above.

## Eleven rejection branches

Each expected result consists of exactly `status: "Rejected"`, `code`, and
`actionIndex`; it has no post-state or effects. Transfer-stage changes remain
tentative when a later Repay fails.

| Case | Input distinction and earlier-guard justification | Expected code / actionIndex |
| --- | --- | --- |
| `reject-missing-sender` | Rename sender balance row to Bystander, retaining Lender as the other row. Input keys remain unique, accounting invariants and work pass, amount 17 is positive, parties differ and IDs are fresh. No `(Payer, Cash)` sender balance exists; the exact Payer allowance still exists. | `MISSING_BALANCE` / `0` |
| `reject-allowance-owner` | Allowance owner is Sponsor while Transfer sender and funded balance remain Payer. Unique valid state and work pass; sender 73 covers 17. The allowance's asset and amount are otherwise valid, but the required `(Payer, Cash)` allowance does not exist. | `MISSING_ALLOWANCE` / `0` |
| `reject-self-transfer` | Transfer recipient becomes Payer, equal to its sender. Both initial rows remain distinct, work passes and Transfer amount is positive. Reject at the Transfer self-payment guard before any later obligation-recipient mismatch. | `SELF_TRANSFER` / `0` |
| `reject-duplicate-balances` | Change Lender's balance-row party to Payer, yielding two `(Payer, Cash)` keys. All records are individually well-formed; financial state uniqueness fails before actions. Work remaining 9 is sufficient. | `DUPLICATE` / `null` |
| `reject-obligation-id` | Repay obligationId becomes fresh valid identifier OtherDebt. Transfer remains funded and succeeds tentatively; Repay amount is positive but no target obligation exists. | `MISSING_OBLIGATION` / `1` |
| `reject-recipient-mismatch` | Both Transfer recipient and existing receiver-row party become Bystander. The positive Transfer is fully funded: sender `73 - 17 = 56`, Bystander `11 + 17 = 28`, allowance 44/24. Repay references the executed transfer, existing outstanding Debt59 and positive nominal 17 within 59. Its creditor remains Lender, so the transfer recipient mismatches only at Repay. | `TRANSFER_MISMATCH` / `1` |
| `reject-asset-mismatch` | Both balance assets, allowance asset and Transfer asset become Token; obligation settlementAsset and denomination remain Cash. Transfer has an exact funded `(Payer, Token)` balance and allowance and safely credits Lender/Token. Target obligation, positive nominal, outstanding bound and transfer reference pass. Token differs from required settlementAsset Cash at Repay. A source adapter must type the Transfer amount as Token to preserve this financial test. | `TRANSFER_MISMATCH` / `1` |
| `reject-zero-repay` | Repay nominalAmount alone becomes 0. Input admits canonical zero and Transfer 17 executes tentatively with sufficient balance, allowance and receiver capacity. Repay's positivity guard fails. | `ZERO_AMOUNT` / `1` |
| `reject-settled-obligation` | Set principal, accrued and outstanding to 0 and status to Settled. This is valid input state: `0 + 0 = 0` and status matches zero outstanding. Transfer is funded and Repay nominal 17 is positive; the existing obligation is not Outstanding. This guard precedes attempting discharge against its zero residual. | `NOT_OUTSTANDING` / `1` |
| `reject-duplicate-before-work` | Combine the duplicate `(Payer, Cash)` rows with work remaining 0. Work spent 4 and reserve 13 still satisfy the UInt128 input invariant (`0 + 4 + 13 = 17`). State uniqueness is checked before the two-action work requirement, so exhausted work does not mask the duplicate. | `DUPLICATE` / `null` |
| `reject-overflow-before-zero-repay` | Receiver balance becomes UInt128 maximum `340282366920938463463374607431768211455`; Repay nominal becomes 0. The maximum is a valid individual amount. State uniqueness, obligation/work/allowance invariants, positive Transfer, distinct parties, fresh ID, sender funding and exact allowance all pass. Receiver credit would be `340282366920938463463374607431768211472`, exceeding the maximum by 17. Transfer action 0 fails before action 1 can reject zero repayment. | `OVERFLOW` / `0` |

## Freeze and verification scope

Frozen SHA-256 values:

| Artifact | SHA-256 |
| --- | --- |
| New `fixtures/branches.json` | `2a9596d52e934533629354a7ecdbd60282f192efcacd3ef9ec7a95ff6d0845dd` |
| Unchanged existing `fixtures/cases.json` | `d46679961af39a10adec03cb86b4481a39fc9393d15ceb48b37b10b8ce28f832` |
| Normative `spec/successor/repayment-kernel.md` | `a918af934307ecca9ee722fb4bd3d33d886ae36d1d2ffa5dce47b098dc7f26e8` |
| Normative `spec/successor/funded-source.md` | `e6c70a8ddb980fd5d86f161597cfa40e2d9960a52c8891c8b1e44ed973b8cc20` |

Author verification checks JSON parsing, exactly 16 unique case IDs, five complete
Prepared records, eleven closed rejection records, the admitted collection shape,
and unchanged existing fixture bytes against Git HEAD. Financial expectations
were authored as decimal constants with the arithmetic above, never harvested
from implementation results. Subsequent implementation comparisons and independent
result audits belong in separate evidence; this freeze does not claim they passed.
