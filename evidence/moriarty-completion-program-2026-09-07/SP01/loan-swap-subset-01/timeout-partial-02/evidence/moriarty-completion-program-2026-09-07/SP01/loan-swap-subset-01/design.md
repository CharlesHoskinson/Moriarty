# RP01-MC02 loan and swap subset design

Status: specified design for existing `moriarty-bounded-atomic/1` behavior. Pending parent verification and independent GPT-6 review. Not a successor semantic freeze.

This note records the admitted loan first-period accrue-then-settle path and the swap exact-input minimum-receive-then-close path. The two `.mori` sources are synthetic bookkeeping examples. They are also the initial restricted Compact mapping examples. They are not live custody, ACTUS-wide conformity, general AMM behavior, exact-output semantics, complete PCD, or ledger acceptance.

The worker who wrote this file did not run the evaluator, Compact, tests, or any command. Numeric financial values come from independent integer arithmetic and from constants already embedded in the sources. Envelope hashes stay symbolic. Tests in `semantics.test.mjs` and `lowering.test.mjs` are observation evidence, not definitions.

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

### loan-accrue

Roles: actor and debtor `borrower`, creditor `lender`. Denomination `USD_micro`. Settlement binding name `USD_micro_asset`, asset text `USD_TEST_ASSET`. Rounding on interest is `floor(accrue, interest_calculated)` bound to FloorDiv node `a_0_s_5_e_0`. Principal installment has `rounding none`.

Read footprint: `obs.now`, `arg.actor`, `state.cursor`, `state.notional`, and the rate, day-count, installment, and expected-interest constants. Stage 10 also reads remaining, revision, horizon, and genesis lifetime.

Write footprint, source order: `principal_due`, `interest_due`, `notional`, `cursor`. Nonfinancial write: `cursor`.

Effects, emit order:

1. `DueCreated` ordinal `0`, due_id `lam01:period1:PR`, debtor borrower, creditor lender, denomination `USD_micro`, amount `500000000` USD_micro. No settlement resolution.
2. `DueCreated` ordinal `1`, due_id `lam01:period1:IP`, debtor borrower, creditor lender, denomination `USD_micro`, amount `33972602` USD_micro. No settlement resolution.

After state: notional `4500000000`, principal_due `500000000`, interest_due `33972602`, principal_paid `0`, interest_paid `0`, borrower_cash `20000000000`, lender_cash `0`, cursor `1`, episode_closed `0`. Revision `1`, remaining `1`. Episode Open. Agreement Outstanding. Remaining notional `4500000000` USD_micro. Two Outstanding obligations in creation order, PR then IP.

Obligation delta: created holds both Outstanding snapshots. Settled is empty. Tombstones do not exist yet.

Lifetime is `2`. Horizon is exclusive UTC epoch-second `2000000000`. This action consumes one allowance. Residual contractual notional remains. Future periods are not implemented.

Invalid mutations: wrong actor, settle-before-accrue on the genesis state, duplicate accrue after cursor moves, replayed or missing dues on a later settle. See traces.json.

Source gap: one first LAM period, not ACTUS. Calendar, event, convention, signed-value, and quantity-price behavior remain MC07 work.

Closure owner MC01, task SP01.8 for atomic acceptance of this specified trace. Residual servicing owner MC07, sprint SP07.

### loan-settle

Roles: actor and debtor `borrower`, creditor `lender`. Same denomination and settlement binding. All Amount writes use policy `settled_cash` with `rounding none`. Settlement reads committed pre-state, so FloorDiv provenance from accrue does not carry into this action.

Read footprint: `obs.now`, `arg.actor`, `arg.settlement_asset`, `arg.amount_due`, `state.cursor`, `state.principal_due`, `state.interest_due`, `state.borrower_cash`, `state.lender_cash`, `state.principal_paid`, `state.interest_paid`, `state.notional`.

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

Read footprint: `obs.now`, `remaining`, `arg.actor`, `arg.recipient`, `arg.asset_in`, `arg.asset_out`, `arg.amount_in`, `arg.min_out`, `state.epoch_closed`, `state.trader_a`, `state.reserve_a`, `state.reserve_b`, and the fee and expected-output/reserve constants.

Write footprint, source order: `trader_a`, `reserve_a`, `reserve_b`, `trader_b`.

Effects:

1. `Transfer` ordinal `0`, asset `ASSET_A`, from trader, to pool, amount `10000` AssetA_quantum. Settlement binding `asset_a_binding`, quantum `1`, ledgerAmount `10000`.
2. `Transfer` ordinal `1`, asset `ASSET_B`, from pool, to trader, amount `19743` AssetB_quantum. Settlement binding `asset_b_binding`, quantum `1`, ledgerAmount `19743`.

No DueCreated, DueSettled, or Fee. Remaining notional is `NotApplicable`. Obligations stay empty.

After state: reserve_a `1010000`, reserve_b `1980257`, trader_a `90000`, trader_b `19743`, provider_a `0`, provider_b `0`, epoch_closed `0`. Revision `1`, remaining `7`. Episode Open. Agreement NoOutstanding.

Minimum-receive predicate: `min_out <= output_calculated`. Specified accepting values for this fixture: `0`, `1`, and `19743`. Specified rejecting value: `19744` (`GUARD_FAILED`, message `minimum output not met`). Over-delivery relative to `min_out` is allowed. Exact-output equality is not the predicate. `actionHash` binds the `min_out` argument even when writes and effects are identical, because `actionHash` hashes the ActionCall.

Reserve rule: `reserve swap for close`. The reserved action must keep a top-level guard whose condition is exactly `Gt(Remaining, Literal(UIntLiteral token "1"))` after parenthesis erasure. When remaining is `1`, swap is specified to reject and preserve the last allowance for `close`. The rule does not prove that close can run.

Invalid mutations: wrong actor, wrong recipient, wrong assets, `min_out = 19744`, remaining `1`, unauthenticated principal, stale nonce, non-current state. See traces.json.

Source gap: fixed input vector only. Not Uniswap conformance, not concentrated liquidity, not iterative invariant AMM, not exact-output.

### swap-close

Roles: actor `provider`. Trader balances are not read or written. Close is provider-only.

Read footprint: `obs.now`, `arg.actor`, `state.epoch_closed`, `state.provider_a`, `state.provider_b`, `state.reserve_a`, `state.reserve_b`.

Write footprint, source order: `provider_a`, `provider_b`, then after the two emits, `reserve_a`, `reserve_b`, `epoch_closed`. Emit uses `state.reserve_*` before those fields are zeroed.

Effects:

1. `Transfer` ordinal `0`, asset `ASSET_A`, from pool, to provider, amount `1010000` AssetA_quantum. Settlement `asset_a_binding`, ledgerAmount `1010000`.
2. `Transfer` ordinal `1`, asset `ASSET_B`, from pool, to provider, amount `1980257` AssetB_quantum. Settlement `asset_b_binding`, ledgerAmount `1980257`.

After state: reserves `0` / `0`, trader unchanged at `90000` / `19743`, provider_a `1010000`, provider_b `1980257`, epoch_closed `1`. Revision `2`, remaining `6`. Episode Closed. Agreement NoOutstanding. Remaining notional NotApplicable. Obligations empty. No remaining notional and no retained dues.

Lifetime is `8`. Two successful actions leave remaining `6`. Cancellation, races, private workflow, general fees, and further liquidity operations are unsupported.

## Authority and custody

Evaluation receives a closed `EvaluationInput`. The following bindings are mandatory before execution. Exact field names follow `typed-schemas.md`.

ExactPlan and IntentRefinement both bind:

- `program` ProgramRef (profile `moriarty-bounded-atomic/1`, core `moriarty-core/1`, bounds registry, `programHash`, `sourceHash`)
- genesis, including domain, instanceId, lifetime, horizon, initialState, principalBindings, observationBindings, and `requiredClaimRoot`
- `beforeStateHash` and predecessors exactly `[beforeStateHash]`
- observations, with `now` mandatory and provider equal to the genesis observation binding
- action name and arguments
- nonce, validity interval, and principal
- `requiredClaims` and `requiredClaimRoot`

ExactPlan additionally signs the exact ActionCall, the ordered `{field,value}` write projection, and every ordered complete enriched effect. Extra, missing, reordered, or value-changed writes or effects reject `EXACT_PLAN_MISMATCH`.

IntentRefinement additionally signs allowedActions, permittedRecipients, empty permittedCalls, principal-scoped grossDebitCaps, and minimumNetCredits. It does not introduce a nominal-debt cap for DueCreated. DueCreated and DueSettled do not enter debit or credit sums.

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

Missing proof backend, or client-supplied `requiredClaimsValid` / `historyProofValid` booleans, cannot create accepted Complete. Specified diagnostic is `PROOF_INVALID`. Simulation output is labeled simulation and is outside accepted Result.

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

All negatives specify unchanged input and zero accepted effects. Diagnostics are the closed `DiagnosticCode` registry. Citations name existing tests or source. Variants that use a host program other than current `loan.mori` / `swap.mori`, or that this worker did not execute, are labeled as such.

Wrong actor, recipient, or assets: source guards on `loan.mori` and `swap.mori`. Expected `GUARD_FAILED` with the source message (`borrower authority required`, `trader authority required`, `provider authority required`, `recipient must be trader`, `asset_in mismatch`, `asset_out mismatch`, `settlement asset mismatch`). A mismatched authenticated principal is `PRINCIPAL_BINDING` even if `arg.actor` matches the role. Citation: `semantics.test.mjs` principal/provider test and source guards. Compact mapping observation: `lowering.test.mjs` rejects a swapped actor id.

Swap `min_out = 19744`: `GUARD_FAILED`, message `minimum output not met`. Citation: `semantics.test.mjs` typed-input test and `lowering.test.mjs` `19744n` throw. Accepting adjacent values `0`, `1`, and `19743` are specified.

Settle before accrue: `GUARD_FAILED`, message `dues are not ready`, stage `11`. Citation: B2 control `loan settlement before accrual rejects at its exact source guard`. Input must remain unchanged.

Wrong due amount: `GUARD_FAILED`, message `settlement amount mismatch`, when `amount_due` differs from `principal_due + interest_due`.

No reserve for swap: when remaining is `1`, the source guard `remaining > uint(1)` rejects with `final allowance is reserved for closure`. Citation: `semantics.test.mjs` remap remaining `1` on swap. A program that omits a reserve declaration incurs no reserve check. The loan has none. Removing the swap reserve would be a source change outside this subset.

Stale or replayed nonce: `NONCE_STALE` when trusted `nonceFresh` is false or unavailable. Stale or replayed state: `STATE_NOT_CURRENT` when `stateCurrentAndUnconsumed` is false or unavailable. Citation: `semantics.test.mjs` external-precheck loop and backend-authentication test. Replay of a consumed before-state is the same currentness predicate.

ExactPlan extra or reordered effects: `EXACT_PLAN_MISMATCH`. Citation: B2 control `ExactPlan rejects extra and reordered effects next to its exact accepting plan`. Unsigned simulation may propose writes and effects. That proposal is not an accepted Result.

Gross debit cap below actual protected debit, including Fee, with refunds not netted: specified by `semantics.md` IntentRefinement rules. Adjacent test uses a synthetic Movements host, not `loan.mori`. Citation: `outcome gross caps include fees and never net refunds`. Diagnostic `INTENT_DEBIT_CAP`. Uncapped principal outgoing Transfer or Fee is `INTENT_DEBIT_UNCAPPED`. This variant is specified from the profile and that test. It is not an executed mutation of the loan or swap sources.

Minimum net credit subtracts every debit: same synthetic Movements host. A borrower who pays `100`, receives refund `99`, and pays Fee `2` fails a `minimumLedgerAmount` of `0` because credits `99` are less than debits `102`. Diagnostic `INTENT_NET_GOAL`.

Missing proof backend or client booleans: `PROOF_INVALID`. Citation: `missing backend and client boolean claims cannot produce accepted Complete`.

Direct due mutation: pop due `OBLIGATION_UNKNOWN`, mark Settled then settle `OBLIGATION_ALREADY_SETTLED`, change debtor `OBLIGATION_MISMATCH`, raise retained outstanding above the emit amount `OBLIGATION_PARTIAL_UNSUPPORTED`, lower it `OBLIGATION_EXCESS`, recreate after accrue `OBLIGATION_DUPLICATE`. Conservation failure on an altered Transfer recipient is `OBLIGATION_CONSERVATION` and uses altered source, labeled unexecuted against current `loan.mori`.

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

traces.json keeps an observation map for every Complete and State field, including those digests, rather than dropping them.

Static trace derivations are specified expectations pending parent verification. They are not executed results.

## Compact mapping observation

`lowering.test.mjs` records that a generic Compact mapper compiled loan and swap functions and compared decoded values, remaining, revision, and effect operands (settlement stripped) to a simulator candidate for the four transitions. That is observation evidence from an existing test file. This worker did not compile Compact, did not run that test, and does not treat a restricted kernel snapshot as ledger acceptance or PCD.

## Lifetime, horizon, and continuation

Loan lifetime `2`, swap lifetime `8`. Horizon `2000000000` exclusive on both. `obs.now` is explicit and externally authenticated. The evaluator does not read a host clock. `now < horizon` is mandatory even if a source guard is absent.

Every Complete decrements remaining once and increments revision once. `revision + remaining == genesis.lifetime` is checked. Remaining `0` rejects `LIFETIME_EXHAUSTED` before execution.

Loan residual notional persists beyond the closed two-step episode. Continuation ownership for later servicing is MC07. This atomic profile has no Pending, split, join, or continuation export. A request that needs residual progress rejects `UNSUPPORTED_PENDING`.

Swap close is provider-only. Trader cannot close. Last allowance is reserved for close. Remaining after close is `6` of lifetime `8`. Unused allowance is not a promise of later liquidity operations.

## What this subset does not close

Full RP01 remains specified-only and incomplete. Missing from this record: 277 ACTUS fixtures, 32 taxonomy dispositions, 72 DeFi rows, three held-outs, eight intent worked cases other than this minimum-receive sample, eight DeFi regression classes, five composition operators, and the theorem ledger.

Signing successor schema is SP01.3. Native-statement subset RP01-MC03 is SP01.7. Atomic implementation acceptance is SP01.8. Public integration is MC02 after this subset is reviewed and campaign-admitted. Mandatory claims are MC05. Corpus implementation is MC07.

Do not read pending-review on RP01-MC02 as complete RP01, and do not read this design as a successor semantic decision.
