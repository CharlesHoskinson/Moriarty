# Derivatives review, liquidation and loss lens: independent Opus 5.5 recommendation

**Status.** Specified-only design review, 2026-09-29. Nothing here is implemented, proved or accepted on the ledger. Guarded CLI status was read first; its blocked SP01.6 dispatch does not affect this read-only review. I did not read any other reviewer's output.

## 1. Verdict on design fitness

**Repository observation.** MIL/2 has some of the pieces liquidation needs:
- ranked obligations and prioritized encumbrances (`concepts/intent-language/DESIGN-MIL2.md:91-102`);
- a debtor-consented `enforce` right (`DESIGN-MIL2.md:191-193`);
- a reserve account inside conservation (`MIL2-PROPOSED-SEMANTICS.tex:171-177`);
- liability roll with forgiveness kept separate from repayment (`MIL2-PROPOSED-SEMANTICS.tex:178-187`);
- one-shot tombstones (`DESIGN-MIL2.md:216`).

It has no eligibility rule, close-size rule, bankruptcy price, insurance draw, ADL or social-loss transition (`category-review/04-derivatives.md:20`).

**Inference.** The category is **not yet fit** for liquidation. The rules it lacks would change the U0 frozen vocabulary and the numeric profile. They are not just library code. The bad-debt row ("reject erased residual debt or missing socialized loss", `openspec/REPORT-RECONCILIATION-2026-09-07.md:61`) has nothing to check against. One unstated fact matters here: any health test that multiplies a variable mark by a variable position size is a variable×variable product, so it is Φ₁ and deferred to U4 (`DESIGN-MIL2.md:158-165`). The smallest slice therefore has to fix sizes as signed literals.

## 2. Five ranked design edits (recommendations)

### E1. Loss waterfall as one ordered, exhaustive, non-erasing allocation (core)

**Rule sketch.** For a closeout with shortfall `D : Qty<Q>` (settlement asset Q):

```
D = draw_ins + haircut_total + residual_debt          -- all ≥ 0, checked u128
tranche order τ = signed policy list, e.g. [insurance, socialize, retain_debt]
∀ i<j in τ:  amount(τ_j) > 0  ⇒  tranche τ_i exhausted
             (insurance: draw_ins = pre(balance(d, INS, Q)) − floor_ins)
residual_debt > 0  ⇒  obligation'(debtor=acct, creditor=pool, rank=τ.class) created   (L1)
authorizedForgiveness_o = haircut_total      -- forgiveness only against a recorded haircut
```

- **Counterexample.** D = 10, insurance holds 4. A witness draws 4, writes no haircut and no obligation, and tombstones the account. Every local check passes except L1, and 6 of loss disappears. The rule rejects this because `Σ = 4 ≠ 10`.
- **Why core.** Tranche order is policy. Exhaustion and non-erasure are acceptance clauses (`LiabilityRoll ∧ Conserve`, `MIL2-PROPOSED-SEMANTICS.tex:131-138`). Leaving them to a library would let a lowerer omit them.
- **Placement.** Clause shape at U0 (effect/history keys). Enforcement at U2/U3. The library policy is U6.

### E2. Account-scoped seize order and liquidation replay key (core; widens a U0 boundary)

**Rule sketch.** Replace "signed total order on seizes of one encumbrance id" (`DESIGN-MIL2.md:193`) with an order over the seize set of one **margin scope**:

```
liqKey = H(acct, scope, markObsId)            -- replay(liqKey) consumed once
seize sequence ⟨enc_1..enc_k⟩ sorted by (priority, id); k ≤ cap
pre(state(acct,scope)) ∈ {liquidatable}  ∧  ¬tombstone(acct,scope,epoch)
```

- **Counterexample.** Cross-margin collateral sits in encumbrances e1 (priority 1) and e2 (priority 2). Two keepers act on the same mark. One seizes e2 first. The other seizes e1 in a separate stage. Each stage is valid under the per-encumbrance order, but together they seize past the shortfall and the result depends on order.
- **Why change U0.** The ordering unit becomes part of the stage public-input schema and the authority enum (`DESIGN-MIL2.md:345`). A per-encumbrance order cannot be widened after the digest freezes.
- **Placement.** U0 (schema field), U3 (race behaviour, per the U3 late-race row, `ROADMAP.md:24`).

### E3. Keeper right as bounded delegated `enforce`, with reward inside cumulative fees (core carrier, library parameters)

**Rule sketch.**

```
authority enforce by any                       -- permissionless keeper
  scope    (acct, scope, instrument)
  guard    liquidatable(markObs) ∧ fresh(markObs, Δ) ∧ mark ! {anchored@d}
  budget   affine: closeQty ≤ closeFactor_lit × |pre(position)|   -- Φ₀: literal coefficient
  reward   ≤ reward_lit, paid from tranche named in policy, counted in cumulativeFees
```

The keeper chooses **when** to act. The consented policy fixes **price source, size, reward and recipients**. The fill price is never a keeper hole. Partial close must leave either a healthy post-state or a closed position. It must not leave a new position that is still liquidatable at the same mark. For the literal-size profile this is a Φ₀ check.

- **Counterexample.** A keeper with a stale or imported mark, or with the reward as a fillable hole, liquidates a healthy account and takes the penalty. The source-set rule (`DESIGN-MIL2.md:185`) and the fixed-reward bound reject it.
- **Placement.** U0 (the `enforce` scope field names position/scope). U3 (race against top-up). Keeper liveness stays a named assumption, not a guarantee (`DESIGN-MIL2.md:219`).

### E4. Bankruptcy and liquidation prices as role-directed roundings in the numeric profile (U0 profile entries, U1 certificates)

**Rule sketch.** For a long of literal size `q` with collateral `c`:

```
bk  = entry − c/q        rounding role: AGAINST_LIQUIDATED  (bk rounded up for long, down for short)
liq = bk + mm            mm literal;   same role
shortfall D = max(0, q × (bk − exitPrice))   rounding role: TOWARD_POOL (up)
haircut_i  rounding role: AGAINST_HAIRCUT_HOLDER (up); Σ haircut_i − D retained in insurance
penalty, keeper reward:  rounding role: DOWN (beneficiary receives ≤ exact)
```

All values are checked against witnesses by literal cross-multiplication, e.g. `bk × q ≤ entry × q − c`. This keeps them in Φ₀ while `q` is a signed literal.

- **Counterexample.** Rounding `bk` toward the account gains 1 smallest unit per liquidation. Repeated partial closes pull value out of the insurance reserve and no conservation check sees it.
- **Why U0.** The roadmap already requires "per-primitive rounding direction and beneficiary policy" in U0 (`ROADMAP.md:21`). These roles must be listed there.
- **Placement.** Variable-size `q × mark` stays Φ₁/U4. There is **no change** to the Φ₁ deferral.

### E5. Social loss as a lazily applied per-side loss index; ranked ADL deferred (core cell, library mechanism)

**Rule sketch.** Add one U0 cell kind: `index(instrument, side, kind)` with `kind ∈ {funding, loss}`. A socialize tranche writes `index' = index + ⌈haircut_total·S / openInterest_side⌉`. On its next touch, each position settles `Δindex × |pos|` as a checked debit. This makes the per-stage write set O(1).

Ranked ADL writes to k opposing accounts selected by a global ranking. That needs bounded collections (open item 6, `DESIGN-MIL2.md:360`) and consent on origination from every counterparty. It is post-U4 library work.

- **Why add a cell at U0.** The cell vocabulary is U0 (`DESIGN-MIL2.md:239-243, 345`). Funding needs the same cumulative index (D2).
- **Counterexample.** A pro-rata haircut written directly to N positions exceeds the footprint cap of 32 (`DESIGN-MIL2.md:280`). A stage that haircuts only the first 32 silently exempts the rest.
- **Placement.** Cell at U0. Mechanism at U6 (loss-allocation library, `ROADMAP.md:48`).

## 3. Core versus library boundary (recommendation)

**Core:**
- tranche exhaustion and D-equality;
- no erasure (forgiveness = recorded haircut);
- liquidation replay key and account-scope seize order;
- keeper cannot choose price, recipient or reward;
- rounding-role enum;
- `index` cell;
- debt stays an obligation, never negative supply (`docs/MORIARTY-PRODUCT-CONTRACT.md:43`).

**Library:**
- margin formulas;
- close factor, penalty and reward values;
- tranche order;
- insurance floor;
- ADL ranking key;
- orderbook versus backstop takeover;
- portfolio and stress-grid margin.

**External comparison, not Moriarty rules:**
- dYdX v4 sends liquidation profit and loss to an insurance fund. Governance configuration decides how much of a position is closed. The fill is bounded by a "fillable price" limit ([docs.dydx.xyz/concepts/trading/liquidations](https://docs.dydx.xyz/concepts/trading/liquidations)).
- Hyperliquid ranks ADL counterparties by `(mark/entry)·(notional/account_value)` and closes them "at the previous mark price" ([hyperliquid.gitbook.io/.../auto-deleveraging](https://hyperliquid.gitbook.io/hyperliquid-docs/trading/auto-deleveraging)).

Both treat ranking and parameters as venue policy, which supports placing them in the library.

## 4. Smallest implementable slice and evidence pair (recommendation)

**Slice.** Single domain, one signer (the keeper under a delegated `enforce`), isolated margin, one linear instrument, full close only, backstop takeover at the anchored mark. The signed literals are:
- size `q = 10`;
- entry 100 USDC;
- collateral 60;
- mm = 1, so `bk = 94` and `liq = 95`;
- insurance floor 0;
- keeper reward 1, paid from insurance **before** the shortfall draw (signed order);
- tranche order `[insurance, retain_debt]`.

Everything stays in Φ₀ through literal-coefficient cross-multiplication. Social loss is out of this slice.

**Positive control.** Anchored mark 93, fresh within Δ, insurance 5.
- Loss 70, collateral 60, D = 10.
- Keeper gets 1 from insurance. Insurance then has 4, and all 4 are drawn.
- A residual obligation of 6 is created: debtor = acct, creditor = pool, consented rank.
- Pool receives 60 + 4. Replay key is consumed. Position is tombstoned.
- USDC Σ balance deltas are 0, and the liability closes at 6.

**Hostile control.** The same valid witness, except the obligation is omitted (residual 0) and everything else is unchanged. This must reject at `LiabilityRoll` (4 ≠ 10), both in native verification and in the source/K differential. It must not reject for a malformed envelope (`ROADMAP.md:40`).

Secondary hostile cases:
- mark 96 (not liquidatable);
- imported mark;
- reward 2;
- second stage with the same `liqKey`;
- `bk` rounded to 93.

**Placement.** U2 single-stage path. The keeper-versus-top-up race is U3.

## 5. Explicit disagreements

1. **Waterfall order.** R4 lists "liquidation → ADL → insurance → socialized" (`u0-study-2026-09-28/defi-coverage/R4-derivatives.md:259`). The venues cited above use the insurance fund before ADL. I recommend the order be signed policy, with only exhaustion and non-erasure in core.
2. **Seize ordering.** `DESIGN-MIL2.md:193` scopes the total order to one encumbrance. That is too narrow for cross-margin (E2).
3. **"Liquidation-latency bound"** (R4:264, :439). The language can bound when enforcement is *authorized*, through `fresh` and windows. It cannot bound when a liquidation *occurs*. Latency belongs among the named liveness assumptions (`DESIGN-MIL2.md:219`; `docs/MORIARTY-PRODUCT-CONTRACT.md:47`).
4. **Φ₁ and liquidation.** Deferring Φ₁ is not what blocks liquidation. Literal-size isolated margin is Φ₀. Pulling Φ₁ into U0 is not justified by this category.
5. **Social loss in the first slice.** The category report expects social loss to appear. I would keep `retain_debt` as the only post-insurance tranche in the first slice. Debt is the conservative, non-erasing default.

## 6. Residual assumptions

- Mark honesty for anchored feeds, and every imported feed's policy.
- Keeper arrival and chain inclusion. No party is forced to submit.
- Enough market or backstop depth at the chosen exit price.
- Ledger ordering and MEV fairness between a keeper and a user top-up.
- A Midnight phase layout in which a failed fallible phase may still retain fees (`docs/MORIARTY-PRODUCT-CONTRACT.md:47`).
- Foreign collateral cannot be seized locally. Cross-domain shortfall stays debt, with no global rollback.
- The K1 lock-sum and L1 obligations remain unproved (`MIL2-PROPOSED-SEMANTICS.tex:229-237`).
- The optional federated kernel plays no part: any party can act as keeper.