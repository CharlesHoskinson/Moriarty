I've checked the sources I need (the repository design, the lending report, the R2 requirements, and the official Aave, Compound and Maker liquidation specifications). Here is the recommendation.

# Lending and borrowing — liquidation and impairment lens (recommendation, specified-only)

I ran the guarded `status --json`. Implementation and repair of `SP01.6 loan-swap-subset` is blocked by stale inputs, missing accounting and unresolved operational history. There are no pending transactions. This review is read-only. I claim no implementation, proof or ledger acceptance.

## 1. Verdict

**Not fit yet for liquidation or impairment. The substrate is sound.** MIL/2 has the right building blocks:
- a persistent `Obligation` with rank and consent (`concepts/intent-language/DESIGN-MIL2.md:93-100`);
- set-valued encumbrances with an aggregate lock rule (`:96-102`);
- a consent-bounded `enforce` right and signed seize order (`:191-193`);
- a separate roll-forward term for forgiveness (`MIL2-PROPOSED-SEMANTICS.tex:180-186`).

It has no liquidation transition, close factor, bonus arithmetic, impaired status, deficit carrier, waterfall, or keeper and backstop funding. The category report says the same (`02-lending.md:21, :27`), and so does R2 (A7, A9, A10, H7 at `R2-lending.md:219-222, :482`). `seized(enc)` is only a Boolean (`DESIGN-MIL2.md:148`).

There is one more structural problem the category report underplays. The obvious health and seize formulas are `collateral × price` and `repay × (1+bonus) / price`. Both multiply two variables, so both are Φ₁ (`DESIGN-MIL2.md:160-161`). Φ₀ cannot state price-proportional liquidation in general. Edit 1 fixes this without moving Φ₁.

## 2. Five ranked design edits (recommendations)

### Edit 1 — Price-band liquidation transition, stated as inequalities

**Rule sketch.** `LiquidationPolicy` is digest-bound inside the obligation's consent. It holds:
- a finite band table `[(p_i, m_i, b_i)]` with at most 8 bands (the `k_of_n` cap, `:278`);
- `cf_bps`, `minFull`, `minLeftover`.

All values are literals. The transition `liquidate(o, enc, band i, r, q, keeper)` is:

```
pre:   fresh(px, Δ) ∧ px ! {anchored@d} ∧ px ≤ p_i
       locked(enc)·(p_i·10⁴) < outstanding(o)·m_i          -- unhealthy at band ceiling
       r·10⁴ ≤ pre(outstanding(o))·cf_bps  ∨  pre(outstanding(o)) ≤ minFull
       q·(p_i·10⁴) ≤ r·(10⁴+b_i)                          -- seize cap
effects: transfer r debtAsset keeper→creditor; seize q from enc → keeper
L1:    discharge_o = r (AccrualFirst split);  lock' = lock − q
```

Every product has one literal coefficient, so the rule stays in Φ₀ (`:140, :160`).

**Why the rounding is role-directed with no division.** Valuing at the band ceiling `p_i` under-reports how unhealthy the position is, and under-reports the seize amount. Both errors favour the debtor, the protected party. The keeper's loss is bounded by band width, so policy must require `b_i >` relative band width. This is the "beneficiary policy" the numeric profile demands (`ROADMAP.md:21`), expressed as the direction of each inequality.

**Counterexample closed.** Today a keeper can emit `seized(enc)` with any `q`, because nothing in MIL/2 relates `q` to `r` (`02-lending.md:21`).

**Placement.** The policy record and the transition schema are reserved in the U0 stage schema. The end-to-end slice is U2/U3. A general `value(q, px, rounding)` primitive stays Φ₁/U4. I am not changing the Φ₁ boundary.

### Edit 2 — Close factor evaluated at the authenticated head, plus a leftover rule

**Rule sketch.**
- `cf` is always computed on `pre(outstanding)` at the authenticated predecessor head, never on `post`.
- The obligation cell is in every liquidation's write set, so two same-head liquidations conflict and the signed seize order (`DESIGN-MIL2.md:193`) serializes them.
- Leftover rule: `post(outstanding)=0 ∨ post(outstanding) ≥ minLeftover`, and the same for `locked(enc)`.

**External practice.** Aave v3.3 allows a 100% close factor below `MIN_BASE_MAX_CLOSE_FACTOR_THRESHOLD` and forbids leftovers below `MIN_LEFTOVER_BASE` (https://github.com/aave-dao/aave-v3-origin/blob/main/docs/3.3/Aave-v3.3-features.md). Maker's Clipper rejects a partial take that leaves a tab below `chost` (https://github.com/sky-ecosystem/dss/blob/master/src/clip.sol). These are external practice, not Moriarty rules.

**Counterexamples.**
- Two keepers each repay 50% from the same pre-state, which closes 100%.
- A keeper leaves 1 unit of debt against 0 collateral that nobody will ever liquidate (dust griefing).

**Placement.** The footprint rule is U0 (cell vocabulary). The leftover guard is a U3 partial-progress control (`ROADMAP.md:24`).

### Edit 3 — Impaired status and deficit recognition, separate from forgiveness

**Rule sketch.**
- Add `Obligation.status ∈ {active, defaulted, impaired, discharged}` and `terms.recourse ∈ {full, none}`.
- Add cell `deficit(creditor|pool, asset)`.
- Transition `recognizeDeficit(o)`:
  - guard: `post(locked(enc))=0 ∧ outstanding(o)>0`;
  - under `recourse=none`: `authorizedForgiveness_o = outstanding(o)` and `deficit' = deficit + outstanding(o)`, with the same quantity and asset;
  - under `full`: status becomes `impaired`, the debtor's outstanding balance is unchanged, and the deficit is only recorded.

In L1 (`tex:180-184`), `authorizedForgiveness` must equal exactly the deficit that was recognized. The debtor's origination consent to non-recourse terms is what authorizes it.

**External practice.** Aave v3.3 burns residual debt and "the new deficit created is accounted to the reserve" (same URL). Burning debt is the external mechanism. The Moriarty rule is that forgiveness is a separate, matched effect (`docs/MORIARTY-PRODUCT-CONTRACT.md:43-45`).

**Counterexample.** A keeper seizes all collateral against 1,000 debt and recovers 800. The host marks the obligation `discharged`. Debt is erased with no discharge transfer, which is the `ROADMAP.md:40` "erased debt" control. A second variant: forgiveness while collateral remains.

**Placement.** The status enum and the `deficit` cell are U0, because U0 freezes the cell vocabulary and schema (`DESIGN-MIL2.md:345`). The transition is U3.

### Edit 4 — Bounded claim-class waterfall at class level (Φ₀)

**Rule sketch.** `allocateLoss(deficit D, classes c₁…c_k)`, with `k ≤ 4` in signed rank order. The backstop or reserve comes first, then junior, then senior.

```
x₁ = min(D, cap₁),  x_j = min(D − Σ_{i<j} x_i, cap_j)
trancheAssets'(c_j) = trancheAssets(c_j) − x_j
Σ x_j = D  or  residual deficit persists
```

`cap_j = pre(trancheAssets(c_j))`. Each class is charged only after every more-junior class is exhausted. Subtraction and `min` are Φ₀. The per-share value change appears later through Φ₁ `assetsFor`.

This needs one type change: `totalAssets : PoolId × AssetId` (`DESIGN-MIL2.md:84`) cannot hold tranches. Add `trancheAssets(p, c)` or index `totalAssets` by `ShareClass`.

Authority: use `enforce` scoped to `lossAllocation(pool policy)`. Do not add a ninth right, because the rights enum is a U0 freeze item (`:345`).

**External practice.** Compound III reserves "automatically protect users from bad debt" (https://docs.compound.xyz/liquidation/). That is a single backstop class.

**Counterexamples.**
- Senior is charged while junior still has capacity.
- The reserve is charged beyond its balance. This must be a checked-subtraction `Reject` (`:124`), not a wrap.

**Placement.** The cell and index are U0. The library family is U6 ("redemption/loss allocation", `ROADMAP.md:48`). Tranche share conversion is U4 (Φ₁).

### Edit 5 — Keeper incentive as a funded effect, arrival as a named assumption

**Rule sketch.**
- The liquidation bonus comes only from debtor collateral (Edit 1).
- A flat or proportional keeper fee (Maker `coin = tip + wmul(tab, chip)`, `clip.sol`) is a separate transfer from a named reserve custody. It is charged to an affine pool-policy budget and counted in gross and fees under E1 (`tex:171-176`).
- A lending intent or policy that claims "no bad debt" must name:
  - keeper arrival latency `L_k`;
  - oracle staleness `Δ`;
  - maximum price move per `L_k+Δ` versus the minimum bonus margin.

  Otherwise recovery viability (`DESIGN-MIL2.md:219`) rejects the claim. A policy that names no such bound may only claim local safety: debt persists and the deficit is recorded.

**Counterexamples.**
- A fee is paid from the reserve with no budget, so the reserve drains through repeated micro-liquidations.
- A policy asserts solvency while relying on an unbounded keeper (R2 H6/H7, `R2-lending.md:479-485`).

**Auctions.** A Dutch-auction mode is a multi-stage escrow: a kick stage creates a pending continuation, and take stages are partial fills. It can use literal time-step price floors (`after(t_k) ⇒ ask ≥ lit_k`), with reset guarded by `tail`/`cusp` literals (`clip.sol`).

**Placement.** The fee effect is U3. The auction mode is U3 lifecycle plus a U6 library. The latency claim check belongs to O5 (`tex:251`).

## 3. Core versus library

**Core (U0 reservation, U3 semantics):**
- encumbrance reserve, commit, release, seize and cancel transitions (`tex:229-237`);
- obligation status and recourse fields;
- L1 with forgiveness coupled to deficit recognition;
- `deficit` and `trancheAssets` cells;
- digest-bound `LiquidationPolicy` inside consent;
- a seizure effect class distinct from gross (R2 §4);
- head-relative close-factor semantics and seize order;
- the leftover-guard form;
- the latency-assumption field in recovery viability.

**Library (U6):**
- band tables, close-factor and bonus values;
- Dutch-auction curves;
- tip/chip schedules;
- waterfall orderings;
- staked backstops and slashing;
- governance-dilution recapitalisation;
- soft liquidation.

## 4. Smallest implementable slice and evidence pair

**Slice.** One fixed-rate obligation (USDC, accrued 0) and one ETH encumbrance. One anchored, fresh price observation. One band. Fixed-bonus partial liquidation with cf 50%, the leftover rule and AccrualFirst discharge. No auction, deficit or waterfall. Units follow literal elaboration (`DESIGN-MIL2.md:78`), and price orientation is fixed by the U0 numeric profile.

**Positive control:**
- **Starting state:** outstanding 1,000 USDC; lock 1.5 ETH; observed price 1,200 USDC/ETH, fresh and anchored.
- **Band:** `p₁=1,250`, `m₁=20,000` bps, `b₁=500` bps, `cf=5,000`, `minLeftover=100`.
- **Unhealthy check:** `1.5·1,250·10⁴ = 18.75M < 1,000·20,000 = 20M` ✓.
- **Close factor:** `r=500`, and `500·10⁴ ≤ 1,000·5,000` ✓.
- **Seize cap:** `q=0.42 ETH`, and `0.42·1,250·10⁴ = 5.25M ≤ 500·10,500 = 5.25M` ✓.
- **Resulting effects:** keeper pays 500 USDC to the creditor and receives 0.42 ETH; outstanding becomes 500 (principal), which is ≥ 100; lock becomes 1.08 ETH; E1 holds per asset and L1 holds with discharge 500.

**Hostile control:** the same signed, fresh, well-formed stage with `q=0.43 ETH`. The seize-cap check gives `5.375M > 5.25M`. It must reject at that clause only, with native acceptance failing. This is not an envelope error (`ROADMAP.md:40`).

**Later hostile variants:**
- discharge recorded as 525;
- close factor checked against `post`;
- imported price substituted;
- two same-head keepers.

## 5. Explicit disagreements

1. The category report's priority 1 asks for "one fresh-price threshold" with "one fixed-bonus partial seizure" (`02-lending.md:38`). It does not say that proportional valuation is Φ₁. Without bands (Edit 1), that slice cannot be stated in Φ₀.
2. The PDF's `authorizedForgiveness_o` (`tex:183`) is a free term. It must be coupled to a recognized deficit and a recourse term, or it becomes a laundering channel for erased debt.
3. `enforce` "bounded by a policy the debtor consented to" (`DESIGN-MIL2.md:193`) points to `ConsentRef`. The policy parameters must be digest-bound fields, not an external reference.
4. I do not treat backstops as core, and I do not add a ninth authority right. Both would expand the U0 freeze without need.
5. MIL/1's "bad debt/backstop expressible" is wrong, as the report already notes (`02-lending.md:34`). I agree, and I add that `rank` alone cannot order a waterfall without tranche asset cells.

## 6. Residual assumptions

- Oracle honesty, anchored-read authentication and keeper arrival stay named premises. Safety does not supply liveness (`MORIARTY-PRODUCT-CONTRACT.md:47`).
- Band discretization assumes the policy author sizes `b_i` above band width. The authoring lint can check this; the circuit cannot.
- The 8-band and 4-class caps are unmeasured proposals (`DESIGN-MIL2.md:270`).
- Midnight phase semantics for a failed liquidation (retained fees) are unmodelled here.
- Cross-domain collateral liquidation needs non-receipt evidence (`tex:202`) and stays open.
- The external protocol behaviour I cite is from current official sources and was not tested.