Composition candidate-03: BLOCKED

Independent system-described GPT-6 review; requested routing GPT-6 Astra high. Exact serving identifier is unavailable. Candidate `603246fd7465bacedb7ff5a6a9e51d1b591b099826d11f476f15bb98f077a89a`, 708830 bytes. Scope is the finite toy design only; no Moriarty evaluator, K, proof, network, semantic-freeze, Preview or sprint acceptance.

The supplied tests pass150/150 and two independent clean generations exactly reproduce the frozen JSON. Those facts do not establish full transition correctness. All six Transfer/Mint cash subreplays and aggregate histories reconcile, but literal Lock application and several state/authority updates contradict the stored results.

CM01: The finite model and tests do not evaluate the claimed complete transition relation

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:299`

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:1525`

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.test.py:494`

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.test.py:243`

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.test.py:897`

- The generator transforms a seed and writes dictionaries of equation strings. apply_equations returns data; no complete step/replay/schema/claim evaluator applies those equations to reconstruct the states. UpdatePositionReserves assigns 'named' values; SatisfiesHard, restriction maps and first errors remain uncomputed.

- The independent test module does not import the generator, but important tests only check words or record presence. apply.equations-present accepts any one equation-bearing constructor; typed-domains checks words input/output/restriction; first_failure_is_derived accepts evaluated:true and a nonempty relation label.

- Four temporary, separately tampered copies each pass all150 tests: atomic post Lender.USDC100->101; deletion of apply.Transfer; NAM post debt principal5236.461356333502->0; replacement of a failedPredicate with arbitrary text. These are test adequacy probes, not allegations that the frozen file contains those four edits. Exact commands and outputs are retained under commands.compReplay and adversarialProbes.

Required correction: Supply a bounded pure Python finite schema decoder, per-primitive step relation, ordered full-state replay and four-claim/first-error checker. Generate positive and negative materializations from that relation. Tests must independently reject these probes and compare complete states, not consume supplied evaluated flags. This requests the admitted finite toy model, not a Moriarty/K/runtime implementation or proof.

CM02: The purported closed schemas are field-name lists that reject or leave ambiguous their own records

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/schemas`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/jsonDecoder`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/4/preState/positions/0`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/4/preState/authority/joint`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/5/expected/postState/messages/0`

- Products list field names without their types, bounded cardinalities or optional/variant relations. QuantityNat requires an object with value/unit/scale/quantityDomain, while actual account quantities are bare strings. The decoder identifies the FinMap shape but does not give a total conversion of these scalar quantities into those products.

- PositionRecord lists id/reserveUSDC/reserveWETH/lpSupply/kFormula; the atomic position has only the first three. JointEnvelope requires remainingSplitAuthority, absent in its preState. FeeRecord requires annotates, absent from all three fee-bearing post records. MessageRecord requires from/to/escrow, absent from the main refund record.

- schemas.primitiveFields.Transfer requires id, while most positive transfers have none; effectSystem.primitiveConstructors.Transfer omits id entirely. AssignDuty has two nominal variants but no closed field alternatives; ordinary debt duties lack lockId/dutyVariant. ObservationRecord lists id/authenticated/tick although NAM source observations lack those fields.

- Documentary projection is a list including unit/scale and several derivation fields, with no total typed projection for notes, accountsMeta, authority variants, allocation records or other unlisted fields. Silently dropping a unit would be material.

Required correction: Define actual finite closed sums/products with field types and explicit optional variants; give one authoritative scalar/record decoder and metadata projection. Validate every full positive, prefix, negative and alternate state plus each constructor. Do not solve validation by ignoring unknown material fields.

CM03: Authority scope and multi-asset counter projection are corrupted or undefined

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:86`

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:119`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/1/preState/authority/Alice/allowedEffects/0`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/3/preState/authority/Bob`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/canonicalResourceProjection`

- fill_effect_fields walks every dictionary with ctor Transfer, including AllowedEffect descriptors. It inserts from=BobAlias,to=VaultX,amount=bound throughout the positive authority tables. Sequential Alice's actual transfer is Alice->Bob10/20, but her stored descriptor is BobAlias->VaultX80. The descriptor schema lists payeeScope, which is not supplied. These are not coherent sender/payee bindings.

- Shared Bob holds USDC in transferAllowance and WETH in transferAllowanceWETH. The declared Transfer equation always updates authority[from].transferAllowance and the alias authority.P.Transfer.A only maps to that field when asset=A. Bob's WETH transfer8 therefore has no defined valid projection; literal application touches the USDC allowance. PoolP has the same separate WETH allowance and an added zero USDC allowance.

- The correct numeric gross results are retained: atomic Alice101,Lender50,PoolQ47; shared Alice21,Bob USDC12/WETH8,PoolP WETH16. But appended UpdateAuthorityCounters and Transfer both purport to update counters without a precise annotation-versus-update convention, and joint envelope consumption has no primitive equation. System allowedEffects enumerates names but does not bind their permitted state changes.

Required correction: Separate authority descriptors from concrete effects. Use a typed principal/asset/payee scope and an asset-indexed allowance map or explicit total aliases. Derive gross counters once from outgoing transfers and bind joint/system changes to exact transition semantics. Keep Alice's original101 and no-refund-refresh policy.

CM04: Literal primitive application still changes money twice and cannot reconstruct complete poststates

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/effectSystem/apply/Lock`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/5/concretePlan/effects`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/effectSystem/apply`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/4/expected/postState/debts`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/3/expected/postState/fees`

- The async plan includes Transfer Alice->Escrow25 and Lock Alice->Escrow25. apply.Lock explicitly debits/credits cash again, without annotation linkage. Independent literal Transfer+Lock replay ends Alice14,Escrow25,Bob0,Treasury1, while the claimed refund post is39/0/0/1. AccrueFee itself is now correctly annotation-only; preserve that fix.

- Positive atomic SettleTransfer has no apply equation. Async SendMessage,SetObservedTick,UpdateMessageStatus also have none. Alternate Receive includes ConsumeNonce and UpdateObservation without equations. Constructors used in apply, such as SetNominalRate and UpdatePositionReserves, are missing from primitiveConstructors.

- UpdateWork only sets spent=pre+ordinaryDelta; it never updates remainingOrdinary although all positives do. Mint changes account balance and position supply but not the shared shares.Bob quantity that changes in post. ReduceDebt leaves principal0 in the debt record; the atomic post removes D_flash without a deletion rule.

- AccrueFee says the fee record contains annotates, yet all three materialized fee records omit it. UpdatePositionReserves accepts arbitrary named reserve values rather than deriving them from the transfers and selected pool formula.

Required correction: Define every used constructor, its exact preconditions, complete field updates and zero-record retention/deletion policy. Make Lock either the sole cash mover or a precisely linked annotation. Materialize posts only by replay; fee links, share supply, work remaining, allocations and envelope consumption must be actual outputs.

CM05: NAM19 has the right source sequence and intervening accrual idea, but violates exact precision and replay

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/0/concretePlan/effects/4`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/0/concretePlan/effects/5`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/0/expected/postState/debts/nam19`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/schemas/SourceDecimal`

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:606`

- The pinned source's first four events are IED,RR,IPCI,RR. Independent exact arithmetic confirms cap1=5000*.08*90/365 and cap2=5000*(.010567901234567900+.10)*91/365, giving source principal5236.461356333502 after source reporting. The correction preserves those source values and adds the missing cap2.

- The actual second AccrueNominal.accrued has77 fractional digits, exceeding SourceDecimal.maxPrecision50. The first accrued is the rounded source decimal98.63013698630137, while the second is an80-digit Decimal approximation. Exact summation of these actual input strings differs from stored principal by approximately4.50821917808219e-13. 'JSON pins may truncate' does not specify a rounding operator or an exact-to-reported equality relation.

- Only one SetNominalRate effect exists, setting .110567901234567900. Final debt and contract rate are .11167901234567901 after the second RR. No second rate/observation effect realizes that change. contract.notionalPrincipal and nominalInterestRate updates likewise lack a contract projection/update relation.

- NAM negative prefixEffects includes both accruals and AssignDuty amount5236.461356333502, but the stated post-RR prefix has accrued98.63013698630137 and duty5000. It filters out the rate-setting and history/work effects. This is not the creator replay for the claimed prefix.

Required correction: Retain exact source pins and event order. Use a bounded rational or explicitly rounded decimal arithmetic domain with a precise projection to source-reported values. Add the second RR and all contract/observation updates. Reconstruct each NAM prefix, including the5000 duty before capitalization and its amount-aware update, from its actual primitive prefix.

CM06: All ten negative expansions remain the positive plan and first errors are hardcoded

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:1308`

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:1324`

`experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:1333`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/mutations/0/completeCandidate`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/mutations/2/completeCandidate`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/mutations/5/completeCandidate`

- For every one of the ten mutation IDs, completeCandidate.expandedEffects exactly equals the corresponding unmutated positive concretePlan.effects. A few records additionally carry a different effects field; there is no precedence or typed patch application that reconciles them.

- The work-refresh expansion keeps the correct positive UpdateWork; wrong-asset expansion still includes correct USDC repayment; reorder/partial-commit expansions retain the original full positive order and legs. Thus the declared failure is not computed from the named expanded candidate.

- derived_failure and earlier_ok manufacture evaluated:true/holds flags from a dictionary of labels. Every first stage remains IntentRefinement. No schema, state replay or claim predicate is evaluated there. The probe replacing failedPredicate with arbitrary text still passes150 tests.

- Negative prefix states retain old authority objects without the new required variant/system schema, and NAM prefixes disagree with prefixEffects as CM05 details. Numeric committed-prefix cash/work snapshots alone do not establish that earlier complete-state checks pass.

Required correction: Implement exact typed patches over complete legal references and choose one expanded plan/effect/state representation. Evaluate the same schema, primitive replay and claim order for positives and negatives. Derive first failures and prefix rollback records; retain all ten IDs and the valid committed cash/fee/work/gross values.

CM07: Async branch selection and alternate full states remain inconsistent

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/5/messageStateMachine`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/5/expected/alternateSuccessors/receive`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/5/expected/alternateSuccessors/refund`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/traces/5/expected/postState`

- The added deadlineRule correctly makes receive true/refund false at now100 with authenticated finality99. But old transitions,legalTicks.refundableEnabled and boundaryPriority remain: Pending at100 and not Received enables Refundable even with finality99. The file has two conflicting governing relations and no precedence rule.

- Receive declares fromBase='post-step0 Pending'; that committed base already spent8 work. Its effect UpdateWork ordinaryDelta20 would produce spent28, but its post says20/remaining988. It also repeats Alice grossDelta26 although the branch has no outgoing Alice transfers, contradicting UpdateAuthorityCounters precondition.

- Receive transfers Escrow->Bob25 but stores Escrow cumulativeGross0/remaining25 instead of25/0. The main refund has consumedNonces=[], while the alternate refund has[N3]. Main refund lacks Alice.lockConsumed while the alternate marks true.

- Refund alternate declares the same post-step0 base but supplies the entire positive effect list, including send/lock/fee again and ConsumeHistory H_alice_pre, already consumed by that base. Receive relies on an observation only inserted later in its effect list, and its entitlement transform is a Boolean/text record rather than an applied typed transformation.

- RefundLock apply precondition is refundAdmitted=Pending..., while the positive plan changes the message to Refundable before executing RefundLock. The phase relation must distinguish opening refund eligibility from consuming a Refundable lock.

Required correction: Use one total branch/phase relation and explicit authenticated observation input. Replay receive/refund from the exact same committed base with branch-only costs/effects; derive caps, nonce/history consumption, entitlement and complete states. Keep destination truth an explicit assumption.

CM08: Footprint counts fit64 but resource projection and frame completeness remain false

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/canonicalResourceProjection`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/rows/0/boundedFootprint`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/rows/2/boundedFootprint`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/rows/3/boundedFootprint`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/rows/5/boundedFootprint`

- List totals independently match32,14,28,48,39,32 and fit the amended toy64 bound. This corrects the numeric bound issue, not projection completeness.

- The declared parent authority.P mapping lists allowedEffects,transferAllowance,lockConsumed,envelopeConsumed,lockRefundConsumed but omits variant,transferAllowanceWETH,custody,recipientRight,remainingSplitAuthority and other material fields. The alias Bob.Transfer.WETH points only to the USDC transferAllowance and cannot resolve as specified.

- NAM allowedEffects shrinks in post but its write footprint only names lender.Transfer.USD, which maps to the allowance rather than allowedEffects. Every work.conservation string changes outside the work.spent/remainingOrdinary write leaves, with no total documentary projection rule covering it.

- coordinator.work in parallel has no state record or explicit alias. Added allocations.A_shared_fee and allocations.A_repay_D_flash are absent from the empty allocations post records. Async writes omit the alternate receive's Bob cash,O_dest_M3,nonce,recipient-right and received history resources; no branch-specific footprint is supplied.

- New schemas state no field erasure, but atomic authority.Alice.netPrefundNote disappears and multiple post records omit annotation/schema fields. One populated unrelatedRecord equality is not a full frame check.

Required correction: Implement total canonical resource projection including all variants and explicit documentary data. Derive primitive/prefix/branch reads and writes and compare every projected pre/post record outside writes. Count actual distinct accesses under64; reject unresolved aliases and undeclared state changes.

CM09: Composition, observation and obligation definitions remain names and examples instead of finite relations

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/theoremLedger/theorems/4`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/theoremLedger/theorems/5`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/theoremLedger/theorems/6`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/discriminators/partialDuty50to30`

- ContractInput,ContractOutput and FiniteTrace are named but have no constructors or record schemas. rest_in/rest_out/rest_tr are only asserted total. entail and interference_stable have no finite input/output judgments or checkable records. G_shared says GA and GB on the Ord-serialized trace, and G_atomic applies trace guarantees 'on the joint post-state, or Reject all', without the necessary typed restriction.

- CompleteObs adds lockConsumed/envelopeConsumed and says 'all material records', but supplies no total projection over authority variants,workPartition,frame,consumedNonces or allocations. Sort-preserving rho is a real improvement, yet binding occurrences and free-name scope are unspecified. A bag plus a join history does not choose a canonical serialized sequence.

- Associativity is sensibly restricted to both groupings with equal costs/allocations/order/reject prefix, but those admitted-grouping and FirstError relations depend on CM01/CM06's absent checker. This is not a proof demand; it is a missing statement-domain definition.

- The partial-duty discriminator demonstrates50-20=30 and names A_partial_20/T_partial_20, but supplies no funding Transfer or complete legal state. SettleDuty does not define allocation ledger updates. 'Alloc is injective on allocationId' is vacuous for a map unless use-once and aggregate-per-transfer discharge constraints are defined over effects.

Required correction: Define finite typed contracts, restriction/entailment/interference/operator relations and a total observation/renaming projection. Separate payment allocation use from work/authority splits with exact ledger equations. Add a fully funded partial-payment trace and duplicate-allocation negative. Keep theorems proposed; mechanized proof remains out of scope.

CM10: Old Simulation provenance is corrected, but total old-state transport remains unspecified

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/commonAcceptance/oldDomain`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/theoremLedger/theorems/7/definitions/1`

`experiments/moriarty-language/spec/successor/financial-fragments/composition.json#/theoremLedger/theorems/7/definitions/2`

- The pinned retained fixture really has two Simulation results under simulation-only authority/checks. Program/source/grammar/bounds/numeric/evaluator hashes match local files. This is an accepted correction: no old Prepared/Accepted witness is fabricated, and MC04/MC05 production transport is explicitly conditional.

- However D0Obs_complete only lists categories and says it maps old before/after and hashes. It does not map old values notional,principal_due,interest_due,principal_paid,interest_paid,cursor,episode_closed; obligation status Outstanding/Settled; revision,remaining lifecycle budget; or USD_micro/USD_TEST_ASSET settlement into the proposed closed state.

- The exact old fixture ends with remainingNotional4500000000 and two retained Settled obligations, while episode_closed=1 and lifecycle remaining=0. A complete transport must preserve that distinction; accounts alone cannot do it. The new runtimeNat assets list USDC/WETH/LP-USDC-WETH, with no old settlement asset encoding.

- T is called an identity embedding but no old-state input map or successor D0 derivation witness is defined. Pinning a nonempty old domain fixes the prior vacuity setup, but does not make the new partial observation map total.

Required correction: Retain the actual two-step Simulation witness and all source pins. Give explicit total old input/state/effect/observation maps on that exact finite domain, including lifecycle and residual debt/settled duties, and state the corresponding successor derivation obligation precisely. No implementation of the production translator or proof is required for this design gate.

The four adversarial test copies each passed150/150. They changed atomic Lender cash100->101, deleted apply.Transfer, erased NAM principal to0, or replaced failure evidence with arbitrary text. These are test-adequacy probes, separate from actual frozen candidate defects. Exact runnable Python, JSON paths and outputs are retained in the JSON report for the next correction.

All six row/trace IDs, ten mutation IDs and eight proposed theorem IDs remain. All24 local source hashes, five owned hashes and two bound input hashes match; review-input/fixture/pins hashes remain unchanged. Actual old Simulation provenance and source arithmetic are correctly retained. Full D0 transport still needs a definition.

The JSON report records every C01-C11 disposition, every trace/mutation/theorem disposition, independent arithmetic, commands/exits, source checks and limits. The next correction should implement the admitted bounded finite schema/replay/claim checker and derive the artifact from it; adding more equation strings or evaluated flags will not resolve these findings.

