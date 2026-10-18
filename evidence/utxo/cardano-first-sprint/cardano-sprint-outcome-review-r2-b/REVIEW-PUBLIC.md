## Verdict: APPROVE_EXPECTED_OUTCOMES

Top finding: Revision 2 makes every r1 pass/fail criterion objectively testable without approving source417, native execution, or Preview spending. This approval covers expected outcomes/design/resource predicates only; source/native approvals remain false.

### R1 closure check

| R1 finding | R2 predicate | Required evidence |
|---|---|---|
| A1/B1 audit circularity | Deterministic `DERIVED_AUDITED_EMITTER_ARTIFACT`; audits cover frozen candidate, emitter, schemas, adapters, drivers; runtime recomputation/byte comparison | `auditBinding` with candidate/source/dependency hashes and two distinct FULL compiler+ledger provider records; O01–O04, O07–O08 refuse mismatches before native action |
| A2/B6 freshness | Stable bundle drift `0..5` slots, max 4 attempts; pre-sign/pre-submit ancestor, protocol/era/genesis, input/script equality; drift, spend, or rollback invalidates receipt/signature | O09–O13; postcompile refresh preserves admitted clock/artifacts but requires final-body remeasurement |
| A3 cash/head inputs | Enterprise seed-owner recipient and witness, exactly 100 Cash, measured minimum, separate reconciled change, distinct seeds/fee/collateral inputs except defined settled-output carries | O19–O27; different same-value holding outref refuses |
| A4 receipt binding | Final CBOR/txid, interval slots and POSIX bounds, script-data hash, collateral/total/return, outputs/change, minima, units, fee, witnesses, audits, genesis/instance | O14–O17 mutation controls; receipt equality alone insufficient without authentic issuer and freshness |
| A5/B6 amount/windows | Explicit UInt128/Word64, obligation, seed, and complete-window boundary predicates | O28–O42, including `partition−1`/`partition` and `refundInclusive`/`refundInclusive+1`; retained 68 quantity controls |
| A6/B3 confirmation | I1 = exact inclusion plus at least 1 subsequent block; F18 = same ancestry/full effects plus at least 18; rollback resets both | O43–O45; checked block/ancestry observer required, otherwise `BLOCKED_BY_CHAIN_OBSERVER` |
| A7/B5 race loser | Original-pair, submit-time or original-outcome `BadInputsUTxO` must name the exact shared head, exclude fee/collateral causes, and pair with one F18 full-effect winner; exactly 2 submits | O46–O50; silent loser remains `INCOMPLETE_LOSER_RECEIPT`, no third submission |
| A8/B4 Preview | Prospective ceilings `5,000,000`/tx, `100,000,000` aggregate fees, `2,000,000,000` debit, `200,000,000` collateral; allocations total `405,000,000` from requested `450,000,000`, 16-submission maximum | Executable authority/endpoint/faucet/allocations remain null and status remains `BLOCKED_BY_PREVIEW_DEPENDENCIES`; independent Preview parity/recompile and P01–P02 required |
| B exit gaps | Offline O01–O50, local L01–L03, Preview P01–P02 are separately mandatory; exactly four measurement passes, no fifth | L03 requires actual local phase-2 rejection evidence; evaluator-only rejection leaves it open |
| Timing defect | `preparationSlots=900`, then margin 30 and two 60-slot windows: refund ends 1050 slots after capture within the 1800-second horizon | Explicit scheduling amendment requiring outcome/full-audit approval; exhaustion gives `TIMING_BUDGET_EXHAUSTED`, never silent extension |

### Preserved dissent and limits

Historical FULL audits 391/392 remain rejections, not approvals. The 900-slot schedule is not retroactively part of r1, F18 is an operational threshold rather than irreversible-finality proof, byte-perfect UNIT evidence can never issue production eligibility, and the proposed Preview numbers are not a user spending grant. Source383 remains untouched/unapproved, both later FULL exact-current compiler+ledger audits are still required, and all native/resource operations remain unexecuted.
