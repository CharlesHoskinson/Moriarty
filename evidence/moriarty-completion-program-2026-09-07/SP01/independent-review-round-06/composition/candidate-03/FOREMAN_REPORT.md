# FOREMAN_REPORT

Worker: Grok 4.6 high author
Worktree: this SP01 composition correction worktree
Task: targeted composition finite-model correction against GPT-6 findings C01-C11
Status: specified-only. Independent GPT-6 Astra result review is still required.
Not a successor semantic freeze, Moriarty evaluator, K implementation, proof, Preview settlement, or sprint acceptance.

## Owned files

- `experiments/moriarty-language/spec/successor/financial-fragments/composition.json`
- `experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py`
- `experiments/moriarty-language/spec/successor/financial-fragments/composition-model.test.py`
- `FOREMAN_REPORT.md`
- `FOREMAN_REPORT.json`

Candidate-02 and the cancelled author receipt were not edited. Generator does not read the mutable output path.

## Runnable interface

Generator (explicit seed, old fixture, old pins; no implicit previous output):

```
python3 experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py \
  --seed FILE --old-fixture FILE --old-pins FILE --output FILE
```

Independent tests (explicit candidate and old fixture; optional pins):

```
python3 experiments/moriarty-language/spec/successor/financial-fragments/composition-model.test.py \
  --candidate FILE --old-fixture FILE [--old-pins FILE]
```

`--old-pins` on the test CLI is optional. The generator requires all three inputs. Tests do not import the generator and do not call it to derive expected values.

## Commands actually run and exits

1. Independent tests against immutable candidate-02 composition.json.
   Exit: 1
   Summary: total=137 fail=87 pass=50
   Candidate sha256: `3ed9403616dc6cb6eb843ef94812481f168c525e2de375c80b5eac253827db4b`
   This RED result is retained. Failures covered fee double-count, missing cumulative 120>101 discriminator, incomplete schemas/fields, missing apply equations, missing intervening NAM accrual, footprint bound 32 vs 40/33, contradictory duty set equation, async two-enabled race, untyped negatives, and unpinned old Simulation retention.

2. Generate corrected artifact from seed + old fixture + old pins.
   Exit: 0
   Output sha256: `c5801af2a4fd14e5968c4ecc00963e515ba19fdb38c052cf0fb3f94de42a054e`
   Bytes: 575771

3. Independent tests against generated `composition.json` with the same old fixture and pins.
   Exit: 0
   Summary: total=150 fail=0 pass=150

4. Second clean generation to `/tmp/composition-gen1.json`.
   Exit: 0
   sha256: `c5801af2a4fd14e5968c4ecc00963e515ba19fdb38c052cf0fb3f94de42a054e`

5. Third clean generation to `/tmp/composition-gen2.json`.
   Exit: 0
   sha256: `c5801af2a4fd14e5968c4ecc00963e515ba19fdb38c052cf0fb3f94de42a054e`

Two clean generations are byte-identical to the retained `composition.json`. No network, build, proof, install, git, or subagent.

## Digests

- candidate-02 seed composition.json: `3ed9403616dc6cb6eb843ef94812481f168c525e2de375c80b5eac253827db4b`
- retained-atomic-fixture.json: `2c44dcd0b364afae95213568aa74dd0e75dbc4264b4164772ba0593efbc1a560`
- old-domain-pins.json: `c1025a13d7ccc8c0dfed808969165ca968c4b5af14f167eb80fc028fb2902db0`
- gpt6-result-review.json: `5c15f3b46b65624e62cfc17727ebacfa6a6143fc9a2bc0c1c73ded2dc4191913`
- generated composition.json: `c5801af2a4fd14e5968c4ecc00963e515ba19fdb38c052cf0fb3f94de42a054e`
- composition-model.py: `8c0e442405496d9d7dcf2810d939b1c4c8f32adf18ae0d7546f70757aa2d0f9e`
- composition-model.test.py: `7f2490f38cd6e0aeb12f955da3501eeb2a728b9b54693fd740933a2feaa1cfe6`
- NAM19 source pin retained: `bfc39c7a344b1243ce15accb9800837b3d87a050ffa435395595e4bf559e4a9b`

Generated artifact contains no `/home/` or `/Users/` absolute private paths.

## Coverage retained

Six rows, six traces, ten mutations, eight proposed theorems keep their IDs. Source sha256 pins are unchanged. axioms and mechanizedEvidence stay empty. Theorem status stays proposed. Tests are finite design-validation. No mechanized proof was supplied.

## Finding dispositions (specified-only)

### C01

Internal transcript outputs and external produced heads are named on every concretePlan and expected record. Original producedHistories lists remain for replay.

### C02

Closed sum/product schemas cover CompleteState including workPartition/frame, AuthorityRecord variants, positions/claims/requests/messages/orders, ObservationRecord, ContractRecord, AllowanceRecord, and primitive fields. SourceDecimal has grammar, maxLength, precision, and exact Decimal/Fraction arithmetic, separate from runtime Nat. Positive constructors carry debtor/creditor/unit/scale/observationRef or explicit lock-entitlement variants. Documentary metadata projection is explicit.

### C03

Alice USDC cap 101 is retained. Lender/PoolQ/joint counter changes are derived from transfers and envelope consumption. Membership uses remaining/cumulativeGross plus principal/asset/payee scope. Discriminator two Transfer 60 under 101 rejects at cumulative 120. System constructors are system-authorized.

### C04

Per-constructor preconditions and equations are recorded. AccrueFee is an annotation of an existing Transfer (shared `T_shared_fee`, atomic `T_atomic_fee`, async `T_async_fee`) and is not a second cash movement. Recorded post cash stays Alice 29/0/39 and Treasury 1. NAM prefix is IED, RR, IPCI, RR with AccrueNominal IED->RR `7200/73` and intervening RR->IPCI `5000 * (0.010567901234567900+0.10) * 91/365`. H3 uses that full prefix. Genesis matching Transfer requires payer and payee.

### C05

Canonical projection is total with parent/child paths, including positions and allocations. Toy footprint maximum is 64 (design bound, not registered runtime). Recomputed entry counts: NAM 32, sequential 14, parallel 28, shared 48, atomic 39, async 32. UnrelatedRecord remains outside writes.

### C06

Typed input/output/trace domains and restriction maps are stated. Discharged assumptions use set difference with explicit entailment. Sequential/shared/atomic operators are relations. Shared interference stability is inside shared cells along Ord. No ordered-composition placeholder.

### C07

CompleteObs includes lockConsumed and envelopeConsumed. rho is sort-preserving on sorted binders. PaymentAlloc is separate from WorkAlloc. Disjoint-parallel observations are a bag; serialization is a sequence. Associativity is restricted to both groupings admitted with equal cost/allocation/order/first-error prefix.

### C08

Partial duty 50 paid 20 leaves the same id at 30 with unique allocationId `A_partial_20` and transferId `T_partial_20`. Async lock duty starts as Alice refund-right; receive applies an authorized transform to Bob recipient-right.

### C09

Old domain is the retained-atomic-fixture bounded-atomic derive/createSimulator SUCCESS kind Simulation with synthetic `simulation-only` auth and checks. It is not production Prepared or Accepted. Pins include grammar, bounds.json, numeric-profile, loan.mori, evaluate.ts, programHash `95b46e39a9039e19063bb3d618128aec6cbd9ee656b6e635b3587e7f3f5235b2`, and registry boundsHash `ad0e1d45c9cfb5b1843d73f4d497d7d07f0450caddfcd49f3ef81f07f63d567c`. Concrete witness is the two Simulation steps. The unrelated Borrow/Swap/Repay toy is not old syntax witness. MC04/MC05 accepted-ledger transport is a separate conditional obligation. All eight theorem IDs remain.

### C10

Each of ten mutations has a full typed candidate or exact patch over the legal reference, derived first failure under the same validation order, required Transfer/ReceiveMessage fields, and NAM prefix AssignDuty creating `nam19-debt`. Rollback snapshots retain committed prefix cash/fees/work/gross.

### C11

Deadline predicate: receiveAdmitted iff Pending and authenticated destFinality.tick < deadline; refundAdmitted iff Pending and now >= deadline and not receiveAdmitted. At now=100 and finality=99 exactly the receive branch is admitted. Both receive and refund have complete postState and effects, including O_dest_M3. Destination truth stays an assumption.

## Unresolved limits

- Specified-only finite toy design. Not executed by a Moriarty language runtime.
- Theorems remain proposed. No axioms. No mechanized evidence.
- NAM19 remaining 26 upstream events stay pinned, not re-derived here.
- Toy footprint 64 is a design bound, not a registered runtime limit.
- Async destination finality is an assumed observation, not a proved foreign-adapter fact.
- Conservative extension is a mathematical relation on the Simulation domain. T is not implemented. No fake old production acceptance.
- SP01.3 signing correspondence remains open.
- Independent GPT-6 Astra review of this correction is required before any freeze.
