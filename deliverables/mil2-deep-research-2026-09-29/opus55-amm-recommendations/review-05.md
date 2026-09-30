# Opus 5.5 AMM recommendation 5: adversarial acceptance and release plan

**Scope.** This is a read-only review. I loaded the develop skill and ran `status --json`. It reports that SP01.6 dispatch is blocked by stale inputs. That block does not apply to this review. I edited nothing and ran no tests. The Uniswap v2, ERC-4626 and Balancer patterns below are comparative practice (`opus55-amm-recommendations/PRIMARY-SOURCE-NOTES.md:5-8`). They are not MIL/2 requirements.

## 1. Verdict

**Design inference:** For one spot swap, MIL/2 works as a trader-intent envelope. The typed budget, nominal assets, Φ₀ floors, escrow and residue are enough to bind what the owner pays and receives (`DESIGN-MIL2.md:53-78, 289-324`). It **does not yet work as an AMM acceptance design**, for four reasons:

- The one worked example breaks its own footprint rule (`MIL2-PROPOSED-SEMANTICS.tex:218-227`).
- No pool transition exists.
- `fees` is undefined for fees that stay inside a pool.
- Route hops, pool identity and stale reserves are unbound (`01-amm-exchange.md:12-15`).

**Recommendation (the main point):** The category review treats `x'·y' ≥ x·y` as blocked until Φ₁ reaches U4 (`01-amm-exchange.md:13, 25`). That mixes up two layers. The product contract separates developer-side *contract invariants and transition rules* from user-signed *outcome constraints* (`MORIARTY-PRODUCT-CONTRACT.md:37`). The pool invariant is a contract-side transition rule. It can be a **fixed, certified native primitive**, which U1 already requires for "every primitive needed by the initial slice" (`ROADMAP.md:22`). It does not have to be a Φ₁ term that users can compose inside intents.

Φ₁ was deferred because of the **authoring-time SMT cost** on intents (`DESIGN-MIL2.md:163`). A runtime constraint check with bounded widths does not need a solver. So U0 keeps Φ₀ unchanged, Φ₁ stays deferred, and the first profile stays single-signer.

## 2. Top five changes, ranked by safety and then reviewability

### Change 1: Canonical cell identity and alias-closed derived footprints (U0 schema; U1 checker)

**Rule (recommendation):**

```
key(cell) := (domain, kind, nominal ids…)            -- after completion σ
Foot(transfer(d,a,p,q,n)) := R=W={balance(d,p,a),balance(d,q,a)} ∪ Aux     -- as F1
Foot(swap(pool,…))       := R=W={pool(id), balance(d,poolAcct,a_in), balance(d,poolAcct,a_out),
                                 balance(d,payer,a_in), balance(d,recipient,a_out), replay(intent)}
Admit iff  key-set(declared) ⊇ key-set(derived)  after σ,
       and every hole-selected cell resolves to a statically bounded key set,
       and transfer with key(p)=key(q) → Reject(SELF_TRANSFER)
       and recipient ∉ {poolAcct, escrow custody} unless declared.
```

The public statement binds the resolved keys. Declaring `escrow(E)` does **not** cover the balance cells it owns.

**Counterexamples this closes:**

- The AcquireB omission (`MIL2-PROPOSED-SEMANTICS.tex:221-227`).
- A route hole that sets `recipient := poolAcct`. The output is recredited to reserves, conservation (E1) still holds, and the owner receives nothing, yet the declared footprint looks disjoint.
- Two hops whose pool holes resolve to the same pool. Disjointness certifies them as independent, but they are not.

**Stop rule:** If any declared ⊉ derived mutation is accepted by the **native** verifier (not rejected by envelope parsing, per `ROADMAP.md:40`), do not freeze U0 and do not make any later AMM claim.

### Change 2: `cp_swap` as a certified contract-side primitive that accepts by inequality and never by division (U1 certificate; U2 slice)

**Rule (recommendation).** Pool profile `cp/1` has `x, y : u112` internal reserves, a fee `φn/φd` with `φd ≤ 2^14`, and a version counter `v`.

```
pre:   pool(id) = (x, y, v, φn, φd, progId)   -- authenticated current-head read
wit:   dx, dy
chk:   x+dx < 2^112 ∨ Reject(POOL_RANGE);    dy < y  (range-checked, not field-subtracted)
       x' = x+dx; y' = y−dy
       INV(dy):  (x'·φd − dx·φn)·y' ≥ x·y·φd                    -- products < 2^239
       TIGHT:    ¬INV(dy+1)                                      -- exact floor to trader
post:  pool(id) = (x', y', v+1, φn, φd, progId); both reserve balance cells updated
```

**Design inference on widths:** 112 + 112 + 14 bits is under the 253-bit `less_than` limit (`DESIGN-MIL2.md:169`), so `cp/1` needs no two-limb gadget. The intent's `u128` `Qty` still needs the limb rule, and the `POOL_RANGE` check narrows it.

Each comparison result must pass through `constrain_to_boolean` and then `assert` inside the circuit (`DESIGN-MIL2.md:265`). A host-computed Boolean is never enough.

**Race rule:** The statement binds `(poolId, v, reservesCommit)`. The ledger rejects the swap if `v` is not the current version. Swaps on one pool are therefore serialized. This is optimistic concurrency, not n-party clearing.

**Counterexamples closed:**

- A witness `dy > y` whose `y−dy` wraps modulo p.
- Over-delivery `dy+1`.
- Under-delivery that still meets the trader floor. A solver who is also an LP could otherwise move surplus to itself; `TIGHT` forbids this.
- A stale reserve snapshot proved at head h and landed after another swap.
- Reserve overflow near 2^112.

**Stop rules:**

- If the pinned Midnight target cannot bind the `v` read as an authenticated ledger read, stop. Do not substitute a host freshness check.
- If any admitted width combination exceeds 253 bits, reject that pool profile. Do not add limbs silently.

### Change 3: Fee classification with trader-conservative rounding (U0 numeric profile)

**Rule (recommendation):**

```
fees(stage) := Σ explicit fee transfers + Σ_hops ⌈dx_i·φn_i / φd_i⌉      -- embedded LP fee, rounded UP
gross(episode) := Σ owner debits (incl. escrow funding, fees); refund never decrements it
net := Σ credits to signed recipient in the signed asset
Accept iff fees ≤ F ∧ gross ≤ G ∧ net ≥ N ∧ per-hop fee vector is in the public statement
```

The budget check verifies `c·φd ≥ dx·φn` for a witnessed `c`, so no division is needed.

**Counterexamples closed:**

- **Pool-retained fees:** If they fall outside `fees`, a solver can route through a 10% pool and still clear `fees ≤ 1 A` while `net ≥ 20 B` holds. The signed fee cap becomes empty.
- **Dust splitting:** If fees rounded down, splitting a trade across hops would floor each hop's fee to zero.
- **Refund reset:** A refund followed by a re-spend would get around the gross cap (`AGENTS.md:83`).

This makes the "per-primitive rounding direction and beneficiary policy" of `ROADMAP.md:21` concrete:

| Quantity | Rounding direction | Beneficiary |
|---|---|---|
| Swap output | Toward the pool | LPs |
| Budget fee | Up | Trader protection |
| Share mint and burn outputs | Toward the pool | LPs |

**Stop rule:** Do not freeze `fees` until the owner or a council records that pool-retained fees count against it. If the answer is no, a separate signed `embedded_fees ≤` clause becomes mandatory.

### Change 4: Route completion as an exactly chained, venue-bound hop list (U0 schema; U2 single hop; U3 two candidates)

**Rule (recommendation):**

```
Route := [Hop]_{1..3},  Hop := (poolId, progId, a_in, a_out, amt_in, amt_out, domain)
Fill(route) requires:
  hop_1.a_in = signed A, payer ∈ {owner, E};  hop_n.a_out = signed B, paid to signed recipient
  ∀i<n: hop_i.a_out = hop_{i+1}.a_in ∧ hop_i.amt_out = hop_{i+1}.amt_in     -- no leakage
  ∀i: hop_i.domain = executing domain;  progId ∈ signed venue_policy (set of program/circuit ids)
  poolIds pairwise distinct (profile 1)
```

Each hop runs `cp_swap` against its own authenticated pool head.

**Counterexamples closed:**

- **Intermediate skim:** hop 1 outputs 100, hop 2 takes 90, and a solver keeps 10.
- **Fake pool:** a permissionless program with the same assets but no invariant check (product contract `:5` allows anyone to deploy one).
- **Repeated pool:** A→B→A→B through one pool, which needs sequential reserve composition.
- **Foreign hop:** a hop on another domain presented as settled (`DESIGN-MIL2.md:254`).

`venue_policy` is signed intent data. It is not a maintainer allowlist, so the permissionless rule holds.

**Stop rule:** Two independently produced routes (`ROADMAP.md:38`) must both be accepted, and the skim and fake-program mutations must both be rejected natively. Otherwise no route-neutrality claim is made.

### Change 5: LP shares as a ledger asset issued only by the pool program, with inequality-only mint and burn (U1 primitive; U3 evidence; U6 library)

**Rule (recommendation):**

```
ShareAsset(P,C) := AssetId with issuer = progId(P); issue right held only by progId(P)
invariant: supply(ShareAsset(P,C)) = totalShares(P,C)
bootstrap (S=0): s = da − L0, L0 minted to a permanent sink; require da > L0 ∧ db > 0
mint (S>0):  s·x ≤ da·S ∧ s·y ≤ db·S ∧ ¬(s+1 satisfies both) ∧ s ≥ signed min_shares ∧ s ≥ 1
burn:        oa·S ≤ s·x ∧ ob·S ≤ s·y ∧ tight ∧ oa ≥ min_a ∧ ob ≥ min_b
reserves are internal state; direct transfers to poolAcct never enter x,y
```

Excess deposit stays in the pool and counts toward gross. For widths, `S : u112` gives products below 2^224.

This resolves the mismatch between the `Share` sort and an `issue` right scoped to `(domain, asset)` (`01-amm-exchange.md:14`; `DESIGN-MIL2.md:192`).

**Counterexamples closed:**

- **Inflation by donation:** the first depositor mints 1 share and donates reserves, so the next depositor's shares round to 0.
- **Zero-share mint.**
- **Rounding-up burn** that drains the pool.
- **Cross-pool burn:** shares of pool Q burned against pool P. The nominal `ShareAsset` issuer is bound in the circuit, not only in the type.
- **Supply drift:** `totalShares` diverges from ledger supply.

**Placement:** The bootstrap choice between a lock and a virtual offset is already required by `DESIGN-MIL2.md:89`. I recommend the lock, because a virtual offset needs no extra arithmetic but hides value in LP accounting.

**Stop rule:** Any accepted mint with `s = 0`, or any burn with `oa·S > s·x`, blocks the LP claim.

## 3. Library versus core

**Core (recommendation):**
- Canonical cell keys and alias-closed footprint derivation.
- The `cp_swap`, `cp_mint` and `cp_burn` certified primitives with range checks and tightness.
- Pool version and head binding.
- Fee classification and rounding in the numeric profile.
- Route hop chaining inside `Fill`.
- Share-asset issuer binding.
- Replay cells.

A library cannot provide these because each one is part of what native acceptance must reject.

**Library (recommendation):**
- Quote functions (`getAmountOut`/`getAmountIn` style).
- Route search and solver heuristics.
- Fee-tier menus, which are parameters committed in pool state.
- Slippage and display helpers.
- Exact-output wrappers: the trader signs `gross ≤ G, net ≥ N`, and the solver picks `dx` with `cp_swap` checking only the pool side.
- LP portfolio views.

A library quote is advisory and never an acceptance input.

**Deferred, unchanged:**
- Concentrated liquidity (`DESIGN-MIL2.md:338`).
- Flash accounting (`:332`).
- Stable and weighted pools, which need bounded iteration or powers.
- Hooks.
- Batch and uniform clearing (post-U4, `|S|>1`).

## 4. Smallest implementable slice and evidence pair

**Slice (recommendation):**
- One `cp/1` pool with fixture assets A and B at decimals 0, x = 1,000, y = 2,200 and fee 30/10000.
- One single-signer Φ₀ intent: `gross ≤ 11 A`, `fees ≤ 1 A`, `net ≥ 20 B` to owner.
- One hop, one stage, no escrow. The escrow wrapper is U3.

**Arithmetic (design inference, computed by hand):**
- dx = 11 and x' = 1,011.
- Left factor: 1,011·10,000 − 11·30 = 10,109,670.
- Right side: x·y·φd = 22,000,000,000.
- dy = 23: y' = 2,177, and 10,109,670·2,177 = 22,008,751,590 ≥ 22e9, so INV holds.
- dy = 24: y' = 2,176, and 10,109,670·2,176 = 21,998,641,920 < 22e9, so INV fails. TIGHT therefore holds at 23.
- Fee: ⌈11·30/10000⌉ = 1 ≤ 1.

**Positive control:** dy = 23 to the owner, fee vector [1], v 0→1. Native verification passes and a readback shows x' = 1,011, y' = 2,177, owner +23 B and owner −11 A.

**Hostile twin:** the identical statement with dy = 24. The only difference is one witness value, and the envelope is valid. The native verifier must reject it with an INV failure.

Add these as hostile cases in the same batch, each differing from the positive control in one field:
- dy = 22 (TIGHT).
- dy = 2,201 (wrap/range).
- Stale v = 0 after a competing accepted swap.
- `recipient := poolAcct`.
- A fake `progId` outside `venue_policy`.
- A 10% fee pool (fee 2 > 1).
- dx = 12 (gross).
- Replay of the accepted intent.
- A host-supplied `INV = 1` bit with the constraint removed.

**Slice stop rule:** Do not claim the slice until the positive control is accepted **and** every hostile case is rejected natively on pinned artifacts. One accepted hostile case voids the claim. A rejection that only local evaluator or K produces is not acceptance evidence.

## 5. Disagreements

1. **With the category review, rows 13–15 and correction 1:** The pool invariant does not have to wait for Φ₁ or U4. Contract-side certified primitives fit U1/U2. Φ₁ in intents stays deferred. *(Recommendation.)*
2. **With MIL/2 §3.3** (`DESIGN-MIL2.md:89`): one remainder class, `retained-in-pool`, is not enough. Each primitive needs its own inequality direction and a tightness rule. Without tightness, "toward the pool" still leaves surplus to the solver's discretion, and a solver who is also an LP can capture it. *(Design inference.)*
3. **With the category review, row 18:** RFQ does not strictly require `|S|>1`. A maker can fund a program escrow under its own single-signer intent with a quote predicate, and the taker's stage consumes it under a signed priority for last-look and cancel races (`DESIGN-MIL2.md:195-219`). Atomic two-party signing still needs arity greater than 1. *(Recommendation. Unverified cost.)*
4. **With the category review, row 21:** Surplus is not entirely open for `cp/1`. TIGHT sends all of it to the trader up to the pool's exact floor. Ordering and MEV remain open, as residue. *(Design inference.)*
5. **Agreement:** The AcquireB footprint must be fixed with explicit cells. Counting `escrow(E)` as covering them is unsound under aliasing.

## 6. Remaining risks and assumptions

- **Assumption:** The pinned Midnight ledger can bind a contract-state version read as an authenticated read and reject a stale one. This is unverified in this corpus, and Change 2 depends on it.
- **Risk:** Reviewers may see certified pool primitives as a back door around the Φ₀/Φ₁ decision. The mitigation is that the primitives are fixed, not composable in intents, and need no authoring solver. That still needs an explicit owner decision.
- **Risk:** u112 reserves exclude assets whose smallest-unit reserves exceed about 5.2·10³³. A later profile would need limbs.
- **Risk:** Serializing swaps per pool limits throughput. Batching stays post-U4.
- **Risk:** Counting pool-retained fees in `fees` may conflict with how wallets display fees (Change 3 stop rule).
- **Risk:** Internal reserve accounting leaves direct donations stranded. That needs a declared `skim` owner or a residue line.
- **Unknown:** How the legacy fixture `swap.mori` relates to `cp/1` (`01-amm-exchange.md:13`). Do not count its results.
- **Evidence gap:** None of the arithmetic above is executed. The positive and hostile numbers are hand-computed design inferences that need TypeScript/K differentials and native runs. No paper, protocol source or graph edge shows Moriarty behavior.