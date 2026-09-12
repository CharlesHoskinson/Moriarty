# Task3 complete source lifecycle: independent expectations

Status: specified-only. This is an independent expected-result design, not an implementation approval or reproduced experiment. Grok4.6 high implements; fresh Astra medium audits the final candidate under the latest task routing. Task2 is still awaiting its final audit. Its canonical contract SHA256 is `74299d10cca2874828c4c847f6817677b033090bb85d38bb898dd462fee98383`, checked against the current file when writing this record.

The next capability is a complete four-invocation source lifecycle with actual predecessor propagation. Use one source/5 agreement with `originate`, `accrue`, `repay`, and `settle`, the existing four protected operation records, and the public four-string evaluate API. No kernel or language expansion is needed for Task3.

## Source and consumer recommendation

Use ordinary state `phase: UInt128` and `paid: UInt128`, seeded at0/0. Every action increments phase from its actual PRE; repayment increments paid by the payment amount. Ordinary POST must progress 1/0, 2/0, 3/30, 4/110. A carried ordinary counter makes the currently missing consumer behavior observable without pretending ordinary state can alter financial debt.

Originate emits Transfer D1 lender-to-borrower100 followed by Originate O1/Loan1 nominal100, cap110, conversion1/none, AccrualFirst, rate1/10 floor, duration60, start1000. Accrue takes A1/index1/time1060 and emits one Accrue. Repay takes nominal30 and fresh P1/R1 and emits Transfer then Repay. Settle takes only fresh P2/R2 IDs: source reads `outstanding<Cash>("Loan1")`, checks its magnitude is positive, computes the transfer Amount from that magnitude, and uses the same Quantity for Repay. Do not supply final80 as a consumer argument or literal settlement amount.

Source ensures must verify actual financial results. Check principal/accrued/outstanding at each stage, relevant borrower/lender balances and allowance remaining/spent, and ordinary phase/paid conservation. Accrue must distinguish financial PRE outstanding100 from POST110. Repay must establish80/0/80, settle0/0/0, lender110 and borrower0. Place the final lender balance comparison last in settlement to support a deliberately late false-condition test. Full result comparisons outside the source additionally cover immutable terms, incurred/cap, status, histories and all unrelated rows that have no dedicated source getter. Avoid phase guards or guards tied to a particular outstanding value that mask duplicate-period/cap test cases; a positive-debt guard is appropriate for computed settlement.

The demo loads source and the initial financial/ordinary seed once. For each successful continuation, snapshot Pre is an owned copy or serialization of **previous.post**, financial input is **previous.financialPost**, and workInitial is **previous.financialPost.work.remaining**. Only action selection and Args change. Obs remains empty. Assert the public arguments match those actual previous results. Keep all input strings and complete outputs, including failures. Advance only after success; a failure cannot become a predecessor. The snapshots file should contain the initial ordinary snapshot and action arguments, not canned intermediate PRE or financial state.

Repository observation: Task2's `examples/financial-lifecycle-payment.mjs` currently passes literal `{due:"100",paid:"0"}` for later ordinary PRE, stops at partial repayment, and prints summaries. Its financial propagation is useful API evidence, but this is not the complete Task3 consumer. The worktree is being repaired separately and this observation does not freeze its bytes.

## Complete state and effect oracle

The companion JSON embeds every financial field and ordered effect for all four stages, derived from the independently specified Task2 oracle and canonical equations. It includes ordinary POST and the funded result envelope. Only source-work values remain explicit symbolic placeholders pending final source. These are oracle templates, not valid wire states until substituted with canonical decimal strings.

| After | Lender/borrower Cash | Lender allowance remaining/spent | Borrower remaining/spent | Principal/accrued/outstanding | Incurred/cap | Cursor/next |
| --- | --- | --- | --- | --- | --- | --- |
| Seed | 100/10 | 100/0 | 110/0 | absent | absent | absent |
| Originate100 | 0/110 | 0/100 | 110/0 | 100/0/100 | 100/110 | 0/1060 |
| Accrue10 | 0/110 | 0/100 | 110/0 | 100/10/110 | 110/110 | 1/1120 |
| Repay30 | 30/80 | 0/100 | 80/30 | 80/0/80 | 110/110 | 1/1120 |
| Settle80 | 110/0 | 0/100 | 0/110 | 0/0/0 | 110/110 | 1/1120 |

Loan1 is Outstanding until the last stage, then retained as Settled. Initial principal100, original funding IDs, roles, conversion and accrual terms survive payment. Transfer history ends `[D1,P1,P2]`, allocations `[R1,R2]`, origins `[O1]`, accruals `[A1]`, preserving insertion order. Other/Token balance7 and allowance3/1 remain exactly unchanged. An additional valid unrelated-obligation seed should verify that implementations do not replace the obligations or history arrays wholesale; its history must satisfy the full state equations.

Effects are ordered Transfer+Origination, Accrual, Transfer+Repayment, Transfer+Repayment. The30 repayment discharges principal20/accrued10; the80 repayment discharges80/0. The full Accrual effect records basis100, interest10, previous/current accrued0/10, outstanding100/110, incurred100/110, eligibility1060 and next1120. Compare complete closed effect objects, not just kind or debt delta. Cash totals remain110, each party's gross allowance remaining+spent remains its initial total, and no accrual creates cash.

## Exact source work acceptance

Let Ei be the dynamic expression reductions of the frozen action prefix and suffix, Ni its emitted kernel actions, Wi=Ei+Ni, and Ci the cumulative Wi. Here Ni is exactly2,1,2,2. With the proposed seed remaining256/spent17/reserve16, stage i must return remaining256-Ci, spent17+Ci, reserve16 and equal top-level workRemaining. Remaining+spent stays273 and including reserve stays289. Kernel-only final249/24 is not the source oracle.

Before claiming completion, freeze the actual `.mori` SHA256 and retain elaborated constructors/spans. Independently count every visited expression and statement, including reads, literals, magnitude/Amount constructors, nested records, Require, Next, Emit and Ensure; count only executed branches under short circuit. Add actual N between prefix and suffix. StaticWorkBound is a cross-check, not a dynamic E+N substitute. Record per-statement counts and literal complete outputs, then compare with execution. If the chosen body costs more than256 overall, select a larger seed within65536 before freezing and update every oracle; never replenish between actions.

Calibration only: existing independent `origination/source-fixture.mjs` has origin36+2+17=55 and accrue10+1+10=21. Those counts do not apply after adding Task3 state, guards or financial assertions. No exact Task3 source total is claimed here because its final source does not yet exist.

For each frozen action, a separate work fixture with W spendable must succeed at remaining0, and W-1 must reject without output. Exercise the boundary after prefix but before enough kernel work, plus exhaustion during the suffix after tentative effects. Count attempted diagnostic work for the late false ensure through the actual false condition, including N. Rejected work is not committed. Closure reserve16 remains unavailable for all these attempts.

## Meaningful failure and consumer controls

The JSON specifies concrete inputs, expected codes and valid retries. Required cases are duplicate A1 after accrual (DUPLICATE), fresh A2 repeating index1 (PERIOD_SEQUENCE), time1059 then retry1060 (PERIOD_NOT_ELIGIBLE then success), next period after repay30 (LIABILITY_CAP_EXCEEDED: incurred would118), borrower payment111 from the accrued predecessor (INSUFFICIENT_BALANCE), missing same-invocation origination funding (TRANSFER_NOT_IN_STEP), and final false lender111 ensure (ENSURES_FAILED). The funding fixture changes action Args, not accepted financial state. Retry with valid amount and the same fresh IDs verifies failure did not consume them.

After settlement, computed settle with fresh P3/R3 must fail its positive-debt guard with GUARD_FAILED. Do not label that kernel NOT_OUTSTANDING: a positive Transfer from borrower0 would fail funding first, while a computed zero payment can fail a zero-amount rule. If kernel settled-payment coverage is separately desired, use a specifically reachable Repay-only control with the preserved priority. The demo must retain its actual public-source failure.

Every rejection has no post, financialPost, effects, descriptors, continuation or workRemaining. Compare the entire accepted predecessor, including ordinary state and spent counters; repeat the same failed input deterministically; then succeed from that unchanged predecessor. Separate malformed-fixture tests must not be passed off as live-state edits. Re-admit every successful financialPost.

Detect consumer bugs directly: compare each captured Pre/financial input/workInitial with the actual preceding output; vary the initial ordinary phase in a separate fixture and require offsets to survive all stages. Deep comparisons must fail under deliberate wrong debt split, missing unrelated row, reset paid/spent, deleted settlement history or altered reserve. Assertions that merely recalculate the observed result are insufficient.

## Evidence and limits

Inputs inspected: Task3 in `docs/superpowers/plans/2026-09-12-language-to-ledger.md`; OpenSpec lifecycle design and bounded-loan-lifecycle spec; canonical `loan-origination-accrual-contract.md`; `design-review/origination-accrual-expectations.json`; root `origination/kernel-probes.mjs`, `source-fixture.mjs`, `source-probes.mjs`; Task2's financial-lifecycle-payment source and consumer. Plugin status was refreshed: no financial transaction evidence, dependent dispatch remains blocked by recorded binding/accounting/resource gaps. Those gates do not block this local expectation design.

Run the documented root check/format/demo commands and lifecycle tests on the final reviewed candidate, plus package tests/typecheck. Retain complete results and fresh exact-candidate audit. This document does not execute source tests, certify Task2, discharge K correspondence, authenticate time/consent, or satisfy Docker/Preview/PCD acceptance.
