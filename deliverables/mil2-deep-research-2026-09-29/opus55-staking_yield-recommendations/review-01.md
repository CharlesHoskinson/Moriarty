# Recommendation: MIL/2 staking, restaking and yield, with a focus on vault accounting

**Lens:** ERC-4626-style vault accounting. **Status:** these are design recommendations for a specified-only language. I did not implement, prove, compile or run anything, and nothing here claims ledger acceptance. The guarded CLI `status` ran first. It reports that SP01 dispatch is blocked by stale inputs, which does not affect a read-only review.

## 1. Verdict

MIL/2 provides the right **carriers** for vaults, but its vault accounting is **not fit to freeze** as written. There are four defects:

- **Author-selected rounding.** `sharesFor` and `assetsFor` take a free `Rounding` argument (`concepts/intent-language/DESIGN-MIL2.md:85-86`). The U0 numeric decision fixes the direction by role and names author-selectable rounding as an open conformance gap (`docs/decisions/u0-numeric-profile-decision.md:12-16,22`). R8 identified this gap (`deliverables/u0-study-2026-09-28/defi-coverage/R8-staking-yield.md:403`).
- **Unbound conversion inputs.** The share and asset totals are ordinary arguments, not reads of the pool's own cells. A well-typed call can pass any total.
- **No share-supply law.** E1 conserves asset balances against asset supply (`MIL2-PROPOSED-SEMANTICS.tex:172-178`). No rule constrains writes to `shares(pool,class,acct)` (`DESIGN-MIL2.md:241`).
- **Incomplete bootstrap rule.** Bootstrap is named only as "virtual offset or first-depositor convention" (`DESIGN-MIL2.md:89`). The design does not say what `totalAssets` measures, so a donation can move the price.

The design also tags "remainder `retained-in-pool`" as **[checked]** (`DESIGN-MIL2.md:89`). That is a policy choice, and it conflicts with D2, which sends rounding remainders to the protocol reserve (`u0-numeric-profile-decision.md:16`).

## 2. Five ranked design edits (recommendations)

### R1. Replace the free `Rounding` argument with four method constructors that fix their rounding

Rule sketch, for one pool `P` and class `C`. `S = pre(pool(P).supply[C])` and `A = pre(pool(P).accounted)`. The pool's policy fixes the offsets `vS, vA`.

```
deposit(P,C,a,rcv):  s = ⌊a·(S+vS) / (A+vA)⌋   -- shares to the user: floor
mint(P,C,s,rcv):     a = ⌈s·(A+vA) / (S+vS)⌉   -- assets the user pays: ceil
withdraw(P,C,a,own): s = ⌈a·(S+vS) / (A+vA)⌉   -- shares the user burns: ceil
redeem(P,C,s,own):   a = ⌊s·(A+vA) / (S+vS)⌋   -- assets to the user: floor
```

These directions match ERC-4626's security guidance (`source-text/erc4626.md:631-634`, https://eips.ethereum.org/EIPS/eip-4626). They are also D2's own role rule, so no D2 override is needed for direction.

**Counterexample.** Take A=101, S=100, no offset. A caller-chosen `floor` on `withdraw(1)` burns ⌊100/101⌋ = 0 shares and pays out 1 asset. Repeating this drains the pool.

**Placement.**
- **U0:** reserve the four constructor tags and add their direction/beneficiary rows to the numeric profile. The U0 exit already requires "per-primitive rounding direction and beneficiary policy" (`ROADMAP.md:21`).
- **U1:** certify the two-limb floor/ceil `mulDiv` gadget.
- **U6:** admit the vault library.

**Boundary change, stated explicitly.** I recommend a U0 change and a change to the Φ₁ boundary.

- **Why U0:** otherwise U0 would hash-bind a header whose vault signature carries D2's open gap.
- **Why Φ₁:** these constructors should be **effect-grammar primitives**, not Φ terms. The circuit checks a quotient/remainder witness: `q·D + r = N` and `0 ≤ r < D`, with `N` a two-limb value (`DESIGN-MIL2.md:169-171`). Ceil uses `r = 0 ? q : q+1`.
- **What stays deferred:** Φ₁ was deferred because SMT authoring checks cost too much (`DESIGN-MIL2.md:163`). That cost does not apply to a closed-form computation checked from a witness. Variable-product **predicates** stay in Φ₁ and stay deferred.

### R2. Read conversion inputs from pool cells, snapshotted in the pre-state

Rule sketch: the totals come from `pre(pool(P))`. The prover cannot supply them. The derived footprint writes `pool(P)`, `shares(P,C,rcv|own)`, and `balance(d,P.custody,A)` together with the user's balance.

In the first profile, allow **at most one vault method per `(P,C)` per stage**. Several methods on one pool would need a defined sequential fold, which can come later.

**Counterexample.** A candidate stage passes `totalAssets = 1` to `sharesFor` while the pool actually holds 10⁶. Today's signature type-checks and mints 10⁶ times too many shares.

**Placement:** U0 for the cell vocabulary and snapshot rule. The authenticated ledger read of `pool(P)` belongs to U2 native enforcement (`MIL2-PROPOSED-SEMANTICS.tex:240`).

### R3. Add a share-supply conservation law, separate from asset supply and from debt

Rule sketch:
- **E1-S:** for each `(P,C)`, `Σ_acct Δshares(P,C,acct) = Δpool(P).supply[C]`.
- A nonzero share-supply change may come **only** from the four R1 constructors or a declared seed-burn. It is carried by a pool-scoped, non-delegable `issue` right (`DESIGN-MIL2.md:193`).
- **Paired asset law:** `Δpool(P).accounted = Δbalance(d,P.custody,A)` for method effects.
- **Solvency invariant:** `balance(d,P.custody,A) ≥ pool(P).accounted`.
- Shares are claims, not `Obligation`s. They stay separate from debt, as the product contract requires (`docs/MORIARTY-PRODUCT-CONTRACT.md:43`).

**Counterexample.** A stage writes `shares(P,C,attacker) += 10⁹` and moves no assets. E1 holds because no asset moved, so the stage is accepted.

**Placement:** extend U0's general conservation law (`DESIGN-MIL2.md:256,345`) with a reserved share quantifier. The share side of the encumbrance-sum pattern becomes a MIL/2 obligation, alongside `DESIGN-MIL2.md:365`.

### R4. Make bootstrap a signed, immutable pool policy and track assets in an accounted cell

Rule sketch:
```
BootstrapPolicy ::= virtualOffset(vS: u128 ≥ 1, vA: u128 ≥ 1)
                  | seedBurn(minShares ≥ 1, sink = unspendable)
```
- The policy is committed in the pool digest at creation. It is immutable, and an unknown tag is rejected.
- `pool(P).accounted` changes only through R1 effects or an explicit `recognize(P, amount)` effect. That effect needs a separately scoped authority and must satisfy `accounted' ≤ custody`.
- A donation into custody raises custody but does not move the conversion rate.
- **Judgment clause:** a post-state with `S>0 ∧ A+vA = 0` is unreachable when `vA ≥ 1`. A total-loss state (`A = 0`, `S > 0`) rejects deposits unless the policy declares otherwise. Any `S+vS` overflow rejects with `ARITH_RANGE`.

**Counterexample.** This is VX01 (`deliverables/erc4626-vault-report-2026-09-08/DESIGN-IMPLICATIONS.md:41`).
- Setup: no offset, price read from raw custody. The attacker deposits 1 for 1 share, then donates 100, so A=101 and S=1.
- Victim: deposits 100 and receives ⌊100/101⌋ = 0 shares.
- Attacker: redeems 1 share for 201.
- Under R4 with vS=1000, vA=1: even if the donation were recognized, the victim gets ⌊100·1001/102⌋ = 981 shares. The attacker's 1 share then redeems ⌊202/1982⌋ = 0, so the attack loses money.

**Placement:** U0 for the sort and tags. U6 for the library parameters. Choosing vS is an economic parameter, not a proof.

### R5. Express slippage as signed Φ₀ intent bounds, and add a round-trip obligation

Rule sketch: the signer writes a minimum-out bound directly over the pre/post share cells, which stays linear:
```
post(shares(P,C,owner)) − pre(shares(P,C,owner)) ≥ minShares
```

ERC-4626's `preview*` and `convertTo*` functions are **host tools only**. External practice lets `convertTo*` be estimates (`erc4626.md:624`). A Moriarty rule must never accept a stage on a preview value, because that would be a host-computed Boolean (`MIL2-PROPOSED-SEMANTICS.tex:130`).

Obligation O-V1 (recommended): for every admitted state, a deposit of `a` followed by redeeming all minted shares returns at most `a`. The same holds for `mint`/`withdraw`. Fees round up and go to a declared recipient that is included in E1.

**Counterexample.** A candidate carries `previewDeposit = 10` from an earlier state. The deposit is accepted, but after a front-run it mints 0 shares.

**Placement:** U2/U3 intent refinement. O-V1 is an executed TypeScript/K differential before U6.

## 3. Core versus library boundary (recommendation)

**Core** (it touches soundness, conservation or native enforcement):
- the share cell kind and E1-S;
- the four method tags with fixed rounding;
- the `pool(P).accounted` cell and the `recognize` authority;
- the `BootstrapPolicy` sort and the rate-defined judgment;
- the certified `mulDiv` floor/ceil gadget;
- the rounding-remainder beneficiary row.

**Library:**
- fee schedules and on-top versus gross quotes;
- `max*` limits;
- asynchronous request/claim queues;
- reward indices, rebase adapters and liquid-staking exit queues;
- strategy mandates;
- restaking allocation;
- previews used as quoting helpers.

## 4. Smallest implementable slice, with a positive/hostile pair

**Slice:** one pool, one class, `virtualOffset(1000,1)`. Only `deposit` and `redeem`, which both round down, so only the floor gadget is needed. The pool uses the accounted-assets cell and charges no fees. It is single-signer, single-stage, with a Φ₀ bound of `minShares`.

**Positive control (valid and feasible):**
- Pre-state: A=101 accounted, S=1, custody 201 (100 of it unrecognized donation).
- The owner deposits 100 with `minShares = 900`.
- Witness: N = 100·1001 = 100100, D = 102, q = 981, r = 38, and 0 ≤ 38 < 102.
- Effects: owner −100 A, custody +100 A, `accounted` +100, `supply[C]` +981, `shares(owner)` +981.
- Laws: E1 holds with Δsupply(A) = 0, E1-S holds, the solvency invariant holds (301 ≥ 201), and the intent bound holds (981 ≥ 900).

**Hostile control** (same pre-state and intent, one field mutated):
- The claimed minted amount is 982. Then r = 100100 − 982·102 = −64. No remainder in `[0,102)` exists, so the native constraint must reject.
- Secondary hostile variants: pricing from raw custody (201) instead of accounted (101); or r = 102 with q = 980.
- The rejection must come from the constraint system. A malformed envelope or a host flag does not count (`ROADMAP.md:40`).

## 5. Explicit disagreements

1. **With `DESIGN-MIL2.md:85-86`.** Rounding must not be a caller argument. It must be fixed per method.
2. **With `DESIGN-MIL2.md:89` [checked].** `retained-in-pool` needs an explicit D2 override with a rationale: the remainder benefits the remaining holders, and a separate reserve would need its own account. It should be tagged as a declared override, not as checked.
3. **With placing all vault conversion in U4/Φ₁** (`DESIGN-MIL2.md:161,350`; `MIL2-PROPOSED-SEMANTICS.tex:59`). The effect-side computation needs a U1 gadget, not a solver.
4. **With the category report's "all four ERC-4626 directions" framing** (`08-staking-yield.md:22`). Only the four mutating methods belong in core. `convertTo*` and `preview*` are views that acceptance must not use.
5. **With ERC-4626 as a Moriarty rule.** ERC-4626 permits an estimated `totalAssets` (`erc4626.md:624`). Moriarty needs an exact authenticated cell.
6. **With the report treating donation safety as a provable invariant** (`08-staking-yield.md:23`). Offsets only make attacks unprofitable up to a margin set by the parameters. The provable invariants are E1-S, the solvency invariant, O-V1, and a signed `minShares` bound.

## 6. Residual assumptions

- Custody for a non-rebasing asset has an authenticated ledger read on Midnight. It is not yet established whether shielded custody can support that read.
- Allowing one vault operation per pool per stage creates write contention: every depositor writes `pool(P)`. Resolving this depends on post-U4 multi-signer arity (`DESIGN-MIL2.md:336`).
- The authority behind `recognize` for yield and loss is trusted. Its policy is library scope.
- Offset values and gadget cost are **unmeasured**. The §12 caps are proposals (`DESIGN-MIL2.md:270`).
- ERC-4626 is cited as external practice. Only the rules above labeled as recommendations are proposed Moriarty rules, and none is adopted.