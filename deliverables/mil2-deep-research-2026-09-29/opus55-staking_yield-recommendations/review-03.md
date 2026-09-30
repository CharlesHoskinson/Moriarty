# Restaking and slashing in MIL/2: independent recommendation (Opus 5.5, restaking lens)

**Scope.** This is read-only design review of a specified-only design. Nothing here is implemented, proved or accepted on a ledger. I ran the `status` check; it reports SP01.6 blocked, which does not block this review. Every numbered edit below is a **recommendation**, not an existing Moriarty rule. EigenLayer ELIP-002 (https://github.com/eigenfoundation/ELIPs/blob/main/ELIPs/ELIP-002.md) is cited only as outside practice.

## 1. Verdict

The pieces MIL/2 provides for restaking are sound: an encumbrance that can back a set of obligations, a lock-sum bound, consent-bound `enforce`, and signed seize order (`DESIGN-MIL2.md:93-102`, `:193`). But the design is **not yet fit** for restaking. Five things are missing:

- **No lifecycle for an encumbrance.** There are no states, no slashable window and no deallocation delay.
- **The lock-sum rule does not stop double pledging here.** K1 (`DESIGN-MIL2.md:102`, `MIL2-PROPOSED-SEMANTICS.tex:229-235`) bounds each lock once. When one lock backs several obligations (`against`), nothing stops each creditor seizing up to the full `amount`.
- **Three priority notions that nothing connects.** `Obligation.rank`, `Encumbrance.priority` and "signed total order" (`:95-97`, `:193`) have no stated relationship, and the design never says who signs the order.
- **No adjudication evidence shape.**
- **No way to slash a pool without per-holder effect lines.** Per-holder lines cannot fit the caps of 16 effect lines and 32 footprint cells (`:279-280`).

The category report reaches the same partial verdict (`08-staking-yield.md:11`, `:29-30`).

## 2. Five ranked design edits (recommendations)

### E1. Encumbrance lifecycle with a slashable window and operator allocation authority

**Rule sketch.** Extend the record with `state ∈ {reserved, active, deallocating, released, exhausted}`, `effective_at: Instant(c)`, `slashable_until: Instant(c) | none`, `seized_total: Qty<A>`, and `allocator: AuthorityRef`. The transitions are:

- `allocate`: reserved → active, only when `after(effective_at)`.
- `deallocate`: active → deallocating, setting `slashable_until = now + delay_decl`.
- `release`: deallocating → released, only when `after(slashable_until)` and no seize reservation is pending (see E3).
- Any state → exhausted when `seized_total = amount`.

`lock_s(x) = amount − seized_total` for every state except `released`, and K1 applies to that value. A seize is legal only in `active` or `deallocating`, and only when `effective_at ≤ offense_time ≤ slashable_until`.

**Operator authority.** An operator may allocate or deallocate only under a standing staker `ConsentRef` that bounds the total, the asset and the minimum delay.

**Counterexample.** An operator deallocates stake and immediately reallocates it to service B. Service A's offense, committed before the deallocation, can no longer be collected. Under the current text, deallocation is just a lock release, and `tex:229`'s "can still settle" has no defined meaning.

**Placement.**
- **U0:** the record fields and state tags in the canonical encoding, and a **reserved** right-kind tag `encumber`.
- **U3:** the transitions, as persistent duties.
- **Proof:** O3.

**Why touch the U0 boundary.** The right-kind enum is frozen at U0 (`:345`), and unknown tags are rejected (`:285`). Without a new tag, operator allocation has only two routes. It can go through `amend`, which rewrites terms and is too broad. Or it needs a staker signature on every allocation, which defeats delegation. Reserving the tag is cheap now; adding it after the digest is hash-bound needs a version change.

### E2. A correlated-loss budget for multi-obligation encumbrances

**Rule sketch.** `against: {(ObligationId, cap: Qty<A>)}`, with:
- per-obligation `seized_o ≤ cap_o`;
- aggregate `Σ_o seized_o = seized_total ≤ amount`.

A declared mode, signed by the owner, fixes the relation between caps and the total:
- `unique`: Σ cap_o ≤ amount, which gives EigenLayer-style unique stake;
- `shared(r)`: Σ cap_o ≤ r × amount, with a literal r. This is Φ₀ because r is a literal coefficient (`:160`).

Each creditor's consent record commits to the mode and the co-obligation set, so a creditor knows how much its collateral is correlated.

**Counterexample.** One lock of 100 A backs o1 and o2, each "enforceable" for 100. o1 seizes 100. o2's seize then either fails on underflow or, if a policy caps each seize separately, succeeds against collateral already taken. Either way, o2 relied on collateral whose correlation it never saw.

**Placement.**
- **U0:** Φ₀ grammar and the canonical fields.
- **U3:** enforcement.
- **Proof:** O3, strengthened to cover `seized_o` as well as the lock sum.

### E3. One ordering: origination priority plus seize reservations, replacing the per-stage "signed total order"

**Rule sketch.**
- `Encumbrance.priority` becomes a total order over its `against` members. It is fixed at origination under owner consent and is visible to every creditor.
- Adjudication writes a `reservation(enc, o, q)` cell, and the reservation counts in `lock_s`.
- `seize(enc, o, q)` is accepted only if `q ≤ amount − seized_total − Σ{reserved q' : priority(o') < priority(o)}`. In words, a junior seize must leave room for every senior reservation still pending.
- Within one stage, ties use the same declared order. Across stages, the ledger's authenticated order applies.

**Counterexample.** A senior claim is under adjudication while a junior enforcer seizes everything first. Under `:193`, the order is signed by an unnamed party at seize time, and with `|S|=1` (`:336`) the only signer is the enforcer itself, who can put itself first.

**Placement.**
- **U0:** the reservation cell kind. It fits under the existing `encumbrance(id)` cell or is added as a sub-cell.
- **U3:** the rule, as a late-race analogue of `ESC` (`tex:192-200`).

### E4. Adjudication verdicts as typed, replay-consumed evidence

**Rule sketch.** `Obs<SlashVerdict{verdictId, obligation, offense_time, q}, ε, d>`, where the evidence class ε must match the policy committed in `Obligation.consent`. The seize guard is:

```
verdict.obligation = o ∧ q ≤ verdict.q ∧ final(verdict) ∧ offense-window(E1) ∧ ¬consumed(replay(verdictId))
```

The effect writes `replay(verdictId)`.

Seized proceeds discharge the obligation through L1 (`tex:180-187`), and AccrualFirst splits the payment. Any outstanding shortfall **persists**. It is never forgiven by the seize, which keeps debt separate from supply.

**Burning.** A "burn" is a transfer to a declared, provably unspendable sink custody account inside E1's quantified set. It is not a supply delta, because a restaker has no `issue` right on NIGHT (`:193`, `:256`). As outside practice, ELIP-002 burns ERC-20s by sending them to a dead address.

**Counterexamples.**
- One verdict is replayed to seize twice.
- A verdict for service A is presented under service B's obligation.
- A verdict signed by an issuer outside the consent policy is used.

**Placement.**
- **U0:** the evidence index and `replay` cell, which already exist (`:176-185`, `:242`).
- **U3:** a Midnight-anchored verdict, for example from a local adjudicator contract.
- **U4:** attested or imported verdict verification. That evidence stays a trust premise until then (`:187`, `:350`).

### E5. Pooled slash as an aggregate write-down, not per-holder effects

**Rule sketch.** `writeDown(pool, class, q)` produces one balance line from pool custody to the creditor or sink, and decrements `totalAssets(pool)`. `totalShares` is unchanged. Holder claims are derived afterwards through `assetsFor(…, Floor)`, with residue `retained-in-pool` (`:89`).

The obligation is `Σ_holders assetsFor(shares_h) ≤ totalAssets`, the `shareBound` invariant. The rounding direction by role:
- a proportional loss rounds **down** on holder claims;
- the seized amount is `floor(locked × wad / 10^18)`, which favours the debtor, since the creditor's consent is "up to".

A cross-class waterfall uses `ClaimClass` rank: junior classes are written down before senior ones.

**Counterexample.** 1,000 delegators need 1,000 effect lines, which the caps reject. Alternatively, a per-holder floor combined with an aggregate floor burn leaves claims that no assets back. R8 found this sum-of-floors versus floor-of-sum gap in `eigenlayer.qnt` (`R8-staking-yield.md:233-243`).

**Placement.**
- **U0:** reserve `writeDown` as an effect kind.
- **Fixed-amount write-down (Φ₀) at U3:** the amount is a literal verdict `q`, so no pool conversion is needed.
- **Proportional amounts and conversions** need Φ₁ (`sharesFor`/`assetsFor`/`mulDiv`, `:161`), so they wait for U4.
- **U6:** the waterfall library.
- I do not propose moving Φ₁ earlier.

## 3. Core versus library

**Core (recommended):**
- the encumbrance state machine;
- K1 extended with per-obligation and aggregate seized bounds;
- the reservation cell and the priority-bounded seize rule;
- the verdict replay cell;
- the `writeDown` effect kind;
- sink custody inside E1;
- the reserved `encumber` right.

These belong in core because they write shared linear cells and bind multiple creditors. A library cannot enforce them against another library.

**Library (U6):**
- operator-set and AVS templates;
- magnitude arithmetic and proportional wad slashing;
- tranche waterfalls and first-loss backstops;
- LST queue, finalize and claim;
- rewards;
- concentration bounds.

## 4. Smallest implementable slice (U3, Φ₀ only, one domain, `|S|=1`)

Owner O locks 100 A in encumbrance E in `unique` mode. E backs `{(o1, cap 60), (o2, cap 40)}`, priority o1 > o2. Each obligation has outstanding 60 A. Verdicts come from a local, Midnight-anchored adjudicator.

**Positive trace.**
1. Verdict v1 (o1, q = 60) authorizes seizing 60 to creditor C1. Result: `seized_total = 60`, o1 is discharged, `replay(v1)` is written.
2. Verdict v2 (o2, q = 40) authorizes seizing 40 to C2. Result: E is **exhausted**, and o2's outstanding stays at **20**.

The positive control must be a feasible valid witness with complete readback of every balance, obligation, replay and encumbrance cell.

**Hostile control.** After step 1, resubmit v1 against o1, with a well-formed envelope and a correct digest. It must reject at the `replay(v1)` constraint. It must not reject because the envelope is malformed (`ROADMAP.md:40`).

**Second hostile variant.** Submit v2 with q = 50. It must reject on the cap or aggregate bound.

## 5. Explicit disagreements

1. **With `08-staking-yield.md:11`.** The report says `Σ active locks ≤ balance` "forbids duplicate collateral allocation". It does not do so across obligations that share one multi-obligation encumbrance; E2 is needed.
2. **With `DESIGN-MIL2.md:193`.** A per-stage "signed total order" has no named signer and lets the enforcer rank itself first. Also, with `|S|=1`, the argument that footprint disjointness would otherwise reject concurrent seizes applies to forks, not to a single stage. Replace it with origination priority plus reservations (E3).
3. **With R8's "library only (U6)" placement of slashing (`R8:850`).** Seize, reservation and lock accounting must be core. Only the waterfall policy is library.
4. **With treating burn as a supply change for restaking.** Burning native assets needs `issue`; a sink transfer keeps E1 and debt separation intact.

## 6. Residual assumptions

- Adjudicator correctness and liveness before `slashable_until` are named trust and liveness premises. They are not proved.
- Attested verdicts remain unverified before U4.
- The unspendability of the sink account needs a ledger-level argument.
- Cap values (16 effects, 32 cells) are proposals and have not been measured (`:270`).
- The declared delays depend on the domain clock.
- The ELIP-002 parameters (14-day deallocation delay, magnitudes) are comparative evidence only.
- O3 and O6 remain unproved. So do the native correspondence (C1) and every U-milestone exit.