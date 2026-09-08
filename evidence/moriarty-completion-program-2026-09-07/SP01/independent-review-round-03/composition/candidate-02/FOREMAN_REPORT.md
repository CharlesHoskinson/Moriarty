# FOREMAN_REPORT

Worker: Grok 4.6 high author
Worktree: `/home/charl/Moriarty/.worktrees/sp01-map-composition-grok`
Task: SP01 composition specified-only correction against GPT-6 findings C01 through C11
Status: specified-only. Independent GPT-6 audit of this correction is required.

## Owned files

- `experiments/moriarty-language/spec/successor/financial-fragments/composition.json`
- `FOREMAN_REPORT.md`
- `FOREMAN_REPORT.json`

Candidate-01 freeze is retained and was not edited.

## Commands actually run

```
python3 /tmp/correct_sp01_composition.py
python3 independent ID, cash, type, pin, theorem, mutation, and footprint checks
sha256sum composition.json candidate-01-freeze.json gpt6-composition-review.json
```

No subagent, network, build, proof, wallet, transaction, or commit.

## Hashes

- gpt6-composition-review.json: `0c6638ba68a68a52b6d8006cc289c4b00abf76f743796a0677dda4cf0bf607c5`
- candidate-01-freeze.json wrapper: `bea5019a933a9381e2af08f1e8673815e8d76e9ecaa8471a2c2813c6e7754cfd`
- freeze candidateSha256 field: `afd9d650c1c20f7891be369784b79e22124ff9a5cef41169984022fef1db8fb9`
- freeze inner composition.json: `5493e0c1a5216a73aa6fb5edba4b54525a857e648e89a842365d2793afeb27c6`
- corrected composition.json: `3ed9403616dc6cb6eb843ef94812481f168c525e2de375c80b5eac253827db4b`

## Coverage retained

Six traces, ten mutations, and eight proposed theorems keep their IDs. Source sha256 pins are unchanged. NAM19 pin remains `bfc39c7a344b1243ce15accb9800837b3d87a050ffa435395595e4bf559e4a9b`. Cash account amounts and AMM outputs were not rewritten. axioms and mechanizedEvidence stay empty. Theorem status stays proposed.

## Finding dispositions

### C01

Disposition: corrected.

Step history now checks consumed subseteq immediate prefix historyIds. Aggregate externalConsumed equals allConsumed minus allProduced. Aggregate externalProduced equals allProduced minus allConsumed. orderedTranscript is retained. expected.history, consumedHistories, producedHistories, and postState.historyIds match.

Independent check:

- NAM19 aggregate consumed [] produced [H_nam19_rr2]
- sequential consumed [] produced [H_seq_B]
- parallel consumed [] produced [H_par_join]
- shared consumed [H_pool_pre] produced [H_shared_AB]
- atomic consumed [H_lender_pre, H_poolq_pre] produced [H_atomic]
- async consumed [H_alice_pre] produced [H_async_refund]

Every step passed consumedSubseteqPrefix and producedDisjointPrefix. Every post.historyIds equaled (pre minus externalConsumed) union externalProduced.

### C02

Disposition: corrected.

jsonDecoder defines CompleteState, FinMap array-or-object decoding, and stripping of accounts `_scale`. Runtime Nat is separate from source-decimal and source-rational. USD remainingGap is no bounded micro-unit Nat encoding. NAM19 principal and duty.amount are marked notNat. NAM19 remaining duty `nam19-debt` amount `5236.461356333502` is in canonical postState.duties. postState also has observations and contract. boundParameters give type, unit, scale, and numeric constraints for each bound including M_a.

Independent check: postState.duties[0].id is nam19-debt and residual is true. USD quantityDomain is source-decimal. Independent IED-RR accrual is Fraction 7200/73.

### C03

Disposition: corrected.

Alice atomic USDC gross is 50 swap plus 50 repay plus 1 fee equals 101. Original toy cap is 101. cumulativeGross is 101. remaining is 0. Prefund/net cost stays 51. Incoming borrow 50 does not cancel gross debit. Joint Transfer USDC bound is 101. Transfer WETH bound 47 is present. Shared and atomic postState keep Alice, Bob or Lender, pool, and Treasury authority counters. This is a recorded toy cap change. It is not a source conformance claim.

Independent check: sum of Alice USDC outgoing transfers is 101. residualAuthority.Alice.original is 101.

### C04

Disposition: corrected.

effectSystem names primitive constructors and fields. elaborate(concretePlan, pre) is defined. apply(pre, effects)=post is a specified finite relation. Every expected.effects item has ctor. concretePlan.effects is populated on all six traces. Hard constraints are `{id,text,predicate}` records. Accepted is the conjunction of the four mandatory claims plus GenesisAdmin. Genesis unfunded debt and admin loss-label erasure bind into TransitionValidity. AccrueNominal, Capitalize, AssignDuty, SettleTransfer, ReceiveMessage, and history/work/authority effects are authorized where used. Status stays specified-only. This is not an implemented runtime.

Independent check: zero effects lack ctor. Plan effect list lengths are 14, 7, 10, 16, 14, 22. ReduceDebt uses amount 50 with allocationId A_repay_D_flash and payee Lender.

### C05

Disposition: corrected.

Canonical paths and resourceAliasMap cover every name used here. H_nam19_prefix is removed. Footprints list history, work, authority, fee, duty, message, coordinator, and observedTick cells. Frame equation is Frame(pre, writes)=Frame(post, writes). Reads do not authorize mutation. unrelatedRecord UnrelatedProbe is populated in every pre and post and is outside writes.

Independent check: H_nam19_prefix is absent from NAM19 writes. history.H_par_join and coordinator.work are in parallel writes. authority.joint is in atomic writes. observedTick is in async writes.

### C06

Disposition: corrected.

A contract is `{Asm,Guar}` on finite traces. sat is implication of Guar from Asm. Composed assumptions are AsmA AND AsmB AND OpAsm unless an explicit entailment discharges one. Intersection that drops a unilateral assumption is forbidden. Compat_sequential requires GA implies AsmB, residual authority, interference stability, and joint financial closure. Parallel, shared, atomic, and async Compat are stated. G_op is defined per operator. Unique receive/refund is in async Compat.

Independent check: theorem predicate contains AsmA AND AsmB. It does not use AsmA intersect AsmB as the composed assumption.

### C07

Disposition: corrected.

CompleteObs includes accounts, transfers, debts, duties, authority counters, work spent/ordinary/closure/lifetime, fees, shares, positions, claims, messages, history, status, observations, observedTick, and provenance. Alpha-renaming is a bijection on the finite bound-ID set actually used. Associativity requires both groupings admitted, equal Alloc, equal Cost, equal Serialization, equal RejectObs, and equal PrefixObs. Those maps are defined. Unconditional associativity is not claimed.

Independent check: observation-equivalence formula includes remainingOrdinary and remainingClosure.

### C08

Disposition: corrected.

ReduceDebt and SettleDuty bind debt or duty ID, entitled payee, funding transfer, and allocationId. Alloc is injective on allocationId. Sum of allocations on one transfer is at most that transfer amount. Partial residuals keep residual true while amount is positive. WriteOff and Novation are typed constructors. Closure and exhaustion do not delete residual duties. NAM19 duty remains in postState.

Independent check: injective-allocation definition is present. NAM19 postState duties is not empty.

### C09

Disposition: corrected.

P0 and D0 syntax are pinned as atomicCoreRetention. T is total on nonempty AdmittedSyntax_D0. T is not implemented. Retention requires every old Prepared execution to have a successor Prepared execution with equal D0Obs_complete. A successor that rejects every admitted old program fails the predicate. Converse/reject scope is stated. Vacuity is not admitted.

Independent check: conservative-extension predicate contains the reject-all failure clause. axioms remain [].

### C10

Disposition: corrected.

validationOrder is ContractInvariant, IntentRefinement, TransitionValidity, HistoryCompliance. Each mutation has a complete prefix/unchanged CompleteState including accounts, fees, work, authority, histories, and duties. earlierStagesDerived is computed. firstFailingStage is IntentRefinement for all ten mutations. Later gates are in alsoFails. Prefix fees, work, authority, and duties are kept. Late receive keeps Treasury 1, Bob 0, sendFee 1, spent 20.

Independent check: firstFailingStage list is IntentRefinement ten times. Late-receive unchanged Bob USDC is 0 and Treasury USDC is 1.

### C11

Disposition: corrected.

Finite message machine is Created then Pending, Refundable, Received, Refunded, Finalized. Send records Created then Pending in one step. Pending is the live lock. Legal ticks and boundary priority are stated. Receive is escrow-backed from Escrow to Bob with nonce N3 consumed once. Alternate receive and refund successors share the post-send Pending base. samePreReceiveVsRefund is the race control. Destination finality stays an assumption, not a proof.

Independent check: step 0 message status is Pending. receive successor credits Bob USDC 25. race contains samePreReceiveVsRefund.

## Independent cash and work checks

These account totals match the prior oracle arithmetic. They were not rewritten.

| surface | check | result |
| --- | --- | --- |
| NAM19 USD | 5000+5000 | 10000 |
| NAM19 IED-RR accrual | 5000*(8/100)*(90/365) | 7200/73 |
| sequential USDC | 70+30+0 | 100 |
| parallel WETH/USDC | 13+7 and 29+11 | 20 and 40 |
| shared AB floor | 100*20//120 | 16 |
| shared mint | min(12*100//120, 8*100//84) | 9 |
| shared totals | USDC/WETH/LP | 170/120/109 |
| atomic floor | 1000*50//1050 | 47 |
| atomic totals | USDC/WETH | 1151/1000 |
| Alice gross USDC | 50+50+1 | 101 |
| async USDC | 39+0+1 | 40 |
| work all traces | spent+ordinary+closure | 1024 |

## Correspondence gap

Authority-safety assumes an SP01.3 signing schema. This fragment does not bind envelope bytes, signature algorithms, or key identifiers. Correspondence with the separately reviewed SP01.3 signing interface is not established.

## Limitations

This is specified-only design. It is not implementation, not a successor freeze, not a proof, and not protocol conformance.

Predicates are not executed by a language runtime.

NAM19 remaining 26 events stay pinned. IEEE source-decimal strings stay source-decimal. They are not Nat.

Toy integer AMM, work ticks, named accounts, and the Alice 101 cap are local profile choices.

Destination finality on async is an assumed Midnight-model observation.

Full RP01 and SP10 implementation remain open.

Required source and implementation coverage remains all targets.
