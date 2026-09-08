# RP01-MC02 loan and swap subset design

Status: specified design for existing `moriarty-bounded-atomic/1` behavior. Pending parent verification and independent GPT-6 review. Not a successor semantic freeze.

This note records the admitted loan first-period accrue-then-settle path and the swap exact-input minimum-receive-then-close path. The two `.mori` sources are synthetic bookkeeping examples. They are also the initial restricted Compact mapping examples. They are not live custody, ACTUS-wide conformity, general AMM behavior, exact-output semantics, complete PCD, or ledger acceptance.

The worker who wrote this file did not run the evaluator, Compact, tests, or any command. Numeric financial values come from independent integer arithmetic and from constants already embedded in the sources. Envelope hashes stay symbolic. Tests in `semantics.test.mjs` and `lowering.test.mjs` are observation evidence, not definitions.

Shared maps live in `traces.json`. Reuse them by reference. Do not copy those objects into this note.

- `negativeFixtureConvention`
- `sharedEvaluationDependencies` (`shared-eval-deps-bounded-atomic-1`)
- `sharedObservationMap`
- `captureMetadata`
- `networkMilestone`

`captureMetadata.authoredBase` preserves git prefix `f173`. Historical authored text dated SP01.8 atomic implementation acceptance as pending. That history stays. Bind current independent local atomic acceptance separately: code `c30aaf5c0d091b1efea7e042d0fcbd539ca6cc51`, evidence `b296ce95afae722177b5bba482d1d195fa334d1e`. SP01.8 is no longer pending. This SP01.6 record remains pending-review and does not self-approve. `candidateHash` and output SHA-256 stay null for parent freeze. `reviews` stay empty.

User-required Midnight network milestone tests remain open. Assignment: `raw/assignments/moriarty-midnight-milestone-testing-2026-09-07.md` (exists in main after the authored base). SP05 still needs meaningful loan and swap Preview transactions, finalized receipts, full comparisons, and rejection controls after separately reviewed fixtures, custody, contracts, and campaign. Source, spec, and local checks do not discharge them. Mandatory-proof SP09 and later network gates stay unchanged. Full RP01 and native subset RP01-MC03 stay incomplete.

## Inputs

Authoritative source:

- `experiments/moriarty-language/spec/examples/loan.mori` SHA-256 `1e1e61158ef80d44aa326399731440971fe50de7147ae5fb04e3fb36c48fef49`
- `experiments/moriarty-language/spec/examples/swap.mori` SHA-256 `0c2217365f2e518ec70835cf05a09150253d2df334e7dcf0504fd8c8d887051e`

Normative profile text used for state, effect, authority, bounds, and settlement:

- `spec/semantics.md` SHA-256 `d2c62ea1e34a345753b319532d85652d29b1946b2d77d1f60f9518b01c519002`
- `spec/typed-schemas.md` SHA-256 `6ef3383ad42b7ea2a22822f8116c7e182cbe8b476ef01e9cd3ef899679da4f04`
- `spec/bounds.json` SHA-256 `b548641a1a9d74bab68ba699ffb1e2350fa0889d61b8704e98216f9d4a6c3664`

Observation evidence, not definitions:

- `tests/semantics.test.mjs` SHA-256 `3eac34c2bdb2fd818991d39dfe815ba5ab8884e84bb9210f2ae5d7b5eacadcef`
- `tests/lowering.test.mjs` SHA-256 `0057938310ca2630cbf7475d9d03479cd8b96816f7dbedb013ea9ff66789ecdd`

Independent older capture:

- `evidence/moriarty-completion-program-2026-09-07/MC01/profile-04/evaluator/legacy-reference.json` SHA-256 `7d4ed0761603978c331a7b70910dc53721cea2e0b0a1764cf9da4f93a13cb032`

That capture records executed legacy R2 values and the source digests of `experiments/moriarty-developer-mock/src/language/core.ts` and `packages.ts` as they existed for that capture. The current test renaming map is the provenance bridge. This note does not claim those old paths are the current runnable evaluator.

Gate contract:

- `openspec/REPORT-RECONCILIATION-2026-09-07.md` SHA-256 `a1a1661fc0d554d8e469651fdbd42c41e7c9e2115c8550f369742dcee94bd5da`
- `openspec/sprints/sp01-financial-contract-and-execution-admission.md` SHA-256 `fdb586d9bf05f2fbf7e9c2294be49854528db20e575faf9aaa1144b033908172`

## Sample-specific source limitation

Both examples pin expected results as source constants and then guard them.

Loan constants include `expected_interest = 33972602 USD_micro`, `expected_total_due = 533972602 USD_micro`, and `expected_outstanding_notional = 4500000000 USD_micro`. Accrue rejects unless the floor interest equals `expected_interest`. Settle rejects unless the paid total equals `expected_total_due`. Both actions reject if remaining notional leaves `expected_outstanding_notional`.

Swap constants include `expected_output = 19743 AssetB_quantum`, `expected_reserve_a = 1010000 AssetA_quantum`, and `expected_reserve_b = 1980257 AssetB_quantum`. Swap rejects unless the floor output and the two reserves match those constants.

That is a fixture lock, not a general pricing engine. Changing input amounts, reserves, day count, or fee factor without changing those constants is specified to reject. This subset does not change source semantics to remove that lock.

## Independent arithmetic

### Loan first-period interest

Notional is `5000000000` USD_micro. Rate is `8/100`. Day count is `31/365`. Principal installment is `500000000` USD_micro.

```text
numerator   = 5000000000 * 8 * 31 = 1240000000000
denominator = 100 * 365           = 36500
floor       = 33972602
remainder   = 27000
27000 / 36500 = 54 / 73
```

Exact interest is `2480000000 / 73` micro-units. The source policy records the discarded remainder as `54/73` micro-USD for this sample only. Combined settlement is `500000000 + 33972602 = 533972602`. Remaining notional is `5000000000 - 500000000 = 4500000000`.

Borrower cash starts at `20000000000`. After settle it is `20000000000 - 533972602 = 19466027398`. Lender cash becomes `533972602`.

### Swap exact-input floor

Reserves start at `1000000` AssetA_quantum and `2000000` AssetB_quantum. Input is `10000` AssetA_quantum. Fee factor is `997/1000`.

```text
effective_input = 10000 * 997                    = 9970000
numerator       = 9970000 * 2000000              = 19940000000000
denominator     = 1000000 * 1000 + 9970000       = 1009970000
floor output    = 19743
19743 * 1009970000                               = 19939837710000
remainder       = 19940000000000 - 19939837710000 = 162290000
```

Exact output is `1994000000 / 100997`. Unpaid output remainder stays in `reserve_b`. After the swap, reserves are `1010000` and `1980257`. Trader balances are `90000` AssetA_quantum and `19743` AssetB_quantum.

The full `10000` input is credited to `reserve_a` and transferred trader to pool. The `997/1000` factor is used only in the floor formula. There is no `Fee` effect.

### Quantum

Both examples bind quantum `1` of the nominal unit to one ledger unit. For every Transfer and DueSettled in these traces, `ledgerAmount` equals the nominal amount. `value mod quantum.value = 0` holds. Reverse multiplication `ledgerAmount * quantum.value == value` holds in UInt128.

## Four rows

Each row is specified, not executed by this worker. Disposition is existing atomic-profile source. Compact mapping tests are observation only. Unsupported items stay open with a named owner.

Row `readFootprint` objects in `traces.json` use four keys: `sourcePreStateReads`, `sourceStagedReads`, `sourceConstantsArgumentsObservations`, and `sharedDependencyRef`. Source files are ground truth for those lists. Staged reads are in-action field reads after a prior `set` or at emit time. They are not dropped because they are not pre-state. `sharedDependencyRef` points at `sharedEvaluationDependencies`, which covers the whole source-bound program and manifest, canonical whole EvaluationInput and StateEnvelope hashes, typed ProgramRef, Authority, Genesis, ObservationSet, and State fields, obligation and status derivation, and finite registered `bounds.json` maxima. Listed source reads do not account for the entire evaluator work. Admission, digest, lifetime, claim-root, predecessor, horizon, settlement, resource, and result checks remain required. These maps are design expectations. Main semantics still require resource and result checks before any actual Complete.

Every positive row `observationMap` keeps specialized economic observations and references `$.sharedObservationMap`. That shared map owns field-by-field Complete, CompleteBody, after StateEnvelope, after StateBody, and ProofContext mappings, including `authorityConsumption.mode` / `principal` / `nonce` / `statementDigest`, `obligationDelta.created` / `settled`, all eight `ResourceCounts` members, and after schema / `stateHash` / body structural fields. Do not invent counts or hash values. Known instruction and effect counts stay. Uncomputed counts stay pending derivation with method, owner, and closure task.

### loan-accrue

Roles: actor and debtor `borrower`, creditor `lender`. Denomination `USD_micro`. Settlement binding name `USD_micro_asset`, asset text `USD_TEST_ASSET`. Rounding on interest is `floor(accrue, interest_calculated)` bound to FloorDiv node `a_0_s_5_e_0`. Principal installment has `rounding none`.

Read footprint, aligned to `traces.json` `positiveRows.loan-accrue.readFootprint` and `sharedEvaluationDependencies`:

- sourcePreStateReads: `state.cursor`, `state.notional`
- sourceStagedReads: `state.principal_due` after set, used by DueCreated PR emit; `state.interest_due` after set, used by DueCreated IP emit; `state.notional` after set, used by the `expected_outstanding_notional` guard
- sourceConstantsArgumentsObservations: `obs.now`, `arg.actor`, `const.borrower`, `const.lender`, rate, day-count, installment, expected-interest, and expected-outstanding-notional constants, horizon literal `2000000000`
- sharedDependencyRef: `shared-eval-deps-bounded-atomic-1`

Role and expected-value constants are source arguments to guards. Remaining, revision, genesis lifetime, authority, ObservationSet, and registered bounds stay in the shared dependency map, not in the listed source reads.

Write footprint, source order: `principal_due`, `interest_due`, `notional`, `cursor`. Nonfinancial write: `cursor`.

Effects, emit order. Exact DueCreated wire records omit `settlement` entirely. `settlement: null` is illegal on that closed variant. Conceptual not-applicable design metadata may still use null elsewhere.

1. `DueCreated` ordinal `0`, due_id `lam01:period1:PR`, debtor borrower, creditor lender, denomination `USD_micro`, amount `500000000` USD_micro. Settlement field absent.
2. `DueCreated` ordinal `1`, due_id `lam01:period1:IP`, debtor borrower, creditor lender, denomination `USD_micro`, amount `33972602` USD_micro. Settlement field absent.

After state: notional `4500000000`, principal_due `500000000`, interest_due `33972602`, principal_paid `0`, interest_paid `0`, borrower_cash `20000000000`, lender_cash `0`, cursor `1`, episode_closed `0`. Revision `1`, remaining `1`. Episode Open. Agreement Outstanding. Remaining notional `4500000000` USD_micro. Two Outstanding obligations in creation order, PR then IP.

Obligation delta: created holds both Outstanding snapshots. Settled is empty. Tombstones do not exist yet.

Lifetime is `2`. Horizon is exclusive UTC epoch-second `2000000000`. This action consumes one allowance. Residual contractual notional remains. Future periods are not implemented.

Invalid mutations: wrong-role source guard, settle-before-accrue on the genesis state, duplicate accrue after cursor moves, missing dues, and `loan-replayed-due` on after-accrue with one status changed to Settled. See traces.json. Do not use exhausted after-settle remaining `0` as the replayed-due fixture.

Source gap: one first LAM period, not ACTUS. Calendar, event, convention, signed-value, and quantity-price behavior remain MC07 work.

Closure owner MC01. Historical authored f173 dated SP01.8 as pending. Current local atomic acceptance hashes are bound separately. This subset remains pending-review. Residual servicing owner MC07, sprint SP07.

### loan-settle

Roles: actor and debtor `borrower`, creditor `lender`. Same denomination and settlement binding. All Amount writes use policy `settled_cash` with `rounding none`. Settlement reads committed pre-state, so FloorDiv provenance from accrue does not carry into this action.

Read footprint, aligned to `traces.json` `positiveRows.loan-settle.readFootprint`:

- sourcePreStateReads: `state.cursor`, `state.principal_due`, `state.interest_due`, `state.borrower_cash`, `state.lender_cash`, `state.principal_paid`, `state.interest_paid`, `state.notional`
- sourceStagedReads: `state.principal_due` and `state.interest_due` at DueSettled emit time before zeroing; `state.notional` after `episode_closed = 1` residual-notional guard
- sourceConstantsArgumentsObservations: `obs.now`, `arg.actor`, `arg.settlement_asset`, `arg.amount_due`, `const.borrower`, `const.lender`, `const.expected_total_due`, `const.expected_outstanding_notional`, text `USD_TEST_ASSET`, horizon literal `2000000000`
- sharedDependencyRef: `shared-eval-deps-bounded-atomic-1`

DueSettled also depends on retained obligations in StateBody. That record set is a shared evaluation dependency, not a `loan.mori` `state.*` name.

Write footprint, source order: `borrower_cash`, `lender_cash`, `principal_paid`, `interest_paid`, then after the three emits, `principal_due`, `interest_due`, `cursor`, `episode_closed`.

Effects, emit order. Emit captures values at its position, before due fields are zeroed:

1. `Transfer` ordinal `0`, asset `USD_TEST_ASSET`, from borrower, to lender, amount `533972602` USD_micro. Settlement: binding `USD_micro_asset`, quantum `1` USD_micro, ledgerAmount `533972602`.
2. `DueSettled` ordinal `1`, due_id `lam01:period1:PR`, amount `500000000` USD_micro, asset `USD_TEST_ASSET`, same parties and denomination. Settlement as above with ledgerAmount `500000000`.
3. `DueSettled` ordinal `2`, due_id `lam01:period1:IP`, amount `33972602` USD_micro, asset `USD_TEST_ASSET`. Settlement ledgerAmount `33972602`.

Conservation: the checked sum of the two DueSettled nominals equals the Transfer nominal for `(USD_TEST_ASSET, borrower, lender, USD_micro)`. Ledger sums match because quantum is `1`.

After state: notional still `4500000000`, principal_due `0`, interest_due `0`, principal_paid `500000000`, interest_paid `33972602`, borrower_cash `19466027398`, lender_cash `533972602`, cursor `2`, episode_closed `1`. Revision `2`, remaining `0`. Episode Closed. Agreement still Outstanding because remaining notional is greater than zero. Two Settled tombstones retained in creation order.

Obligation delta: created empty, settled holds both Settled snapshots. After.obligations keeps those Settled records. Identity `(instanceId, dueId)` is retained for the genesis lifetime. The records are not deleted.

Closing the episode must not erase remaining notional. The source guard after `episode_closed = 1` still requires `state.notional == 4500000000`.

Invalid mutations: settle before accrue (`GUARD_FAILED`, message `dues are not ready`), wrong due amount, wrong settlement asset, missing due, erased due, already-settled replay, identity-field mismatch, amount below outstanding, amount above outstanding. See traces.json.

Continuation: lifetime is exhausted after this step. Residual notional `4500000000` still exists. Future servicing, refinance, and later periods are unsupported.

### swap-exact-input

Roles: actor and recipient `trader`, pool identity `pool`, unused provider on this row. Units `AssetA_quantum` and `AssetB_quantum`. Settlement bindings `asset_a_binding` / `ASSET_A` and `asset_b_binding` / `ASSET_B`. Output rounding is `floor(swap, output_calculated)`. Input path has `rounding none`.

Read footprint, aligned to `traces.json` `positiveRows.swap-exact-input.readFootprint`:

- sourcePreStateReads: `state.epoch_closed`, `state.trader_a`, `state.reserve_a`, `state.reserve_b`, `state.trader_b`
- sourceStagedReads: `state.reserve_a` after input credit, expected-reserve-a guard; `state.reserve_b` after output debit, expected-reserve-b guard
- sourceConstantsArgumentsObservations: `obs.now`, remaining source guard `remaining > uint(1)`, `arg.actor`, `arg.recipient`, `arg.asset_in`, `arg.asset_out`, `arg.amount_in`, `arg.min_out`, `const.trader`, `const.pool`, fee and expected-output/reserve constants, text `ASSET_A` / `ASSET_B`, horizon literal `2000000000`
- sharedDependencyRef: `shared-eval-deps-bounded-atomic-1`

`min_out` is an ActionCall argument. OutcomeStatement does not sign `min_out`. `trader_a` and `trader_b` are source pre-state reads on this row. Source never reads trader balances again after writes. The shared whole-state hash dependency still commits them.

Write footprint, source order: `trader_a`, `reserve_a`, `reserve_b`, `trader_b`.

Effects:

1. `Transfer` ordinal `0`, asset `ASSET_A`, from trader, to pool, amount `10000` AssetA_quantum. Settlement binding `asset_a_binding`, quantum `1`, ledgerAmount `10000`.
2. `Transfer` ordinal `1`, asset `ASSET_B`, from pool, to trader, amount `19743` AssetB_quantum. Settlement binding `asset_b_binding`, quantum `1`, ledgerAmount `19743`.

No DueCreated, DueSettled, or Fee. Remaining notional is `NotApplicable`. Obligations stay empty.

After state: reserve_a `1010000`, reserve_b `1980257`, trader_a `90000`, trader_b `19743`, provider_a `0`, provider_b `0`, epoch_closed `0`. Revision `1`, remaining `7`. Episode Open. Agreement NoOutstanding.

Minimum-receive predicate: `min_out <= output_calculated`. Specified accepting values for this fixture: `0`, `1`, and `19743`. Specified rejecting value: `19744` (`GUARD_FAILED`, message `minimum output not met`). Over-delivery relative to `min_out` is allowed. Exact-output equality is not the predicate. `actionHash` binds the `min_out` argument even when writes and effects are identical, because `actionHash` hashes the ActionCall.

Reserve rule: `reserve swap for close`. The reserved action must keep a top-level guard whose condition is exactly `Gt(Remaining, Literal(UIntLiteral token "1"))` after parenthesis erasure. When remaining is `1`, swap is specified to reject and preserve the last allowance for `close`. The rule does not prove that close can run.

Invalid mutations: wrong-role source guard, wrong recipient, wrong assets, `min_out = 19744`, remaining `1` with revision `7` on lifetime `8`, principal-binding mismatch, present-false stale nonce or non-current state. See traces.json.

Source gap: fixed input vector only. Not Uniswap conformance, not concentrated liquidity, not iterative invariant AMM, not exact-output.

### swap-close

Roles: actor `provider`. Trader balances are not read or written. Close is provider-only.

Read footprint, aligned to `traces.json` `positiveRows.swap-close.readFootprint`:

- sourcePreStateReads: `state.epoch_closed`, `state.provider_a`, `state.provider_b`, `state.reserve_a`, `state.reserve_b`
- sourceStagedReads: `state.reserve_a` at Transfer emit time before zeroing; `state.reserve_b` at Transfer emit time before zeroing
- sourceConstantsArgumentsObservations: `obs.now`, `arg.actor`, `const.provider`, `const.pool`, text `ASSET_A` / `ASSET_B`, horizon literal `2000000000`
- sharedDependencyRef: `shared-eval-deps-bounded-atomic-1`

Close source does not read trader balances. Full `stateHash` still commits `trader_a` and `trader_b`.

Write footprint, source order: `provider_a`, `provider_b`, then after the two emits, `reserve_a`, `reserve_b`, `epoch_closed`. Emit uses `state.reserve_*` before those fields are zeroed.

Effects:

1. `Transfer` ordinal `0`, asset `ASSET_A`, from pool, to provider, amount `1010000` AssetA_quantum. Settlement `asset_a_binding`, ledgerAmount `1010000`.
2. `Transfer` ordinal `1`, asset `ASSET_B`, from pool, to provider, amount `1980257` AssetB_quantum. Settlement `asset_b_binding`, ledgerAmount `1980257`.

After state: reserves `0` / `0`, trader unchanged at `90000` / `19743`, provider_a `1010000`, provider_b `1980257`, epoch_closed `1`. Revision `2`, remaining `6`. Episode Closed. Agreement NoOutstanding. Remaining notional NotApplicable. Obligations empty. No remaining notional and no retained dues.

Lifetime is `8`. Two successful actions leave remaining `6`. Cancellation, races, private workflow, general fees, and further liquidity operations are unsupported.

## Authority and custody

Evaluation receives a closed `EvaluationInput`. Exact field names follow `runtime-types.ts` and `typed-schemas.md`. Signed statement fields are distinct from derived action and observation context.

Both ExactPlanStatement and OutcomeStatement share AuthorityCommon:

- `program` ProgramRef (`bounds`, `coreVersion` `moriarty-core/1`, `profile` `moriarty-bounded-atomic/1`, `programHash`, `schemaVersion` `moriarty-program-ref/1`, `sourceHash`)
- `domain` ExecutionDomain, `genesisHash`, `instanceId`
- `beforeStateHash` and `predecessors` exactly `[beforeStateHash]`
- `principal`, `nonce`, `validity` `{notBefore, notAfterExclusive}`
- `requiredClaims` and `requiredClaimRoot`

GenesisBody separately holds `observationBindings` and `principalBindings`, plus lifetime, horizon, initialState, domain, instanceId, bounds, profile, program, and `requiredClaimRoot`. `genesisHash` therefore indirectly commits observation/provider and principal bindings. The signed statement does not contain ObservationSet.

ExactPlanStatement additionally signs `action` ActionCall including `arguments`, ordered `exactWrites`, and ordered `exactEffects`. Extra, missing, reordered, or value-changed writes or effects reject `EXACT_PLAN_MISMATCH`. For ExactPlan controls, signed `statement.action` must equal the mutated ActionCall.

OutcomeStatement (IntentRefinement) additionally signs `allowedActions`, `permittedRecipients`, `permittedCalls`, principal-scoped `grossDebitCaps`, and `minimumNetCredits`. OutcomeStatement does not contain ActionCall or `min_out`. It does not introduce a nominal-debt cap for DueCreated. DueCreated and DueSettled do not enter debit or credit sums.

ObservationSet is authenticated separately through trusted `ExternalChecks.observationsAuthentic`. After evaluation, CompleteBody binds `actionHash` and `observationsHash` of the evaluated invocation. ProofContext binds observations indirectly through `traceHash`. ProofContext has no `observationsHash` member. Neither ExactPlanStatement nor OutcomeStatement directly signs the current ObservationSet.

Source-authorized constants remain fixture locks. Unresolved nominal-debt and pool/provider custody limitations remain.

Gross debit:

- Every Transfer or Fee with `from == principal-bound actor` needs a cap for `(actor, asset)`, including zero amount.
- Checked ledger sums must not exceed the cap.
- Refunds never reduce the gross sum.
- Fees count in the gross sum.

Net credit:

- For each goal, credits sum Transfer/Fee with `to == actor` and debits sum Transfer/Fee with `from == actor` for that asset.
- The goal holds exactly when `checked(credits) >= checked(debits + minimumLedgerAmount)`.
- Every debit is subtracted. A self-transfer contributes to both sums.

On these rows:

- loan-accrue emits only DueCreated. There is no principal Transfer or Fee, so debit caps are unused for this action. The dues still require the ContractProperty claims named by the interest and principal policies.
- loan-settle has principal debit `533972602` of `USD_TEST_ASSET` from borrower. A cap below that amount is specified to reject. The two DueSettled records do not add a second debit.
- swap-exact-input has principal debit `10000` of `ASSET_A` from trader to pool. The pool-to-trader output is not a principal debit. Recipients `pool` and `trader` must both be permitted.
- swap-close has principal `provider` with no outgoing Transfer from provider. Both transfers leave the pool. Recipients must include `provider`.

Pool outgoing effects are part of all-effects evidence. Principal debit caps do not prove that the pool is authorized to pay the trader, or that the provider is authorized to receive the reserves. That external custody and consent relation is an unresolved MC02, MC04, and MC05 obligation before public spending.

Source guards such as `arg.actor == const.borrower` constrain a program role. The wrapper must still bind authenticated principal to that actor. Text equality is not authentication.

Trusted `ExternalChecks` booleans (genesisValid, signatureValid, nonceFresh, observationsAuthentic, predecessorSetValid, stateCurrentAndUnconsumed) are pre-execution assumptions. A local simulation that sets them true does not prove signatures, currentness, or oracle truth.

## Required claims

Manifest `requiredClaims` are generated from the profile. They are proof obligations, not evidence that a backend exists.

Loan, sorted by `(kind ASCII, claimId UTF-8)`:

1. ContractProperty `bounded_profile_safety_v1`
2. ContractProperty `loan_first_period_interest_floor_v1`
3. ContractProperty `loan_first_period_principal_v1`
4. ContractProperty `loan_first_period_settlement_exact_v1`
5. IntentRefinement `atomic_intent_refinement_v1`
6. PredecessorHistory `bounded_history_compliance_v1`
7. TransitionValidity `bounded_atomic_transition_v1`

Swap:

1. ContractProperty `bounded_profile_safety_v1`
2. ContractProperty `constant_product_close_a_exact_v1`
3. ContractProperty `constant_product_close_b_exact_v1`
4. ContractProperty `constant_product_exact_input_floor_v1`
5. ContractProperty `constant_product_input_exact_v1`
6. IntentRefinement `atomic_intent_refinement_v1`
7. PredecessorHistory `bounded_history_compliance_v1`
8. TransitionValidity `bounded_atomic_transition_v1`

The four mandatory judgments remain named and not discharged: ContractProperty, IntentRefinement, TransitionValidity, and PredecessorHistory. Policy `proof` strings are documentary identifiers bound into the program hash. They are not parsed as predicates and are not a substitute for those claims.

Missing proof backend cannot create accepted Complete. A malformed third `evaluate` argument `{requiredClaimsValid:true, historyProofValid:true}` used as a fake backend also cannot. Those booleans are not EvaluationInput fields. Specified diagnostic is `PROOF_INVALID`. Simulation output is labeled simulation and is outside accepted Result.

## Obligation identity

Identity is `(instanceId, dueId)` for the genesis lifetime. Capacity includes Settled tombstones. This profile neither partially settles nor recreates an ID.

DueCreated rejects zero amount, denomination/unit disagreement, duplicate identity including a tombstone, and capacity failure. DueSettled requires exactly one retained record with matching debtor, creditor, denomination, and unit. Amount below outstanding rejects `OBLIGATION_PARTIAL_UNSUPPORTED`. Amount above rejects `OBLIGATION_EXCESS`. Unknown ID rejects `OBLIGATION_UNKNOWN`. Already Settled rejects `OBLIGATION_ALREADY_SETTLED`.

Obligations cannot vanish because:

1. DueSettled changes status to Settled without deleting the record.
2. Episode Closed does not imply Agreement NoOutstanding.
3. Settled tombstones occupy capacity and block duplicate create.
4. A later action cannot drop a due from the supplied state and still settle it.
5. Delta snapshots are immutable event-time copies. A later effect does not rewrite an earlier created snapshot.

Direct missing, erased, or replayed due mutations are specified to reject. They are not a path to silent debt erasure.

## Negatives

All negatives specify zero accepted effects. `unchangedInput` means the supplied mutated fixture immediately before versus after evaluation. Diagnostics are the closed `DiagnosticCode` registry. Citations name existing tests or source. Variants that use a host program other than current `loan.mori` / `swap.mori`, or that this worker did not execute, are labeled as such.

Construction follows `traces.json` `negativeFixtureConvention`. That object is not an extra `negativeTraces` id. Each control starts from a fresh independent valid fixture. Preserve prior admission and binding checks unless the control deliberately targets them. When targeting obligation or lifetime predicates, reseal the mutated StateBody with `stateHash = hash(STATE, body)`, then rebind `authority.statement.beforeStateHash` and `predecessors` to that mutated hash. Simulator controls assume `signatureValid`. That assumption is simulation-only. A valid real signature does not survive changing signed bytes. Malicious mutated state is not historically reachable. Per-row construction references and exceptions live on the matching `negativeTraces` records.

Wrong-role source guard: `action.actor` is the wrong role, authority `principal` is that same wrong actor, trusted `authenticatedPrincipal` is the same, and genesis principal binding matches that actor. The source guard then fails (`GUARD_FAILED` with `borrower authority required`, `trader authority required`, or `provider authority required`). Distinguish actor/principal mismatch: `arg.actor` matches the role while `authenticatedPrincipal` does not, which is `PRINCIPAL_BINDING`. For ExactPlan, signed `statement.action` must equal the mutated ActionCall. Wrong recipient or assets stay source-guard `GUARD_FAILED` (`recipient must be trader`, `asset_in mismatch`, `asset_out mismatch`, `settlement asset mismatch`). Citation: `semantics.test.mjs` principal/provider test and source guards. Compact mapping observation: `lowering.test.mjs` rejects a swapped actor id.

Swap `min_out = 19744`: `GUARD_FAILED`, message `minimum output not met`. Citation: `semantics.test.mjs` typed-input test and `lowering.test.mjs` `19744n` throw. Accepting adjacent values `0`, `1`, and `19743` are specified.

Settle before accrue: `GUARD_FAILED`, message `dues are not ready`, stage `11`. Citation: B2 control `loan settlement before accrual rejects at its exact source guard`.

Wrong due amount: `GUARD_FAILED`, message `settlement amount mismatch`, when `amount_due` differs from `principal_due + interest_due`.

No reserve for swap (`swap-no-reserve-remaining-one`): synthetic state with revision `7` and remaining `1`, consistent with lifetime `8`. Reseal and rebind that synthetic state. The source guard `remaining > uint(1)` rejects with `final allowance is reserved for closure`. Citation: `semantics.test.mjs` remap remaining `1` on swap. A program that omits a reserve declaration incurs no reserve check. The loan has none. Removing the swap reserve would be a source change outside this subset.

`loan-replayed-due` mutates after-accrue state by changing one obligation status to Settled, then reseals and rebinds, then settle. Expected `OBLIGATION_ALREADY_SETTLED`. Real after-settle state with remaining `0` fails `LIFETIME_EXHAUSTED` first and is a separate non-equivalent case. Do not use exhausted after-settle as this fixture.

Stale or replayed nonce: present `nonceFresh: false` gives `NONCE_STALE`. Stale or replayed state: present `stateCurrentAndUnconsumed: false` gives `STATE_NOT_CURRENT`. Omitting a required `checks` field is `INPUT_SCHEMA`, not those named codes. Citation: `semantics.test.mjs` external-precheck loop and backend-authentication test. Replay of a consumed before-state is the same currentness predicate when the false boolean is present.

ExactPlan extra or reordered effects: `EXACT_PLAN_MISMATCH`. These controls use a new simulated statement with a signature-valid assumption. They never assert that an unchanged real signature still verifies. Citation: B2 control `ExactPlan rejects extra and reordered effects next to its exact accepting plan`. Unsigned simulation may propose writes and effects. That proposal is not an accepted Result.

Gross debit cap below actual protected debit, including Fee, with refunds not netted: specified by `semantics.md` IntentRefinement rules. Adjacent test uses a synthetic Movements host, not `loan.mori`. Citation: `outcome gross caps include fees and never net refunds`. Diagnostic `INTENT_DEBIT_CAP`. Uncapped principal outgoing Transfer or Fee is `INTENT_DEBIT_UNCAPPED`. This variant is specified from the profile and that test. It is not an executed mutation of the loan or swap sources.

Minimum net credit subtracts every debit: same synthetic Movements host. A borrower who pays `100`, receives refund `99`, and pays Fee `2` fails a `minimumLedgerAmount` of `0` because credits `99` are less than debits `102`. Diagnostic `INTENT_NET_GOAL`. That net-goal fixture must carry a sufficient gross cap. Use `maxUInt128` as the fresh cited test cap, not the preceding `101` debit-cap fixture. Otherwise `INTENT_DEBIT_CAP` can fire first.

Missing proof backend: `PROOF_INVALID`. Citation: `missing backend and client boolean claims cannot produce accepted Complete`.

Client booleans: `semantics.test.mjs` passes a malformed third `evaluate` argument `{requiredClaimsValid:true, historyProofValid:true}` as a fake backend. Those booleans are not extra fields on EvaluationInput. Specified diagnostic remains `PROOF_INVALID`.

Direct due mutation starts from after-accrue, reseals, and rebinds unless noted. Missing due (`loan-missing-due`) pops a retained record then settle, diagnostic `OBLIGATION_UNKNOWN`. Erased due (`loan-erased-due`) drops a retained record from supplied state then settle, same diagnostic. Replayed due (`loan-replayed-due` above) marks one status Settled then settle, diagnostic `OBLIGATION_ALREADY_SETTLED`. Change debtor is `OBLIGATION_MISMATCH`. Raise retained outstanding above the emit amount is `OBLIGATION_PARTIAL_UNSUPPORTED`. Lower it is `OBLIGATION_EXCESS`. Recreate after accrue (`loan-duplicate-due`) sets `state.body.values[name="cursor"].value` to UInt128 `0`, `state.body.values[name="notional"].value` to Amount `5000000000` USD_micro, and `state.body.remainingNotional.amount` to the same Amount `5000000000` USD_micro. It retains revision `1`, remaining `1`, and existing dues from after-accrue. It then reseals `stateHash` and rebinds `authority.statement.beforeStateHash` and predecessors with a simulated valid signature assumption. Without that derived remainingNotional update, `STATUS_MISMATCH` occurs first, not `OBLIGATION_DUPLICATE`. Citation: `semantics.test.mjs` line 88. Conservation failure on an altered Transfer recipient is `OBLIGATION_CONSERVATION` and uses altered source, labeled unexecuted against current `loan.mori`.

`PROGRAM_BOUNDS` is stage 7. The next source size above admitted instruction, local, node, depth, or effect maxima rejects before execution. Citation: `runtime counts attain the admitted instruction/local/node bounds; the next source size rejects before execution`.

`RESOURCE_BOUNDS` is stage 12. It is a defensive runtime count check after execution. In the fixed admitted straight-line loan and swap profile, source already passed stage 7, so this code is specified as unreachable on those two programs. Do not treat a `PROGRAM_BOUNDS` source rejection as a `RESOURCE_BOUNDS` runtime result.

## Envelope fields versus financial values

Concrete financial values are the amounts, units, roles, due identifiers, statuses, revision, remaining, and ordered effect operands listed in traces.json.

Envelope identity fields are not invented as hex. Parent recomputes them from exact inputs:

- `sourceHash = SHA256(original source UTF-8 bytes)` and equals the pinned file digest for each example
- `boundsHash = SHA256(UTF8("MORIARTY-BOUNDS-bounded-atomic/1") || 00 || exact bounds.json bytes)`
- `programHash = SHA256(UTF8("MORIARTY-PROGRAM-bounded-atomic/1") || 00 || canonical(SemanticManifest))`
- `claimRoot = SHA256(UTF8("MORIARTY-CLAIMS-bounded-atomic/1") || 00 || canonical(requiredClaims))`
- `genesisHash`, `stateHash`, `actionHash`, `observationsHash`, `traceHash`, `statementDigest`, `authorityDigest`, and `proofContextHash` follow the typed-schemas registry

`traces.json` `sharedObservationMap` maps every Complete wrapper, CompleteBody, after StateEnvelope, after StateBody, and ProofContext field, including `authorityConsumption.mode` / `principal` / `nonce` / `statementDigest`, `obligationDelta.created` / `settled`, all eight `ResourceCounts` members, and after schema / `stateHash` / body structural fields. Each positive row references that shared map and retains specialized economic observations. Hash fields may be symbolic. Exact domain and preimage references are required. Do not invent counts or hash values.

Static trace derivations are specified expectations pending parent verification. They are not executed results. Main semantics still require resource and result checks before any actual Complete.

## Compact mapping observation

`lowering.test.mjs` records that a generic Compact mapper compiled loan and swap functions and compared decoded values, remaining, revision, and effect operands (settlement stripped) to a simulator candidate for the four transitions. That is observation evidence from an existing test file. This worker did not compile Compact, did not run that test, and does not treat a restricted kernel snapshot as ledger acceptance or PCD.

## Lifetime, horizon, and continuation

Loan lifetime `2`, swap lifetime `8`. Horizon `2000000000` exclusive on both. `obs.now` is explicit and externally authenticated. The evaluator does not read a host clock. `now < horizon` is mandatory even if a source guard is absent.

Every Complete decrements remaining once and increments revision once. `revision + remaining == genesis.lifetime` is checked. Remaining `0` rejects `LIFETIME_EXHAUSTED` before execution.

Loan residual notional persists beyond the closed two-step episode. Continuation ownership for later servicing is MC07. This atomic profile has no Pending, split, join, or continuation export. A request that needs residual progress rejects `UNSUPPORTED_PENDING`.

Swap close is provider-only. Trader cannot close. Last allowance is reserved for close. Remaining after close is `6` of lifetime `8`. Unused allowance is not a promise of later liquidity operations.

## What this subset does not close

Full RP01 remains specified-only and incomplete. Missing from this record: 277 ACTUS fixtures, 32 taxonomy dispositions, 72 DeFi rows, three held-outs, eight intent worked cases other than this minimum-receive sample, eight DeFi regression classes, five composition operators, and the theorem ledger.

Signing successor schema is SP01.3. Native-statement subset RP01-MC03 is SP01.7. Historical authored `f173` dated SP01.8 atomic implementation acceptance as pending. Current independent local atomic acceptance is code `c30aaf5c0d091b1efea7e042d0fcbd539ca6cc51` and evidence `b296ce95afae722177b5bba482d1d195fa334d1e`, bound separately. SP01.8 is no longer pending. This record remains pending-review and does not self-approve.

Public Preview loan and swap transactions, finalized receipts, full comparisons, and rejection controls remain SP05 work after separately reviewed fixtures, custody, contracts, and campaign. Assignment `raw/assignments/moriarty-midnight-milestone-testing-2026-09-07.md`. Source, spec, and local checks do not discharge them. Mandatory-proof SP09 and later network gates stay unchanged. Mandatory claims are MC05. Corpus implementation is MC07.

Do not read pending-review on RP01-MC02 as complete RP01, and do not read this design as a successor semantic decision.
