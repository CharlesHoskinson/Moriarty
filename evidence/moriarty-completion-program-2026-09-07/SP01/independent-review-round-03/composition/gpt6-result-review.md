# Independent composition result review

**BLOCKED** — immutable candidate-02 specified-only design.

Candidate SHA256: `791a5ed0fb5e02ba4e7ad8baa0fd173bc095939329601df089dcb261fec1c834`. All3 owned files, both bound inputs and all4 source pins match. Author exit1/cancelled after24calls is preserved; this recovery review does not infer author success.

Reviewer: system-described GPT-6. Requested routing: fresh GPT-6 Astra high. Exact serving model identifier is unavailable.

All six step-level cash and work replays match the retained numbers. Alice atomic gross101 is correct. These checks do not establish the new primitive relation, which double-counts fees and has missing fields/permissions. All eight theorem entries remain proposed with empty axioms and mechanized evidence.

## C01 — PARTIALLY_RESOLVED_NONBLOCKING: Step and aggregate history arithmetic corrected; plan history vocabulary remains inconsistent

Paths in composition.json: `/commonAcceptance/predicates/HistoryCompliance`, `/traces/0/concretePlan/producedHistories`, `/traces/0/expected/producedHistories`.

Independent replay confirms every step consumes live prefix histories and every final history set agrees. NAM19 concretePlan.producedHistories still lists four internally produced IDs, whereas expected.producedHistories lists only H_nam19_rr2; shared and async have the same distinction. The new orderedTranscript makes the intended distinction recoverable.

Required correction: Explicitly name concretePlan fields internal transcript outputs versus external outputs. This is not independently blocking; full prefix semantics depend on C04.

## C02 — BLOCKED: State schema still does not type its supplied records

Paths in composition.json: `/commonAcceptance/jsonDecoder`, `/theoremLedger/theorems/0/definitions/0`, `/traces/2/preState`, `/traces/2/expected/postState/authority`, `/commonAcceptance/effectSystem/primitiveConstructors`.

Decimal/Nat separation, bound values and canonical NAM19 residual duty are improvements. But CompleteState omits workPartition/frame although parallel preState includes them. StateWellTyped requires authority entries to contain allowedEffects and transferAllowance; parallel post Alice/Bob lack allowedEffects, and pool/joint/Treasury/Bob/Escrow variants omit required transferAllowance. ObservationRecord, ContractRecord, AllowanceRecord and positions/claims/requests/messages/orders field schemas are still undefined. SourceDecimal has no grammar, bounded precision/length or arithmetic relation. Declared constructors also fail on positive effects: NAM19 CreateDebt lacks debtor/creditor/unit/scale and AccrueNominal lacks observationRef; shared Mint lacks unit/scale; atomic CreateDebt lacks unit/scale; async AssignDuty lacks required debtId.

Required correction: Define actual bounded closed sum/product records and primitive field types, including optional variants and projection of documentary metadata. Supply records accepted by that schema. Keep source decimal oracle domain explicitly separate from runtime Nat theorems.

## C03 — BLOCKED: 101 cap arithmetic corrected, but effect-to-authority application remains incomplete

Paths in composition.json: `/traces/4/preState/authority/Alice`, `/traces/4/expected/postState/authority`, `/traces/4/expected/effects`, `/theoremLedger/theorems/2/definitions/0`.

Step-transfer replay confirms Alice gross 101, Lender gross 50, PoolQ gross 47, correct stored residuals and prefund 51. Shared gross/residual counters are also arithmetically correct. However atomic effects contain UpdateAuthorityCounters only for Alice while post also changes Lender and PoolQ counters and consumes joint authority. AllowedEffects membership checks only ctor and per-effect amount, without sender/payee binding or cumulative cap enforcement. Every trace contains history/work/counter constructors absent even from the union of all pre-authority allowedEffects. Alice can issue two Transfer60 effects under a Transfer101 per-effect bound without the stated membership predicate detecting gross120.

Required correction: Retain cap101. Specify principal/asset/recipient-scoped authorization, exact cumulative debit updates and system-effect authorization. Derive all changed counters and envelope consumption from effects; do not use arbitrary provided counter deltas.

## C04 — BLOCKED: Primitive elaboration is not a defined complete transition and newly double-counts fees

Paths in composition.json: `/commonAcceptance/effectSystem/elaboration`, `/commonAcceptance/effectSystem/apply`, `/commonAcceptance/effectSystem/primitiveConstructors/AccrueFee`, `/traces/3/expected/effects`, `/traces/4/expected/effects`, `/traces/5/expected/effects`, `/traces/0/canonicalIntent/hard/2`.

apply only says to update fields named by constructors; it gives no equations/preconditions and is called both a total function and a relation. Shared/atomic/async effects each contain the fee Transfer plus AccrueFee, which is explicitly defined as another Transfer. Literal application charges two fees: atomic Alice ends -1 USDC, Treasury2, gross102, instead of 0/1/101. Shared becomes Alice28/Treasury2 instead of29/1; async38/2 instead of39/1. No linkage makes AccrueFee annotation-only. Position reserves/supply, NAM19 observation/rate changes, and some authority changes have no applicable primitive. NAM19 H3 states post.principal=pre.principal+pre.accrued, but immediate IPCI prefix is5000+98.63013698630137=5098.63013698630137, not5236.461356333502: missing intervening accrual is material. Four claims are now conjoined syntactically, but undefined apply/hard satisfaction cannot establish positive acceptance. Genesis matching Transfer still lacks payer/payee binding; admin/genesis fixtures lack complete typed candidate context.

Required correction: Give finite per-constructor transition equations and failure preconditions. Represent each fee movement exactly once. Bind typed plans to ordered step effects including all state changes and intervening NAM accrual. Reconcile positive hard predicates before claiming them as acceptance witnesses; close genesis/admin funding relations.

## C05 — BLOCKED: Correct frame equation is defeated by incomplete paths and oversized footprints

Paths in composition.json: `/theoremLedger/theorems/3/definitions/0`, `/commonAcceptance/canonicalResourcePathGrammar`, `/commonAcceptance/resourceAliasMap`, `/rows/3/boundedFootprint`, `/rows/4/boundedFootprint`, `/traces/3/expected/postState/positions`, `/traces/4/expected/postState/positions`.

Frame now protects outside writes and populated unrelatedRecord is preserved. But shared has40 read+write entries and atomic33 against theorem maximum32. Both mutate positions PoolP/PoolQ while no positions path occurs in their write sets or canonical grammar. Paths authority.Alice.Transfer.USDC do not directly project the actual authority.Alice.transferAllowance and no complete projection is defined. Parallel pre workPartition/frame disappear from post without a closed schema/projection. Thus the positive examples do not establish their own frame predicate.

Required correction: Supply a total canonical resource projection with parent/child semantics; cover positions, allocations and all metadata/state fields, preserve outside writes, and reconcile declared footprint bounds with actual examples through an explicit bounded amendment or a valid smaller representation.

## C06 — BLOCKED: Assumption union improved but discharge and contracts remain ill-defined

Paths in composition.json: `/theoremLedger/theorems/4/predicate`, `/theoremLedger/theorems/4/definitions/0`, `/theoremLedger/theorems/4/definitions/1`.

The original unilateral-assumption intersection defect is explicitly removed. New conclusion AsmA AND AsmB AND OpAsm AND NOT Discharged mixes typed predicates with an empty/set-valued Discharged and does not express removal of proved assumptions. GA implies AsmB compares predicates on traces without defining prefix/output-to-input projection. Interference stability is only outside-write frame equality; it does not preserve another component assumption within shared cells. G_shared remains ordered composition without an operator definition and G_atomic mixes GA/GB trace predicates with a joint post-state.

Required correction: Define typed input/output/trace contract domains and restriction maps. Express discharged assumption set difference with explicit entailment obligations; define sequential/shared interference and guarantees as relations, rather than textual placeholders.

## C07 — BLOCKED: Observation and associativity conditions improved but still lack complete typed interpretation

Paths in composition.json: `/theoremLedger/theorems/5/definitions/0`, `/theoremLedger/theorems/5/definitions/1`, `/theoremLedger/theorems/5/definitions/3`.

Work remaining/closure and authority counters are now observed, resolving the old equal-spent discriminator. However rho is an untyped bijection over a union of HistoryId/DutyId/etc without explicit sort preservation or binding scopes. CompleteObs omits meaningful authority fields such as lockConsumed/envelopeConsumed while those appear in states. Projection from heterogeneous authority records is undefined. Alloc is defined as settlement allocation, not child work/authority allocation; disjoint-parallel order is declared a bag while Serialization and orderedTranscript require sequence equality. Admitted permits any well-typed reject without defining admissible evaluation/first-error relation.

Required correction: Define a total complete observation projection, sorted binders/free names and sort-preserving renaming. Separate settlement allocation from work/authority split allocation. State exact serialization, cost, valid/rejected prefix domains and restricted admissibility hypotheses.

## C08 — BLOCKED: Payment allocation improves but duty equation and receive branch remain inconsistent

Paths in composition.json: `/theoremLedger/theorems/1/definitions/4`, `/theoremLedger/theorems/6/definitions/0`, `/traces/5/concretePlan/effects/6`, `/traces/5/expected/alternateSuccessors/receive/effects/2`.

Creditor, transfer-id and aggregate allocation bounds are real improvements. Partial-duty conservation is still contradictory: a pre duty50 partially paid20 leaves the same residual id with amount30; Out(post)=(Out(pre) minus Discharged) union Created cannot update the retained record unless it is both discharged and recreated, forbidden by disjointness. Async assigns DutyLock_M3 payee Alice but the receive branch discharges it using SettleDuty payee Bob, violating the stated entitled-payee rule. Transfer schema does not declare id although settlement references require t.id.

Required correction: Define amount-aware partial residual update and unique per-discharge allocation use, with explicit target equality. Give lock duty branch-dependent entitlement or a typed authorized transformation and bind every transfer id. Reconcile write-off/novation and positive-duty lifecycle with this relation.

## C09 — BLOCKED: Retention direction fixed but old domain still unpinned and witness invalid

Paths in composition.json: `/theoremLedger/theorems/7/predicate`, `/theoremLedger/theorems/7/definitions/0`, `/theoremLedger/theorems/7/definitions/1`, `/theoremLedger/theorems/7/definitions/2`.

The old-Prepared implies successor-Prepared direction removes the previous direct vacuity. However atomicCoreRetention is a name, with no old profile/program/grammar/semantics hash or concrete admission/state relation. Input atomicLoanSwapRetention is only a prose retention instruction. The fragment atomic Borrow/Swap/Repay toy is claimed to witness old admitted syntax despite not being an old source program or old accepted execution. D0Obs_complete is a shortened projection, omitting old histories and much authority/state evidence. Nonempty syntax alone does not imply any old Prepared execution, so reject-all can still satisfy the implication if Eval_P0 has none in the undefined domain.

Required correction: Pin actual old admitted program bytes, grammar/profile/evaluator relation and full D0 observation/state mapping. Include a concrete old accepted execution witness. State total translation and retained execution obligations on that exact domain; no implemented translator is demanded at this gate.

## C10 — BLOCKED: Rollback records expanded but candidates and earlier-stage derivations remain incomplete

Paths in composition.json: `/mutations`, `/commonAcceptance/validationOrder`, `/commonAcceptance/fixtures`.

All ten first stages were relabeled IntentRefinement and completeCandidate merely wraps mutatedInput/mutatedContext prose with no full typed plan/effect/poststate or defined patch elaboration. EarlierStagesDerived still contains holds:true plus text, not evaluation of the defined predicate. Mutation2 Transfer lacks unit; mutation4 Transfer variants lack unit/scale; mutation9 ReceiveMessage lacks escrowSource and destFinalityObservationId. Therefore well-typed-before-intent claims cannot follow the new required field schema. NAM prefix duty5000 is now present in rollback records but no prefix AssignDuty effect creates it. Expanded post-A and post-refund snapshots correctly preserve committed cash/fees/work/gross usage.

Required correction: Define full candidates or exact typed patches over explicitly referenced complete contexts and deterministic expansion. Check actual first errors under the same schema and finite acceptance relation as positives. Retain correct committed-prefix rollback snapshots and repair NAM prefix duty creation.

## C11 — BLOCKED: Async race still has two enabled deadline choices and incomplete successors

Paths in composition.json: `/traces/5/messageStateMachine/legalTicks`, `/traces/5/messageStateMachine/boundaryPriority`, `/traces/5/expected/race/samePreReceiveVsRefund`, `/traces/5/expected/alternateSuccessors`.

Pending is now explained and escrow-funded receive amounts are correct. At current tick100, Pending with authenticated finality observation tick99 enables receive (99<100) and timeout (100>=100 and not Received). BoundaryPriority only gives refund priority when finality.tick>=100; it does not choose a winner for99. exactly one admitted is an assertion, not a deterministic branch predicate. Alternate receive has postAccounts/postDuties/status only, no complete post work/authority/observations/history state; refund is a prose via reference. Receive has no work/counter effect or supplied O_dest_M3 record. Nonce consumption is a text field without a complete consumed-nonce state relation. Receive additionally fails the duty-payee rule in C08.

Required correction: Specify a total authenticated observation domain and deterministic same-base tie rule; fully materialize both branch states and typed effects, including costs, authority, nonce/history consumption and duty entitlement. Keep foreign/destination finality assumed, not proved.

## All trace dispositions

- heldouts:NAM19-capitalization:trace: BLOCKED_AS_ACCEPTED_DESIGN_TRACE. Step cash/history/work replay agrees; gross {"lender:USD": "5000"}; work {"spent": 24, "ordinaryRemaining": 984, "closureRemaining": 16, "matches": true}; footprint entries 32.
- composition:sequential:trace: BLOCKED_AS_ACCEPTED_DESIGN_TRACE. Step cash/history/work replay agrees; gross {"Alice:USDC": "30"}; work {"spent": 16, "ordinaryRemaining": 992, "closureRemaining": 16, "matches": true}; footprint entries 14.
- composition:disjoint-parallel:trace: BLOCKED_AS_ACCEPTED_DESIGN_TRACE. Step cash/history/work replay agrees; gross {"Alice:WETH": "7", "Bob:USDC": "11"}; work {"spent": 20, "ordinaryRemaining": 988, "closureRemaining": 16, "matches": true}; footprint entries 28.
- composition:shared-state-interleaving:trace: BLOCKED_AS_ACCEPTED_DESIGN_TRACE. Step cash/history/work replay agrees; gross {"Alice:USDC": "21", "PoolP:WETH": "16", "Bob:USDC": "12", "Bob:WETH": "8"}; work {"spent": 24, "ordinaryRemaining": 984, "closureRemaining": 16, "matches": true}; footprint entries 40.
- composition:atomic-synchronization:trace: BLOCKED_AS_ACCEPTED_DESIGN_TRACE. Step cash/history/work replay agrees; gross {"Lender:USDC": "50", "Alice:USDC": "101", "PoolQ:WETH": "47"}; work {"spent": 24, "ordinaryRemaining": 984, "closureRemaining": 16, "matches": true}; footprint entries 33.
- composition:asynchronous-messaging:trace: BLOCKED_AS_ACCEPTED_DESIGN_TRACE. Step cash/history/work replay agrees; gross {"Alice:USDC": "26", "Escrow:USDC": "25"}; work {"spent": 20, "ordinaryRemaining": 988, "closureRemaining": 16, "matches": true}; footprint entries 32.

## All mutation dispositions

- heldouts:NAM19-capitalization:mut:erase-nominal-on-ipci: BLOCKED_INCOMPLETE_TYPED_CONTROL. Nominal erasure should reject; corrected positive IPCI itself fails H3 and prefix duty lacks creating effect. Rollback equals supplied prefix: True.
- heldouts:NAM19-capitalization:mut:reorder-ipci-rr: BLOCKED_INCOMPLETE_TYPED_CONTROL. Reordering contradicts hard order; full typed event candidate/prefix accrual remains missing. Rollback equals supplied prefix: True.
- composition:sequential:mut:skip-predecessor: BLOCKED_INCOMPLETE_TYPED_CONTROL. Missing predecessor intent is plausible; Transfer lacks required unit. Rollback equals supplied prefix: True.
- composition:sequential:mut:refresh-work-on-join: BLOCKED_INCOMPLETE_TYPED_CONTROL. Post-A cash90/10, spent8, ordinary1000, gross10/rem70 are preserved; reset remains forbidden but candidate relation incomplete. Rollback equals supplied prefix: True.
- composition:disjoint-parallel:mut:alias-overlap: BLOCKED_INCOMPLETE_TYPED_CONTROL. Overlap is real, but BobAlias ctor permission does not establish authority over Alice funding; required Transfer fields missing. Rollback equals supplied prefix: True.
- composition:shared-state-interleaving:mut:commute-without-condition: BLOCKED_INCOMPLETE_TYPED_CONTROL. Independent AB/BA arithmetic differs:132/92/supply109 vs132/96/supply112. No complete alternate typed candidate. Rollback equals supplied prefix: True.
- composition:atomic-synchronization:mut:partial-commit: BLOCKED_INCOMPLETE_TYPED_CONTROL. All-or-none intent rejects partial commit; original51/100/1000 prefunds and zero debt/fee rollback snapshot preserved. Rollback equals supplied prefix: True.
- composition:atomic-synchronization:mut:repay-with-weth: BLOCKED_INCOMPLETE_TYPED_CONTROL. WETH47 cannot discharge USDC50. Full effect candidate and actual earliest schema/intention gate not derived. Rollback equals supplied prefix: True.
- composition:asynchronous-messaging:mut:treat-as-atomic-sync: BLOCKED_INCOMPLETE_TYPED_CONTROL. Async-to-atomic relabeling conflicts with operator intent; complete funded attempted credit absent. Rollback equals supplied prefix: True.
- composition:asynchronous-messaging:mut:late-receive-after-refund: BLOCKED_INCOMPLETE_TYPED_CONTROL. Refunded39/0/0/1 balances, fee1, spent20, remaining988 and Alice gross26/rem174 preserved. Receive fields incomplete; first-stage claim unestablished. Rollback equals supplied prefix: True.

## All theorem dispositions

- type-preservation: BLOCKED_PROPOSED_STATEMENT (C02, C04); axioms=[], mechanizedEvidence=[].
- asset-indexed-accounting: BLOCKED_PROPOSED_STATEMENT (C04, C08); axioms=[], mechanizedEvidence=[].
- authority-safety: BLOCKED_PROPOSED_STATEMENT (C03, C04); axioms=[], mechanizedEvidence=[].
- frame-noninterference: BLOCKED_PROPOSED_STATEMENT (C05); axioms=[], mechanizedEvidence=[].
- assume-guarantee-composition: BLOCKED_PROPOSED_STATEMENT (C06); axioms=[], mechanizedEvidence=[].
- structural-associativity: BLOCKED_PROPOSED_STATEMENT (C07); axioms=[], mechanizedEvidence=[].
- obligation-preservation: BLOCKED_PROPOSED_STATEMENT (C08, C11); axioms=[], mechanizedEvidence=[].
- conservative-extension: BLOCKED_PROPOSED_STATEMENT (C09); axioms=[], mechanizedEvidence=[].

## Limits and next action

Correct finite schemas/primitive equations and regenerate complete positive/negative/alternate records; resolve C02-C11 and clarify C01 plan history fields, then freeze and independently review a new candidate. Do not promote candidate-02 based on author claims or scalar cash checks.

No implementation, build, compiler, runtime test, proof, network, wallet or Git mutation was performed. Bounded local Python arithmetic is review evidence only. The review allowance is not cgroup enforcement. Only these JSON/Markdown reports were written. No full map, semantic freeze, Preview, runtime or sprint acceptance. Midnight Preview milestone checks and proof-bearing financial integration acceptance remain downstream.
