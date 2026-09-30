# Recommendation: Derivatives, from a compiler and native-feasibility view

*Specified-only design review. Nothing here claims implementation, proof or ledger acceptance. Every design item below is a **recommendation**.*

## 1. Verdict

**The design is partly fit.** MIL/2 has the right carriers for bounded derivatives:
- signed `Position<I>`, kept apart from `Delta<A>` (`DESIGN-MIL2.md:65-76`);
- clock-indexed time (`:71-73`);
- evidence types with source sets (`:173-187`);
- escrow as a transition relation with tombstones (`:195-219`);
- checked `u128` values and the limb rule (`:167-171`).

It cannot compile even the smallest cash-settled option today. Four decisive gaps:

1. **No price arithmetic in Φ₀.** Φ₀ allows only `term × lit` (`:136`). It has no division and no formation rule for multiplying a `Price` by a `Qty`. A payoff needs both.
2. **Checked subtraction breaks out-of-the-money settlement.** It rejects on underflow (`:124`), so writing `S − K` for a fixing below the strike yields `Reject`. The settlement branch is then never admissible and the collateral is stuck.
3. **No fixing rule.** `Instrument` is "opaque identifier" metadata (`:58`), there is no fixing cell, and `exercised(opt)` has no producer (`04-derivatives.md:17`).
4. **An uncapped cash-settled call cannot be fully collateralised in the quote asset.** Paying out in the underlying needs `p·S ≤ N·(S−K)`, which multiplies two variables and so is Φ₁ (`:160-161`). The report's "fully collateralized … European call" (`04-derivatives.md:52`) therefore needs a cap, i.e. a call spread.

The existing source/5 Core already has most of the missing piece. `Amount × Price` produces `ScaledAmount` (`experiments/moriarty-language/src/successor/financial-expression-v1.ts:341`). `FloorDiv`/`CeilDiv` accept only a literal `10^scale` divisor (`:346-348`). MIL/2 should adopt this, not reinvent it.

## 2. Five ranked design edits

### E1. Dimensioned scaled-price arithmetic in Φ₀, with role-directed rescaling

**Rule sketch:**
```
Γ ⊢ n : Qty<Q> lit    Γ ⊢ t : Price<B,Q,s,w> ! S    bits(n)+w ≤ 252
──────────────────────────────────────────────────────────────
Γ ⊢ n ⊗ t : Scaled<B,s> ! S

Γ ⊢ x : Scaled<B,s> ! S    ρ ∈ {floor, ceil}   (ρ fixed by result role)
──────────────────────────────────────────────────────────────
Γ ⊢ rescale_ρ(x) : Qty<B> ! S
```
- Add a declared mantissa width `w` to `Price` (the slice uses `u64`).
- Lower `rescale_floor(x) = p` as a witness `p` plus the bracket `p·10^s ≤ x < (p+1)·10^s`. Every multiplier is a literal, so the check stays solver-free linear arithmetic. It does not need Φ₁.
- If the static bound fails, use the two-limb gadget or reject (`:169-171`).
- ρ comes from the result's role in the numeric profile (a receipt rounds down, an obligation rounds up). The author cannot choose it; the profile already records author-selectable rounding as a gap (`numeric-profile.json`, `accrual-interest`).

**Counterexample.** Without the lower half of the bracket, the payout is a hole bounded only above. A keeper aligned with the writer settles `p = 0` and still meets every bound. That violates acceptance refinement C1 (`MIL2-PROPOSED-SEMANTICS.tex:122-127`).

**Placement.** U0: grammar, formation rule and numeric-profile rows. U1: a native certificate for the bracket primitive (`ROADMAP.md:22`).

**U0 boundary change, and why.** This widens Φ₀ by one operator with a literal power-of-ten divisor. It stays out of Φ₁ because the divisor is never a variable and no variable-by-variable product appears.

### E2. A total positive-part operator, and a totality lint on settlement guards

**Rule sketch:** `pos(a, b) ≜ max(a, b) − b`. This never underflows and is linear in Φ₀. Any `−` in a terminal-branch guard or effect amount must either have that shape or carry a dominance premise (`b ≤ a`) in the same branch guard. Otherwise authoring rejects with `unsupported`.

**Counterexample.** `payout = N ⊗ (S − K)` with S = 2,900 and K = 3,000. Evaluation gives `Reject(underflow)`. The settlement stage fails closed forever, and only the recovery branch remains, which strands the holder. Failing closed on `Reject` is correct for safety (`:124`), which is why the author must not be able to write this.

**Placement.** U0 (Φ₀ totality rules, `DESIGN-MIL2.md:344-346`). It shares an execution trace with obligation O1.

### E3. A write-once fixing cell with deterministic round selection

**Rule sketch.** Add `fixing(I)` to the cell vocabulary (`:239-242`), or generalise to `observation(feedId, round)`. The FIX transition:
```
pre(fixing(I)) = ⊥   o_r, o_{r+1} : Obs<Price,anchored,d>, both authenticated reads of feed(I)
o_r.observedAt ≤ T_fix(I) < o_{r+1}.observedAt
────────────────────────────────────────────────
post(fixing(I)) = o_r.value, with round id r bound in the public statement
```
If the ledger keeps only the current feed value, fall back to a snapshot stage within `[T_fix, T_fix+g]`. Declare the keeper's timing choice as a named premise.

**Counterexample.** With only `fresh(obs, 5m)` and `after(T_fix)`, the keeper can pick whichever of five in-window rounds suits them. A second settlement could also read a later value. Both break the report's "17:00 fixing, not a later value" requirement (`04-derivatives.md:17`).

**External practice, not a Moriarty rule.** The ACTUS dictionary separates `exerciseDate` ("Date of exercising a contingent event/obligation") and `exerciseAmount` ("fixed at Exercise Date") from `settlementPeriod` ("from fixing … to settlement"): https://raw.githubusercontent.com/actusfrf/actus-dictionary/master/actus-dictionary-terms.json. This supports fixing first and settling later.

**Placement.** U0 for the cell kind. U2/U3 for the transition. Imported fixings stay U4 (`DESIGN-MIL2.md:350`).

### E4. Instrument terms become hash-bound, readable policy, plus a core position-conservation law

**Rule sketch.**
- `Instrument` becomes `{feed, quoteAsset, N, K, Kcap, s, T_fix, g, settleBy, fallback}`. It sits inside the canonical intent digest and is readable through `policy(I)`, not as opaque metadata (`:58`).
- New core law, alongside E1 conservation (`:256`): for each (d, I), `Σ_acct Δposition(acct, I) = 0` in every stage.
- States: `offered → open → fixed → settled | fallback`, each terminal state tombstoned (reusing `:199-216`).
- The cash-settled European is **automatically exercised**. The payout formula (from E1 and E2) is zero out of the money, so the slice needs no `exercised(opt)` producer.

**Counterexample.** The open stage writes `position(buyer, I) = +N` with no matching `−N` for the writer. That mints unbacked exposure while E1 balance conservation still holds, because positions are not balances.

**Placement.** Position conservation is U0 core. The option template is U3/U6 library.

### E5. Name the remainder beneficiary per primitive, allowing a residual party

**Rule sketch.** Payout `p = rescale_floor(...)`, and the writer's residual is `C − p`, computed exactly. Allow `remainderBeneficiary: residual-party(writer)` alongside `protocol-reserve`.

**Why this changes U0.** The numeric profile defaults the remainder to `protocol-reserve` but records the reserve as `"status": "absent"`. As a result, every floor or ceil primitive there is an "open conformance gap" (`numeric-profile.json`, `reserveMechanism`). A bilateral contract has a natural owner for the remainder, and exact complement arithmetic closes the gap without a reserve account.

**Counterexample.** Rounding up for the holder with a reserve default and no reserve account makes `p + r = C + 1` or loses a unit. Either breaks E1 conservation for escrow custody.

**Placement.** U0 numeric profile.

## 3. Core versus library

**Core (language and native):**
- the `Price` width index;
- the scaled product `⊗` and role-directed `rescale_ρ`, lowered as a bracket;
- `pos`;
- the write-once fixing cell and authenticated round binding;
- the position-conservation law;
- per-primitive remainder beneficiary;
- the existing escrow tombstone.

These carry arithmetic soundness, provenance or linearity that a library cannot restore.

**Library (U6, `ROADMAP.md:27, 48`):**
- option, call-spread, put and digital templates;
- exercise styles (American and Bermudan need an exercise cell and producer);
- fallback policies;
- funding schedules, margin, liquidation waterfalls, ADL;
- tranches;
- listing governance.

## 4. Smallest implementable slice and one evidence pair

**Slice:** single domain; quote asset USDC (6 decimals); underlying feed ETH/USDC; fully collateralised, cash-settled, capped European call (a call spread); automatic exercise; anchored feed with round history; holder bound once and not transferable (this avoids the D9/D15 claim minting). Each stage has one signer, respecting `|S| = 1` (`DESIGN-MIL2.md:336`):

1. **Writer signs:** funds escrow `C`, publishes the offer (`complete by any`).
2. **Buyer signs:** premium to writer, `holder := buyer`, positions `±N` — one atomic stage.
3. **Any keeper (`complete` right):** FIX, then settle with payout to the holder, residual to the writer, positions zeroed and tombstone written. FIX and settle may be one stage when `T_fix` has passed.

**Parameters:** `Price<USDC,ETH,18,u64>`, measured per smallest unit.
- `N` = 1.5 ETH = 1.5×10¹⁸ wei
- `K` = 3,000,000,000 (3,000 USDC/ETH)
- `Kcap` = 3,500,000,000
- `C` = `rescale_ceil(N ⊗ (Kcap − K))` = 750,000,000 µUSDC

**Positive control.** Fixing S = 3,123,456,789.
- `x = N ⊗ pos(min(S, Kcap), K)` = 1.5×10¹⁸ × 123,456,789 = 185,185,183.5×10¹⁸.
- `p` = 185,185,183, and the bracket holds: 185,185,183×10¹⁸ ≤ x < 185,185,184×10¹⁸.
- Writer residual = 564,814,817.
- Conservation: 185,185,183 + 564,814,817 = 750,000,000.
- `x < 2^88`, so the bracket check fits in native `less_than`.

**Hostile control.** Same signed intent, same fixing round, same footprint. Only the payout changes: `p` = 185,185,182, residual = 564,814,818.
- This still conserves (the sum is 750,000,000), is correctly signed, and uses a valid envelope.
- Only the upper half of the bracket rejects it: x ≥ 185,185,183×10¹⁸.
- So the rejection comes from the rounding constraint itself, not from a malformed envelope (`ROADMAP.md:40`).

**Additional hostile controls:**
- `p` = 185,185,184 (rounds up for the holder);
- a round `r+1` fixing;
- a second settlement after the tombstone;
- a wrong recipient;
- an out-of-the-money fixing written with raw `S − K` (must be rejected at authoring);
- an unpaid premium in stage 2.

## 5. Explicit disagreements

1. **With `04-derivatives.md:52`:** the first slice should drop the explicit exercise cell. Automatic exercise removes the `exercised(opt)` producer problem. That cell belongs to American or physical-settlement slices.
2. **Same report:** "fully collateralized cash-settled call" needs a cap. Otherwise the payout must be in the underlying, which is Φ₁ (`DESIGN-MIL2.md:160-161`).
3. **With `DESIGN-MIL2.md:136, 160`:** "no division in Φ₀" is too strict. Literal-divisor rescaling is linear, is already a source/5 Core operator (`financial-expression-v1.ts:346-348`), and derivatives cannot be written without it. Conversely, `term × lit` is too loose: it lacks dimension typing.
4. **With `DESIGN-MIL2.md:58`:** opaque `Instrument` metadata cannot drive guards. Its terms must be hash-bound and readable.
5. **With the numeric-profile default:** a protocol reserve that is declared absent should not be the only remainder beneficiary.

## 6. Residual assumptions

- The feed is honest, and its round history is stored on the ledger and authenticated. If only a current value exists, keeper timing becomes a named premise.
- The ZKIR v3 `less_than` bound of ≤253 bits and the width cap `bits(N)+w ≤ 252` hold on the pinned target (still unmeasured, `DESIGN-MIL2.md:270`).
- A writer's prior `complete by any` authority composes across a ledger-linked Episode without multi-signer stages.
- A keeper arrives, and fallback-after-`settleBy` is a signed policy, not a deadline entitlement (`:219`).
- Midnight phase-failure fee retention is modelled separately (`MORIARTY-PRODUCT-CONTRACT.md:47`).
- The ACTUS citation is comparative external practice only.