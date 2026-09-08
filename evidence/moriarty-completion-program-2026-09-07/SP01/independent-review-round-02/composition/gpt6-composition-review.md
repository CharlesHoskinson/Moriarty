# Independent composition design review

**Verdict: BLOCKED.** Candidate `afd9d650c1c20f7891be369784b79e22124ff9a5cef41169984022fef1db8fb9`.

Reviewed by GPT-6 Astra high. This is a specified-only design review. All six cash trajectories and listed work sums recompute correctly; the complete records and proposed acceptance/theorem predicates remain inconsistent or incomplete. No proof or runtime completion is claimed.

## C01 — Aggregate histories violate the mandatory history predicate

Pointers: `/commonAcceptance/predicates/HistoryCompliance/formula`, `/traces/0/expected/history`, `/traces/1/expected/history`, `/traces/2/expected/history`, `/traces/3/expected/history`, `/traces/5/expected/history`

HistoryCompliance requires consumed subseteq pre.historyIds. NAM19 consumes H_nam19_ied/H_nam19_rr1/H_nam19_ipci from an empty pre.historyIds; sequential consumes H_seq_A from empty; parallel consumes H_par_A/H_par_B from empty; shared consumes H_shared_A absent from [H_pool_pre]; async consumes H_async_created/H_async_refundable absent from [H_alice_pre]. These are legitimate internal step histories, but they are not external inputs of the aggregate transition. Atomic is the only aggregate satisfying the stated formula. Acceptance explicitly binds the aggregate pre, post and expected.history.

Required fix: Define both step and aggregate history semantics. Check each step against its immediately preceding full state, and compute aggregate consumed/produced histories by canceling internal edges. Retain the ordered transcript separately. Reconcile the multiple producedHistories fields and enumerate all consumed/produced history footprint resources.

## C02 — The complete state and quantity domain do not type the supplied records

Pointers: `/theoremLedger/theorems/0/definitions/0`, `/commonAcceptance/predicates/ContractInvariant/formula`, `/traces/0/expected/postState`, `/traces/0/expected/duties`, `/traces/0/preState/authority/lender/transferAllowance/scale`, `/quantitySchema`, `/commonAcceptance/quantitySchema`

StateWellTyped requires observations in every complete state, yet all six postState records omit observations and five preState records omit them. It declares principal, accrued and duty.amount Nat, whereas NAM19 principal and duty are 5236.461356333502 USD. Its account FinMap is incompatible with the in-band _scale scalar absent an explicit decoding rule. Duties/shares/fees are declared FinMaps but mostly encoded as lists; positions/claims/contract/observedTick are omitted from the purported complete record. NAM19 expected.duties contains a residual repayment duty while expected.postState.duties is empty; pre.contract is also silently omitted from post. USD authority quantities say scale 0 while the asset registry says source-decimal.

Required fix: Define a complete bounded sum/product schema and explicit JSON decoding, including decimal/rational source observations separately from runtime Nat units, optional fields, positions, claims, rates and observations. Supply complete states and put the NAM19 duty in canonical postState. Bind units and scales consistently and give M_a and collection bounds actual typed values or explicit parameters.

## C03 — Atomic gross authority omits 50 USDC of actual debit

Pointers: `/traces/4/steps/0/transfers`, `/traces/4/expected/residualAuthority/Alice`, `/traces/4/preState/authority/joint/allowedEffects`, `/traces/3/expected/postState/authority`

Atomic Alice spends 50 USDC to PoolQ, 50 to Lender and 1 to Treasury: cumulative gross is 101, although net loss and required initial prefund are 51. The residual record reports original=51, cumulativeGross=51, remaining=0. Incoming borrowed funds do not cancel gross debit. The joint Transfer USDC bound 51 is not a partition authorizing cumulative Alice spend 101. Shared-state also deletes the Alice and Bob authority records and retains only PoolP.copied=false, so its actual gross debits (Alice USDC 21; Bob USDC 12 and WETH 8; PoolP WETH 16) and residuals are not represented.

Required fix: Keep prefunding/net cost distinct from per-principal per-asset cumulative debit caps. Explicitly authorize and account for Alice USDC gross 101 (or separately partition all legs and show their aggregate). Carry complete authority/counter records through shared and atomic states, with named pool output authority and fee budgets.

## C04 — Mandatory claims and effect authorization remain open or fail on the positive traces

Pointers: `/commonAcceptance/predicates`, `/commonAcceptance/fixtures`, `/theoremLedger/theorems/2/definitions/0`, `/traces/0/expected/effects`, `/traces/3/expected/effects`, `/traces/4/expected/effects`, `/traces/5/expected/effects`

closed=true does not close formulas with undefined apply, conservation witness, concretePlan.effects, effect elaboration, envelope selection and hard-constraint satisfaction. concretePlan.effects is absent in every trace and hard constraints are prose strings without a satisfaction relation. Transfers in expected.effects have no ctor; debt effects have principal rather than amount. NAM19 AccrueNominal and Capitalize are absent from AllowedEffects constructors and its envelope authorizes only Transfer. Shared envelopes authorize Swap/AddLiquidity, but effects contain Transfer and Mint with no elaboration rule or WETH/LP authority. Atomic lacks a WETH Transfer capability. Async only lists transfers as effects despite creating/updating messages, duties and histories. Thus effects subseteq AllowedEffects and apply(pre,effects)=post are neither defined on these records nor sufficient as supplied. Genesis/admin fixture-specific rejection prose is not an integrated complete acceptance predicate.

Required fix: Specify typed constructors, parameters, elaboration from plans to primitive effects, the full effect application relation and exact acceptance conjunction on complete records. Define each hard constraint as a predicate. Include every state-changing effect and authorize it. Bind genesis funding and admin debt/discharge rules directly into these relations. Keep status proposed; no implementation or proof is required at this gate.

## C05 — Footprints omit modified resources and read-only cells are not protected

Pointers: `/rows/0/boundedFootprint`, `/rows/2/boundedFootprint`, `/rows/3/boundedFootprint`, `/rows/4/boundedFootprint`, `/rows/5/boundedFootprint`, `/theoremLedger/theorems/3`

Parallel writes omit H_par_join and the aggregate work cell, and the advertised side frames contain only balances, omitting authority/history/work ownership. Shared changes fees, share records and authority outside its listed resources. Atomic changes authority.joint although it is only in reads; async changes observedTick 10->100 although it is only in reads and changes a fee record absent from its footprint. NAM19 writes H_nam19_prefix, an ID not produced by the trace, and omits its repayment duty and actual intermediate/final histories. No declared resource-to-state projection resolves names such as nam19.debt.principal versus debts.nam19.principal. Frame using reads union writes permits mutation of read-only resources, e.g. an oracle value, without violating its equation.

Required fix: Define resource names as canonical paths or a complete alias map. List full account/debt/share/fee/duty/message/history/work/authority reads and writes for every step, aggregate and parallel coordinator. Require unchanged values outside writes, and separate read dependency compatibility from write permission. Include an unrelated populated framed record to make preservation discriminating.

## C06 — Assume-guarantee composition drops necessary assumptions and does not establish compatibility

Pointers: `/theoremLedger/theorems/4/predicate`, `/theoremLedger/theorems/4/definitions`

The conclusion uses (AsmA intersect AsmB) union OpAsm, dropping every assumption held by only one component. Counterexample: no-op A assumes x=0 and guarantees x=0; no-op B has no assumptions and guarantees true. Both have empty histories/authority and compatible disjoint frames. Intersection is empty; at x=1 the composed guarantee is false. Sequential compatibility checks histories/authority but not that A guarantees B assumptions. Async compatibility merely requires names to exist; it cannot imply no double credit/refund. sat, G_op, residual and operator-specific contracts have no precise semantic domains; GA then GB is still a placeholder.

Required fix: Define contracts as typed predicates/trace relations and satisfaction explicitly. Retain the conjunction/union of required assumptions unless discharged by a proved or proposed explicit entailment. Add actual compatibility obligations for each operator, including A guarantee implies B precondition, interference stability, joint financial closure and a unique receive/refund transition rule. State the derived guarantee mathematically.

## C07 — Observation equivalence and associativity omit material state and admissibility hypotheses

Pointers: `/theoremLedger/theorems/5/predicate`, `/theoremLedger/theorems/5/definitions/0`, `/theoremLedger/theorems/5/definitions/1`, `/theoremLedger/theorems/5/definitions/2`

ObsEq compares accounts/transfers/debts/duties/requests/messages/fees/work.spent, omitting authority, work.remainingOrdinary, work.remainingClosure, lifetime, claims, observations and explicit order; positions and LP supply are only mentioned in prose outside the equality tuple. Concrete equal-spent states with (spent, ordinary, closure)=(24,984,16) and (24,992,8) both conserve 1024 and are ObsEq under the formula despite different closure capacity. Same outputs with different residual authority likewise compare equal. rho:Bindable->Bindable uses field-name tags rather than actual finite bound-ID domains. Sequential/parallel associativity is unconditional despite compatibility, boundedness, split allocation, prefix/rejection and cost semantics being undefined.

Required fix: Define a complete observable record including monetary, obligation, authority, budget, order, status and provenance fields, and typed capture-avoiding renaming of actual IDs. State associativity only for both admissible groupings under compatible contracts, the same allocation/cost/serialization policy and equal rejection/prefix observations; define those conditions instead of naming them.

## C08 — Debt and duty discharge permit unrelated or reused payment witnesses

Pointers: `/theoremLedger/theorems/1/definitions/1`, `/theoremLedger/theorems/1/definitions/3`, `/theoremLedger/theorems/6/definitions/0`, `/theoremLedger/theorems/6/definitions/1`

ReduceDebt only requires a Transfer of the same asset and amount. It does not bind sender, creditor, debt ID or single-use allocation. Counterexample: two USDC debts of 50 from Alice to Lender can each cite the same unrelated Bob->Carol transfer of 50; token conservation holds and both stated reduction conditions hold. DutyConserv similarly asks for some SettleTransfer matching asset and amount, allowing the same witness to discharge two duties without identified payees/allocation. SettleTransfer and Novation are not defined AllowedEffects constructors. Out filters only residual=true, so omission/toggling and partial-duty updates need explicit rules; NAM19 already omits its duty from state. Work shutdown and successor reserve conditions lack transition/binding definitions.

Required fix: Tie settlement effects to debt/duty IDs, entitled payees and authorized funding, with injective or quantitatively bounded allocations across all discharges. Define partial payment residual amounts/controllers, authorized write-off and novation with consistent constructor typing. Connect debts to duties and specify closure/exhaustion transitions on complete records.

## C09 — Conservative extension can be satisfied by rejecting every old program

Pointers: `/theoremLedger/theorems/7/predicate`, `/theoremLedger/theorems/7/definitions`

The implication assumes Eval_succ=Prepared before requiring old evaluation and observation equality. A successor that Rejects every admitted old program satisfies the predicate vacuously. Admitted_D0 is defined through already admitted/current atomic acceptance without a pinned admission predicate or precise syntax/profile domain. T is same program text but does not specify a common grammar or state relation. This does not state retention of existing old-program acceptance.

Required fix: Pin P0, D0 syntax, admission and observation/state relation. Require every admitted old execution to have a corresponding accepted successor execution with equal complete D0 observations (and specify the converse/rejection scope as intended). State translation totality on retained admitted syntax; lack of an implemented translator is acceptable, vacuous retention is not.

## C10 — Negative controls do not isolate the stated validation stages or bind complete unchanged states

Pointers: `/mutations`, `/commonAcceptance/fixtures/genesisUnfundedDebt`, `/commonAcceptance/fixtures/adminDebtErasure`

All ten mutations assert satisfiesEarlierAssumptions=true without a complete candidate plan/effect/post-state and defined stage order. NAM19 mutations 0 and 1 retain only postDebt as unchangedFinancialState, omitting accounts, fees, work, authority and histories. Reordered NAM events and skipped sequential predecessor already violate the corresponding hard intent constraints; partial atomic commit and wrong repayment asset do likewise, before the named later-stage checks if claims are checked in listed order. Alias mutation replaces B USDC authority with an Alice WETH action without a concrete independently authorized B envelope. Late receive uses a ctor absent from its retained authority and an empty escrow, so a later HistoryCompliance failure is not isolated. Its fee preservation 1 and Bob=0 are correct as expected values, but flags and partial contexts are insufficient evidence.

Required fix: Define validation order and supply each mutation as a complete typed candidate plus complete unchanged pre/prefix state. Derive all earlier-stage predicates explicitly from that context. If a mutation intentionally fails multiple gates, identify the actual first gate rather than asserting later-stage isolation. For prefix failures retain prior fees, work, histories, authority and duties.

## C11 — Async recovery lacks a complete competing receive transition

Pointers: `/traces/5/steps`, `/traces/5/expected/race`, `/traces/5/expected/successor`, `/rows/5/observablePrefixes`, `/theoremLedger/theorems/4/definitions/0`

The timeout/refund path has correct lock/fee arithmetic and a post-refund receive rejection. However Created moves directly to Refundable while the advertised state/order vocabulary includes Pending. There is no fully typed receive/settle effect, destination payment source, authorization, consumed history/nonce or deterministic deadline tie rule for competing valid receive and refund candidates from the same live lock. unique=true and excludes=[Received,Finalized] do not specify an exclusive successor relation. The external finality assumption is honestly labeled, but even under that assumption the local transition predicate is incomplete.

Required fix: Specify the finite message state machine, legal ticks and boundary priority, authenticated observation inputs, escrow-backed receive settlement, exact-once nonce/history consumption, and complete alternate receive/refund successor records. Add a same-pre-state competing-transition control and explain Pending or remove it consistently. Keep finality explicitly assumed.

## Independent financial computations

### heldouts:NAM19-capitalization:trace

IED lender USD 10000->5000, borrower 0->5000; total USD 10000 unchanged. RR/IPCI/RR transfer zero and fee zero.

5000*(8/100)*(90/365)=7200/73=98.630136986301369863013698630136986301369863013698630136986301369863013698630137. First reset=0.110567901234567900. Capitalized interest=236.46135633350245068493150684931506849315068493150684931506849315068493150684932; principal=5236.4613563335024506849315068493150684931506849315068493150684931506849315068493. Second reset=0.111679012345679000. Upstream IEEE captures differ as explicitly disclosed.

Debt remains 5236.461356333502 upstream USD; repayment duty must survive but canonical postState.duties is empty.

8+4+8+4=24; ordinary 1008-24=984; closure 16; sum 1024.

Input and all 30 source result records match the hash-pinned local NAM19 file. Remaining 26 are preserved, not independently financially re-derived.

### composition:sequential:trace

Alice USDC 100->90->70; Bob 0->10->30; Treasury 0. Total100. Fee0. Gross30, cap80, residual50.



No debt/duty in finite records; A creates H_seq_A, B consumes it and produces H_seq_B.

8+8=16; ordinary992; closure16; lifetime1024.



### composition:disjoint-parallel:trace

Alice WETH20->13, VaultX0->7; Bob USDC40->29, VaultY0->11; asset totals20 WETH and40 USDC. Fees0. Gross7 WETH/11 USDC exactly exhaust listed caps.



No debts/duties. Monetary frames are disjoint. Complete authority/history/work frames and coordinator are not specified.

1008=504+504; 16=8+8; each spends10 leaving494 ordinary+8 closure; join ordinary988, closure16, spent20. Join/split costs are not independently defined, and no full child lifetime records are supplied.



### composition:shared-state-interleaving:trace

AB: floor(100*20/120)=16 WETH; pool120/84. B supplies12 USDC and floor(12*84/120)=8 WETH; mint min(floor(12*100/120),floor(8*100/84))=9 LP. Final Alice29USDC/16WETH; Bob8USDC/12WETH/9LP; Maker100LP; pool132USDC/92WETH; Treasury1USDC. Totals170USDC,120WETH,109LP (mint9).

BA: B adds12/12 and mints12; pool112/112 supply112. A receives floor(112*20/132)=16. Final pool132/96 supply112; Bob8USDC/8WETH/12LP, Alice29USDC/16WETH, Treasury1. AB != BA. Floor matching is approximate ratio, and is accepted only as the disclosed local integer toy.

No listed debt/duty; fee1 persists after swap prefix; authority accounting incomplete.

12+12=24; ordinary984; closure16.



### composition:atomic-synchronization:trace

Alice USDC51+50-50-50-1=0; WETH47. Lender100-50+50=100. Pool1050USDC/953WETH. Treasury1USDC. Total1151USDC and1000WETH. Actual Alice gross101USDC, not51.

floor(1000*50/1050)=47 WETH. Minimum initial Alice USDC51 funds repayment principal50+fee1 after spending all borrowed USDC in swap.

Debt50 created and repay50 eliminates it within joint envelope in intended chronology; no residual. expected.effects does not represent chronology/full application sufficiently.

24 ordinary; residual984 ordinary+16 closure.



### composition:asynchronous-messaging:trace

Send at10: Alice40-25-1=14, Escrow25, Bob0, Treasury1. Tick100 timeout preserves balances/duty25. Refund: Alice39, Escrow0, Bob0, Treasury1; total40. Fee stays1. Alice gross26, cap200, remaining174 before and after refund; Escrow refund debit25.



Lock duty25 survives timeout and is discharged by actual escrow refund. Post-refund receive must leave all these values unchanged; competing receive-first state not fully defined.

8+4+8=20; ordinary988; closure16; lifetime1024.



## Negative controls

- `heldouts:NAM19-capitalization:mut:erase-nominal-on-ipci` (TransitionValidity): Nominal rejection is economically justified; rejection baseline is only a debt subrecord and no complete IPCI candidate with current accrual/date context is supplied.
- `heldouts:NAM19-capitalization:mut:reorder-ipci-rr` (HistoryCompliance): Pinned IPCI-before-RR order is correctly identified; hard intent also rejects, and unchanged state only references a debt subrecord.
- `composition:sequential:mut:skip-predecessor` (HistoryCompliance): B is funded for20 but required A history is absent. Rejection is justified; hard intent already requires B consumes H_seq_A, so later HistoryCompliance isolation is unestablished.
- `composition:sequential:mut:refresh-work-on-join` (TransitionValidity): After A, balances90/10, spent8, ordinary1000, closure16, gross10/rem70 survive. Reset to spent0/ordinary1008 is invalid despite both snapshots conserving1024; transition monotonicity must be defined. Unchanged record is partial.
- `composition:disjoint-parallel:mut:alias-overlap` (TransitionValidity): Both mutated WETH transfers7+3 are funded by20, but they share Alice/VaultX writes. Correct conflict example; B independent authority and typed context are missing.
- `composition:shared-state-interleaving:mut:commute-without-condition` (IntentRefinement): AB pool132/92/109 differs from BA132/96/112; commutation claim is false. The context remains a claim plus references rather than a full mutated candidate.
- `composition:atomic-synchronization:mut:partial-commit` (TransitionValidity): Borrow/swap tentative state would be Alice51USDC/47WETH, Lender50USDC, pool1050/953, Treasury0, debt50. Discarding repay breaks hard all-leg intent as well as atomic validity. Baseline balances must revert51/100/1000 and WETH0/1000, debt empty, fee0.
- `composition:atomic-synchronization:mut:repay-with-weth` (TransitionValidity): 47WETH cannot settle50USDC debt; fee1 is prefunded. Whole envelope rejection is justified, but hard repay-asset intent and missing WETH authority preclude isolated later-stage acceptance.
- `composition:asynchronous-messaging:mut:treat-as-atomic-sync` (IntentRefinement): Immediate Bob credit with no lock duty is inconsistent with async intent. No full typed funded attempted-credit effect/state is supplied.
- `composition:asynchronous-messaging:mut:late-receive-after-refund` (HistoryCompliance): After refund Alice39/Escrow0/Bob0/Treasury1, gross26/rem174 and spent20/rem988/closure16 must remain. Correct already-charged fee handling. Missing ReceiveMessage authority/live-lock funding and unspecified validation order prevent isolated HistoryCompliance evidence.

## Eight proposed theorem statements

- `type-preservation`: Separating expression and state typing is a real improvement. StateWellTyped still mismatches record/quantity types, lacks full finite domains and expression typing rules. Proposed Eval alone does not close these definitions. C02/C04.
- `asset-indexed-accounting`: Token conservation equation is financially appropriate once quantified over accepted transitions and integer differences, but debt settlement is underconstrained and token/nominal effect elaboration is incomplete. C04/C08.
- `authority-safety`: Constructor-and-bound membership ignores required amount variants, principals, recipients, cumulative usage and mint/pool rights. Split partition prose does not provide complete finite child ownership/state. C03/C04.
- `frame-noninterference`: Frame restriction is a concrete mathematical shape, but resource projection is undefined, actual writes are omitted and union(reads,writes) fails to protect read-only cells. C05.
- `assume-guarantee-composition`: Unsound loss of unilateral assumptions; compatibility and guarantee derivation remain incomplete. Explicit counterexample in C06.
- `structural-associativity`: ObsEq omits material observations; unconditional associativity lacks typed operational/compatibility/budget hypotheses. Explicit reserve counterexample in C07.
- `obligation-preservation`: Duty record conservation is better specified, but settlement can be reused/unrelated and partial/successor/controller semantics are missing. Canonical NAM19 state already loses a duty. C02/C08.
- `conservative-extension`: One-way conditional permits reject-all successor; D0 admission remains indirectly/circularly named and unpinned. C09.

## Verification and boundary

All frozen owned hashes and the input hash matched before review and on final verification. All four unique local source hashes matched. NAM19 input and all30 upstream result records are preserved. Concrete local references resolve. All eight theorem entries retain empty axioms and mechanizedEvidence. Independent arithmetic used Fraction/Decimal and integer transfers; supplied verification commands were not executed.

This review does not accept full RP01, SP01, successor semantic freeze, runtime/K/native/ledger execution or completion of the twelve sprints. A revised frozen candidate needs a fresh review.
