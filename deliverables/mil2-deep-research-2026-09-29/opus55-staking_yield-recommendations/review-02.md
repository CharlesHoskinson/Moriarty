# Staking, restaking and yield: validator and withdrawal lifecycle review

**Recommendation document. It covers a design that is only specified.** Nothing below claims an implementation, a proof, or acceptance by a ledger. I did not run any tests, proofs or compilers. The guarded status check reported that SP01.6 dispatch is blocked. That block does not affect this read-only review.

## 1. Verdict on whether the category design is fit

MIL/2 has the right **raw pieces** for staking lifecycles, but it cannot yet express one:

- a pending state is legal (`DESIGN-MIL2.md:215`);
- terminal tombstones and a required `priority` field exist (`:203-216`);
- locks are summed per owner (`:102`);
- stages are linked through the ledger (`:46`);
- a deadline is only a clock fact, not an entitlement (`:219`).

Three things are missing:

1. **No object for a withdrawal request.** Escrow has one fixed funding amount and two terminal branches (`:202-203`). An unbonding or queue request needs different things: it is finalized by a third party, can be finalized in part, has an amount decided at finalization, and is claimed later. The category report's use of escrow as the carrier for K11 and K12 (`08-staking-yield.md:31-32`) therefore overloads it.
2. **Lock accounting does not follow the lifecycle.** The semantics paper counts locks that "can still settle" (`MIL2-PROPOSED-SEMANTICS.tex:229-236`). It does not count stake that is unbonding but can still be slashed.
3. **Rate freeze, liquidity reservation and reward checkpoints have no rules.** R8 found that the reference models split on exactly this point: "rate freeze at finalise vs claim" (`R8-staking-yield.md:201-203`).

Fitness: **the substrate is adequate, and the category is not yet expressible.** Every repair below keeps the existing pieces: bounded stages, checked u128 with explicit rounding, native acceptance, E1 separate from debt, no global rollback, and a kernel that stays optional.

## 2. Five recommended design edits, ranked

### R1 (Recommendation). A core `request` transition relation, a sibling of escrow

```
request R {
  owner, asset A, claimClass C
  states   requested → (finalized(k) partial)* → claimable → claimed      -- claimed tombstoned
  freeze   at_request | at_finalize | min_of_both                        -- REQUIRED, like escrow.priority
  finalize_by AuthorityRef ; finalize_when Φ₀
  claim_when  Φ₀
}
```

- `request(id)` is a linear cell holding `{requested, finalized, claimed}` as u128 amounts.
- Invariant: `claimed ≤ finalized ≤ requested`.
- Finalization only increases `finalized`.
- A claim moves `finalized − claimed`. The tombstone is written when `claimed = requested`.

**Counterexample.** Model the request as escrow and partially finalize 40 of 100. Escrow can only choose release (all 100) or refund. Either the partial state has no representation, or a second escrow is opened, which breaks request identity. ERC-7540 needs this partial state: "If a Request … becomes partially claimable, all requests of the same requestId MUST become claimable at the same pro-rata rate" (external practice, https://eips.ethereum.org/EIPS/eip-7540).

**Placement.** The `request(id)` cell tag and the `freeze` enum go in **U0**. **Why this changes a U0 boundary:** the cell vocabulary and unknown-tag rejection are hash-bound at U0 (`DESIGN-MIL2.md:285`, `:345`). Adding the tag later forces a version migration, while reserving it now costs one enum entry. The fixed-quantity transitions go in **U3**, next to escrow and recovery (`ROADMAP.md:24`). Every guard stays in Φ₀.

### R2 (Recommendation). Unbonding stake stays locked and slashable until it completes

Extend K1 (`tex:229-236`):

```
lock_s(x) includes x.state ∈ {bonded, unbonding(t_c)} unless
      after(t_c) ∧ ¬∃ pending slash evidence e with e.infractionAt < x.unbondRequestedAt
```

- `spendable` still equals `balance − Σ lock` (`DESIGN-MIL2.md:102`).
- A slash transition may seize an unbonding entry. It follows the signed seize order (`:193`).

**Counterexample.** Suppose the lock is released when unbonding is requested. An owner unbonds 40 of 100 at t₀. Slash evidence for an infraction at t₀−1 then arrives, but only 60 remains encumbered. The share of loss that belongs to the exiting owner moves to the remaining delegators, even though every stage passes E1 and K1 as currently written.

External practice (from the documentation, not captured here): Cosmos `x/staking` keeps unbonding-delegation entries slashable for infractions committed before the unbond (https://docs.cosmos.network/main/build/modules/staking). Ethereum separates `exit_epoch` from `withdrawable_epoch` (https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md).

**Placement.** **U3**, as part of obligation O3 (`tex:250`). No U0 change is needed.

### R3 (Recommendation). Prefix-sum queue with liquidity reserved at finalization

Model the queue as O(1) cells, not as a list:

- cells: `queue(q).{nextId, lastFinalizedId, cumRequested, cumFinalized, reservedCustody}`;
- each request stores `cumRequestedAtEnqueue`.

The finalize transition moves the head from `i` to `j`:

```
j > i  ∧  j−i ≤ capBatch
  ∧ cumRequested[j] − cumRequested[i] = Δ         -- FIFO prefix, no reordering
  ∧ transfer Δ' from pool to reservedCustody(q)   -- E1 holds; Δ' = Δ for fixed-quantity, ≤ Δ under R4
```

The claim guard is `id ≤ lastFinalizedId ∧ ¬tombstone`. The claim pays from `reservedCustody` and cannot fail for lack of liquidity.

**Counterexample (liquidity).** A 100 A buffer and two finalized requests of 75 each. Both show "claimable". The first claim succeeds and the second becomes unpayable. "Claimable" was a false statement, and E1 never checked it, because finalization moved no funds.

**Counterexample (ordering).** Without the prefix rule, a finalizer holding `finalize_by` authority can favour later requests.

**Why the prefix-sum shape matters for bounded execution.** Φ has no loops or arrays (`DESIGN-MIL2.md:338`), and each stage is capped at 16 effect lines and 32 footprint cells (`:279-280`). A batch that wrote one cell per request would hit the caps. Prefix sums finalize any batch size with a constant number of writes.

**Placement.** The counters and the reservation transfer are **library** code over core cells. They go in **U3** for the fixed-quantity case and in **U6** for liquid staking. Whether FIFO applies is a library policy, but it must be declared in signed terms.

### R4 (Recommendation). Rate freeze uses `min_of_both`, with rounding set by role

At request, the owner's shares move to queue custody and are **not burned**. Record:

```
assetsAtRequest = assetsFor(shares, totalAssets, totalShares, Down)
```

At finalization:

```
payout = min(assetsAtRequest, assetsFor(shares, pre(totalAssets), pre(totalShares), Down))
```

Then burn the shares. That burn is supply delta = −shares, which needs the `issue` right on the share asset. The truncation residue is `retained-in-pool` (`DESIGN-MIL2.md:89`). The rounding rule for this role is **Down whenever the exiting party is paid**. For a claim with `finalized(k)`, apply the same rounding to each part, and pay the final claim as `payout − Σ prior`, so no residue drifts.

**Counterexample for `at_request` alone.** A slash occurs between request and finalization. Exiting users are paid at the rate from before the slash. The loss falls entirely on holders who stay: loss escapes through the queue.

**Counterexample for `at_finalize` alone.** Rewards that accrue while a request waits go to users who have already left.

External practice: ERC-7540 says assets received "MAY NOT be equivalent to … convertToAssets(shares) at the time of Request", and its preview functions revert (URL above). R8's Lido model freezes the rate at request (`R8-staking-yield.md:205-208`). The `min` rule is my recommendation, not a MIL/2 rule.

**Placement.** This needs Φ₁ (`assetsFor`), so it goes in **U4**, where Φ₁ is placed (`DESIGN-MIL2.md:350`), and **U6** for the library. I am **not** asking for Φ₁ at U0. Deferral is justified by the measured QF-BV cost (`:163`), and every product needs the two-limb gadget (`:171`).

### R5 (Recommendation). A reward index with a named writer and a mandatory checkpoint

- Add a cell `index(pool, class)` and a cell `rewardDebt(pool, class, acct)`.
- The index is monotone non-decreasing. Only an `accrue` scope may write it: an `issue`-scoped right on the reward asset (`:193`), or a pool-yield write for which a funded transfer into the pool is shown.
- A decrease is allowed only through a declared loss or slash transition.
- **Checkpoint rule:** any effect that writes `shares(pool, class, acct)` or a bonded amount must, in the same stage, settle `pending = shares·index − rewardDebt` (Down) and reset `rewardDebt = post(shares)·index`.

**Counterexample.** Deposit just before an index update, then claim. With no checkpoint, the new depositor collects rewards for the whole epoch (reward sniping). Also, a pool can mint rewards with no funded source, which confuses emission with yield (K7, `08-staking-yield.md:27`).

**Placement.** The `index` and `rewardDebt` cell tags go in **U0**, for the same hash-binding reason as R1. The arithmetic goes in **U4 (Φ₁)** and the library in **U6**.

## 3. Core versus library boundary (recommendation)

**Core:**
- the `request` state machine, its tombstone and the partial-finalization invariant;
- the required `freeze` and ordering-policy fields;
- lock accounting for unbonding entries that can still be slashed (R2);
- the new cell tags;
- the rule that a writer of `index` must hold a scoped right;
- validator events entering only as `Obs<ValidatorEvent, ε, d>`, which stay `imported` or `attested` until U4 (`DESIGN-MIL2.md:187`).

**Library:**
- the prefix-sum queue;
- FIFO versus other declared orders;
- the batch cap;
- the chosen freeze policy;
- the reward schedule and emission curve;
- AVS allocation;
- Lido, ERC-7540 and Cosmos-style profiles;
- K18 reaction bounds (queue duration against liquidity), written as declared obligations, never as guarantees.

**No core liveness claim.** A finalization that depends on a validator exit names inclusion, finality and attester assumptions under the recovery-viability obligation (`DESIGN-MIL2.md:219`; `tex:204-205`; product contract `:47`).

## 4. Smallest slice that can be implemented, with positive and hostile evidence

**Slice (U3, Φ₀ only, single domain, one signer, no conversion).** This is fixed-quantity delegation unbonding using R1 and R2.

1. Owner holds 100 A with a bonded encumbrance of 100.
2. A request stage creates `request(r) {requested 40, unbond t_c}`. The encumbrance becomes bonded 60 plus unbonding 40.
3. A claim stage at `after(t_c)` transfers 40 A from custody to the owner and writes the tombstone.

All guards are difference logic plus one lock sum, with zero supply delta under E1.

**Positive control.** Run the claim stage at `t_c + 1` with a valid signature, the complete effects (`balance(custody, A) −40`, `balance(owner, A) +40`), the tombstone written, and derived footprint ⊆ declared footprint. It must be accepted by native verification, with effects read back.

**Hostile control (discriminates R2).** Same predecessor state. After the unbond request and before `t_c`, submit a correctly signed and well-formed transfer of 1 A from the owner. Spendable is `100 − (60 + 40) = 0`, so the transfer must be rejected by the `LocksSafe` constraint, not by an envelope error (`ROADMAP.md:40`). Under the current reading, where an unbond releases its lock, the same witness would be **accepted**. That is why the pair discriminates.

A second hostile variant: a claim at `t_c − 1`, rejected by the `after` guard.

## 5. Explicit disagreements

- **With the category report** (`08-staking-yield.md:31-32`): escrow is not the carrier for K11 or K12. A separate `request` relation is needed (R1).
- **With `tex:229`**: "can still settle" is too narrow for K1. It must also mean "can still be seized" (R2).
- **With `DESIGN-MIL2.md:89`**: `retained-in-pool` is right as the *default* residue. For queue payouts made in parts, the final claim must take the remainder (R4), or residue builds up in each request.
- **With the rule that `enforce` is bounded by consent at origination (`:193`)**: this is necessary but not sufficient for slashing. The slash also needs authenticated external evidence of the infraction and its time, which is currently only a trust premise.
- I agree that Φ₁ stays out of U0. I disagree with any plan that postpones the **cell tags** as well.

## 6. Residual assumptions

- Validator exit, slash and infraction-time evidence is `imported` or `attested` until U4. Verifying it inside the circuit is open.
- The prefix-sum counters fit in u128 only under a declared lifetime cap. They wrap after roughly 2¹²⁸ smallest units, and that cap must be stated.
- The batch cap and per-stage caps are unmeasured proposals (`DESIGN-MIL2.md:270`).
- The rules for slashing unbonding entries follow external practice. The Midnight-native staking semantics that Moriarty would bind to are not identified in the repository.
- The encumbrance sum, footprint containment and recovery viability are all still obligations (`:365`). None of these recommendations discharges them.
- The external citations for Cosmos and Ethereum were not captured in `source-text/`. I fetched only ERC-7540 during this review.