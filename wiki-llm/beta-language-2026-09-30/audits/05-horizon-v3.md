# Horizon v3 audit 5 — exact-byte correspondence review

**Scoped verdict:** concur with the seven repairs and the displayed design choices below. **Do not claim full first-failure correspondence concurrence until new H1 is disposed.** One new medium finding M1 concerns the admitted Deposit domain. The published worked values agree with independently recomputed integer arithmetic. This is a proposed-interface review, not grammar, code, certificate, authentication, native proof or ledger approval.

## Identity and evidence scope

Requested reviewer: GPT-6.1 Sol, medium effort, seat5. No independently returned provider/model identity or effective-effort receipt is available inside this child. The requested routing is recorded honestly without substituting an attested identity.

Exact candidates inspected:

- `PROGRAMMER-MOCKUP.md`: `16d419cb4e8f26571d9aab79406e5e8864e7db623028b98929274c727e5d601e`.
- `FULL-LANGUAGE-HORIZON.md`: `f5c947822879f55d25a06efc48eec77eb82149c9cadb262b31e6274f683db488`.

Both matched the assignment. AGENTS.md, the checked-in develop skill and guarded status were refreshed in `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. The entire candidate interface, supplement, coverage/status matrices and §16 were inspected; source segments truncated by tool output were reread. No peer audit or standalone repair report was opened. Candidate links to previous audits were not followed. The original programmer mockup requirements remain the scope; the audit does not expand implementation authority. Guarded status still reports operational evidence gaps and no pending transactions.

All findings below are document observations, inferences or recommendations; numerical results are integer/rational arithmetic observations. Only this new audit file is written. No code or source document was edited, and no commit, campaign or network transaction was performed.

## Recheck of all previous findings

| Previous finding | Current disposition and substantive vote |
| --- | --- |
| H1, claim-first versus proportional slash | **Resolved at design level; concur.** §12 now signs SameTierProportional over Free/Pending/MatureUncommitted holders, excludes Paid, and separates external encumbrance priority from holder loss allocation. The tier book provides exact claims/duties/cursor and reserve dust. Protected senior backing is explicitly outside active B. |
| H2, lifecycle/branch net ambiguity | **Resolved at design level; concur.** Bounds now has only gross/fees/work/stages. Signed ReceiptRequirement carries gate, recipient, asset, scope and exhaustive deductions. Originate/Mint are milestones; Redeemed/EmergencySettled and the three composed outcomes have independent terminal gates. Whole-life profit is expressly a different Delta predicate. |
| H3, composed continuation heads | **Resolved at design level; concur.** Promise explicitly uses Initial for Escrow/Claim and PreviousAcceptedHead for Swap/OriginRefund/DestinationReturn. Exact agreement/Pair/StageId/accepted edge bind continuation. Unrelated progress requires fresh signed preserving authorization. A rejected Swap or service observation supplies no economic head edge. |
| M1, grant passed as KernelScope | **Resolved as interface shape; concur.** scopeFor receives stage, parent, grant, Economic/Query identity, durable attempt and authorization evidence; receipts retain identity, parent digest, grant and attempt. Query observes only and cannot acquire economic custody rights. New H1 concerns validation order, not this corrected shape. |
| M2, cosignatures and expiry | **Resolved at design level; concur.** Parent/grant/resource/native signatures are conjunctive and noninterchangeable; Owner/Reserve keys are domain-bound. Expiry/revocation leaves duties pending. Renewal authenticates original-time history and supplies new current authority while preserving programs, receipts and spent counters. It does not make old expired grants currently valid. |
| M3, cash versus custody debit | **Resolved at design level; concur.** DebitCustody names LoanPledge cell/object/capability; free_gold is read and framed unchanged. The collateral spend bound is charged once and Other's lock survives. |
| M4, pure function and checklist | **Resolved at design level; concur.** PureIntent shows actual reusable bounded functions and exhaustive ordinary ADT matching; guarded unsigned subtraction has no side effect. Checked boxes expressly mean proposed source coverage, not implementation or reviewer financial consistency approval. |

These dispositions approve the repaired source-level decisions, not their unperformed compiler/verifier tests or formal proofs.

## Independent §16 worked-matrix check

I used separately written Python integer arithmetic, without importing any language/profile derivation code. In cents, `owed_i = floor((B-L)*s_i/sum(weights))`; dust is the remainder after summing entitlements. No horizon execution occurred.

| Matrix row/group | Independent result / assessment |
| --- | --- |
| One pending100 / free1000, B1210/S1100/L121 | Post B1089, owed99/990, dust0; claim loss11 and free loss110 sum121. The old pending0 certificate fails the selected proportional relation. |
| Pending100/rank10, pending200/rank20, free800 | Post99/198/792; losses11/22/88 sum121. Rank changes do not change these same-tier weights. |
| Second claim matures without commitment | Same99/198/792. Maturity is withdrawal eligibility, not removal from exposure. |
| Add Paid55 with active shares0 | Same active allocation; paid55 is excluded and unchanged. This is an intended state/framing predicate, not executable proof. |
| B101/S3/L1, weights1/2 | Post B100, owed33.33/66.66, dust0.01. Sum100 exactly. |
| Senior100 plus active B1210 | Protected100 unchanged; active1089 with99/990. Total backing1189 after loss121 from1310. |
| Total1210 contains senior100, active B1110/S1100/L121 | Active B989; owed89.90/899.09, dust0.01. Senior100 remains, total1089. Correctly requires a different signed profile. |
| Consumed slash or ineligible lower-priority lock | No second loss/cursor advance is authorized. History/Effect rejection is specified-only; simultaneous-failure precedence remains subject to H1. |
| Full YieldLife | Deposit100 mints100 into1000/1000; reward110 gives1210/1100; quote110; slash leaves1089 and claim99; withdraw pays99/burns100, leaving990/1000. Paid entry active0/owed0/paid99 and cursor8 are consistent. No-reward loss110 yields claim90. |
| Loan milestone / subsequent repayment | Fresh origination500 passes its milestone; cash delta470 is allowed after repayment30; rolled debt505→475→75 after sale400. Cumulative Alice debit30 remains spent. |
| Coin mint/burn/redeem | Mint milestone100 passes; terminal MUSD0 is allowed; fresh GOLD0.190 passes Redeemed, whole-life GOLD delta−0.010 is not a profit promise. Gross GOLD0.200 and MUSD100 remain. |
| Shutdown/emergency path | EmergencySettled alone selects the GOLD0.190 terminal requirement; historic Mint milestone still required. Inapplicable Redeemed does not waive an applicable floor. |
| Delivered | Source G100.20/F0.20; WUSD G99.70+0.30=100/F0.30; fresh GOLD_F0.900 passes and0.899 fails regardless of old wallet balance. |
| OriginRefund | Fresh USD100 passes; G100.20/F0.20 remain, cash delta−0.20. No GOLD_F requirement is selected. |
| DestinationReturn | New claim-bound100 WUSD minus zero accepted spend leaves100; source backing100 and fee0.20 remain. Unknown late debit prevents a custody terminal proof. |
| Old wallet1000, new480, floor500; charge probes | Old balance contributes0;480 fails. Fresh520 minus20=500 passes isolated receipt measurement;500 minus20=480 fails. Actual LoanPromise fee cap0 separately rejects such20 fee. |
| New claim90 plus old wallet1000 | Reserved claim90 fails100; old1000 cannot fill the deficit. |
| Escrow/Claim/Swap and refund heads | The signed selectors resolve p1/f1, not old p0/f0; forged unrelated predecessor/equal text is insufficient. Unknown-finalized original Claim is linked once; unrelated fx requires new authority. These are specified predicates, not reproduced ledger behavior. |
| Attempt/query mutations | Same economic StageId/Pair cannot reset acceptance; query/selector/parent/grant/domain/attempt mismatch cannot exchange receipts. Typed checks remain unexecuted. |
| Missing Reserve signature, wrong key/domain/epoch, expiry/revocation | The conjunction requires every applicable signature/binding. Late301/401 cannot use old300/400 grants; preserving renewal is new authority. Which first failure is reported under simultaneous errors needs H1. |
| Pledged liquidation | Free GOLD5 unchanged; custody2−1=1; Other lock0.5 remains; buyer GOLD1; creditor receipt400; residual debt/duty475−400=75. One custody transfer. Replacing with cash Debit or dropping custody write is a specified mismatch. |
| PureIntent.missing |900−899=1 GOLD atom, or0.001 GOLD. It is diagnostic data, not financial floor mutation or acceptance. |

This covers every slash, receipt and continuation/scope/authority/custody row in §16. Arithmetic agrees; qualitative rejection/qualification claims were checked against the written predicates and remain specified-only.

## New findings

### H1 — Early scope authorization still needs the shared first-failure schedule

**Document observation:** §2 requires Stage→Intent→Effect→Authority→History→Failure. AuthorizationConjunction includes `GrantExpiryCheck`, revocation, signatures and resolved heads. §4's OneDomain expansion calls `scopeFor(...E.authorization)`, checks its Result and exits before entering §3 OneDomain. Only inside OneDomain does Core.prepare run the full financial relation. §16 expects an expired grant to reject authority.

**Inference:** The currently stated ordering permits an authenticated expired grant to cause Authority rejection before a simultaneously invalid signed cap or effect-range failure reaches Core.prepare. The same problem affects recovery's early scopeFor.requireOk. Correct typed scope shape alone does not preserve first failure. Calling all failed authorization evidence a formation/Stage error would also contradict the advertised Authority result for expiry.

**Repair:** State an explicit scheduled-validation contract shared by scopeFor and Core.prepare. Scope construction may check structural/domain/identity evidence needed for Stage formation, but must defer Authority/History predicates until preceding Intent/Effect checks have determined their observations. Alternatively Core.prepare can produce the canonical first failure before any scope-dependent side effect; scopeFor must not override it with a later judgment. No check may be skipped for successful dispatch. Different client entry points must use this same schedule.

**Exact discriminator:** Well-formed authenticated state and grant, expiry round300, now301 inside an otherwise valid renewed/extended parent window, plus a source-fixed amount exceeding gross cap. Expect Intent cap failure before Authority expiry. Add recipient overflow and valid cap: Effect range before Authority. Add invalid Stage binding as well: Stage first. Repeat through OneDomain, recover and direct local preparation; all rejections publish null new financial state/effects. These tests have not run because the horizon implementation is absent.

This is a validation-order specification gap, not evidence that an implemented backend accepted an invalid transaction. It blocks a full correspondence vote while preserving concurrence on the repaired architecture.

### M1 — Deposit's tier-book recomputation must update active claims/duties or reject that domain

**Document observation:** §12 Deposit recomputes the certified RiskTierBook and writes owner/shares/vault/book. Its footprint has no active withdrawal claim/duty cells. Reward explicitly rejects active withdrawals to avoid partial book edits; Deposit has no corresponding restriction. Exact book entries are intended to remain equal to claim/duty owed.

**Arithmetic discriminator:** Existing active backing1001 USD, supply100, pending10 shares yields owed100.10 USD. Deposit100 mints floor(100×100/1001)=9 shares. Recomputed backing1101/supply109 gives pending floor(1101×10/109)=101.00 USD. The pending entitlement changes by0.90 while its claim/duty cell is outside Deposit writes. This is a possible typed pre-state, not the displayed no-active-withdrawal YieldLife witness.

**Repair:** For this restricted first profile, require no active withdrawal claims on Deposit as on Reward, with a named rejection and same-head evidence. Or include every affected claim/duty in the complete bounded footprint and update them atomically. A generic acceptance invariant could reject the inconsistent input, but the admitted domain should be explicit rather than an accidental final mismatch.

**Test:** Deposit with no active claims; pending and mature-uncommitted claims; paid-only entries; rounding that changes an existing claim; excessive active claims beyond footprint bound. No accepted transition may update the book while silently retaining stale liability cells. This remains a financial-profile implementation obligation, not a demand for new beta execution.

## Full-interface scope and remaining obligations

All eight family lifecycles and BridgeThenSwap remain visibly covered by one proposed horizon. Typed nominal values/resources/prices/clocks, exact bounds, certified arithmetic, finite holes, source-fixed authority, same-head footprints, ordered effects, residual liabilities, Unknown reservation and distinct recovery branches are present. Base-per-Quote direction is preserved. Governance authority remains financial, not a compiler admission prerequisite. There is no generic host call or claimed solver authentication.

The supplement's transfer/repayment trace and Source/6 mapping preserve the existing local S0 qualification limits. Only that existing parser/preparer is local evidence. Proposed beta registry recognition is specified; horizon structs/stages/functions/matches are unsupported beta grammar. All advanced eight-family finance is specified/open. No readable source, matching arithmetic, successful review, hash or receipt promotes these statuses.

Before a formal or executable correspondence claim, require the actual grammar/type checker, total bounded elaboration and origin preservation, independent financial oracles, complete footprints/framing, numeric/certificate soundness, a common first-failure projection, canonical distinct signature/renewal statements, authenticated observations/snapshots/head bindings, no duplicate effect across attempts, and atomic ledger consumption of effects/budgets/history/duties. W-D0–W-D4, M4-C1–C5 and native/U financial gates remain open.

**Recommendation:** Accept the seven repaired decisions as design dispositions, preserve the verified literal arithmetic, explicitly resolve H1 and restrict or complete Deposit's domain under M1. Re-review changed bytes as needed. My concurrence is scoped to the stated design repairs and coverage; it does not certify complete correspondence or authorize financial execution.
