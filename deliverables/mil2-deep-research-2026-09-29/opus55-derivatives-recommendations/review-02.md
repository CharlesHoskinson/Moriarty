# Derivatives: margin and funding recommendation (independent, specified-only)

**Lens.** Portfolio valuation, the maintenance threshold, a periodic ratchet, cross-margin and collateral reservation. **Status.** This is read-only design review. Nothing here is implemented, proved or accepted by a ledger. Startup was done: I read the develop skill and ran `status --json`. The status shows a blocked SP01.6 dispatch, which does not affect this review.

## 1. Verdict

**Not fit yet for margin and funding. The carriers are good ingredients.** MIL/2 already has:
- signed `Position<I>`, kept separate from `Delta<A>` (DESIGN-MIL2.md:65–76);
- encumbrances whose `against` field is a set (lines 93–100);
- a lock-sum obligation (line 102; semantics (K1), MIL2-PROPOSED-SEMANTICS.tex:229–237);
- `pre`/`post` state reads, motivated in part by funding (line 154);
- clock-indexed time (lines 69–73).

What is missing:
- **No margin cells.** The cell vocabulary (lines 239–243) has no margin-scope or funding-index cell.
- **No rule keeping unsettled P&L out of `balance`.** The product requires one (REPORT-RECONCILIATION-2026-09-07.md:64).
- **No reservation partition.** Nothing stops one encumbrance being counted in two margin scopes.
- **No rounding roles for valuation.** The numeric profile's default policy covers only obligations, receipts and exact results (`numeric-profile.json:50–55`). Its reserve for rounding remainders is `absent` (lines 56–58).

**Repository observation.** `pos × mark` is a variable product when the position is read from state, so it is Φ₁ and deferred to U4 (DESIGN-MIL2.md:158–165).

**Inference.** There is a Φ₀ route. If the signed template fixes an isolated position's size as a literal, every margin inequality becomes a literal-coefficient cross-multiplication. Those stay in Φ₀ (line 160) and need no rounding at all.

## 2. Five ranked design edits (recommendations)

### E1. Unsettled P&L is never a balance; funding owed is an obligation

**Rule sketch.**
- A margin account `M` is the tuple `{scope, positions: Position<I>[≤n], entry(I), fundIdxSeen(I), encs: {EncId}}`.
- Unrealized P&L is a derived term. No `Cell` stores it, and it has no `Delta`.
- Only a `SETTLE(M,I)` stage turns P&L into money. It emits balance deltas between `M`'s custody account and a counterparty or clearing custody account, and it is checked by (E1) (tex:171–177).
- Funding owed is an `Obligation` created under consent (DESIGN-MIL2.md:100) and rolled forward under (L1) (tex:178–187). It is never a negative balance. This keeps debt separate from supply.

**Counterexample.** Alice holds 1,000 USDC of collateral and has +500 unrealized. If a withdrawal guard reads `collateral + upnl`, she can take out 1,500. The rule rejects this because spendable = `balance − Σlocks`, and upnl is not in `balance`.

**Placement.**
- **U0:** reserve the `margin(acct, scope)` cell tag and a `settle` effect kind.
- **U3:** the transitions.
- **U6:** the formulas.

**Why this changes U0.** The cell vocabulary is frozen at U0, and unknown tags are rejected (lines 285, 344–345). Adding the tag later means a version migration, which is the same argument as owner decision 4 (line 336).

### E2. Collateral reservation is a linear partition across scopes

**Rule sketch.** Add `scopeOf : EncId ⇀ MarginScope`, a partial function. An encumbrance may back at most one scope.
- An **isolated** scope owns its own encumbrances.
- A **cross** scope is one scope holding many positions, backed by shared encumbrances.
- (K1) keeps its form. `lock_s(x)` now also counts margin reservations and pending withdrawals. A "move collateral between scopes" transition must rewrite `scopeOf` and pass both scopes' tests in the post-state.

**Counterexample.** One 1,000 USDC encumbrance is named in both an isolated ETH scope and a cross BTC scope. Each scope passes its own test, and (K1) still holds because the lock is counted once. Double pledging is undetected. The `against`-set design (line 100) allows this today.

**Placement.** U0 for the partition invariant as an extension of the K1 obligation (tex:249, O3). U3 for the transitions.

### E3. Maintenance and initial tests with role-directed rounding, fail-closed

**Rule sketch.**
- `Eq = collateral_floor(haircut) + Σ pnl(I) − fundingDebt`
- `Req = Σ ceil(|pos_I| · mark_I · mmr_I)`

Rounding roles to add to the numeric profile:
- `collateralCredit = floor`
- `unrealizedGain = floor`
- `unrealizedLoss = ceil`
- `requirement = ceil`
- `fundingPayer = ceil`, `fundingReceiver = floor`, remainder to `protocol-reserve`

Admission rules:
- A withdrawal needs `post: Eq ≥ IM`.
- Liquidation eligibility needs `pre: Eq < MM`.
- Each instrument uses **one mark observation identity per stage**. That same mark serves both sides and every scope term (tex:67 keeps identity in provenance).
- The mark must be `anchored@d` and `fresh`.
- An empty scope, or a missing or stale mark, gives `Reject`, never `true`.
- When the size is a literal (see §4), write the test in cross-multiplied form so no division occurs.

**Counterexample.** Rounding each position's requirement down lets someone split one position into many dust accounts, each with `Req = 0`. Summing floors across a cross scope under-margins it by up to `n−1` units. Making requirements ceil per term closes this. A second exploit: a guard compares two different marks for the same instrument, one for P&L and one for the requirement, and picks the favourable one.

**Placement.**
- **U0:** the rounding roles in the numeric profile. ROADMAP.md:21 already requires per-primitive rounding direction.
- **U2/U3:** the literal-size Φ₀ form.
- **U4:** the variable-size form. It needs Φ₁ and a U1 certificate for `mulDivRole`.

**Inference.** If price mantissas are declared ≤ `u64`, then `u128 × u64` fits in 192 bits, below the 253-bit `less_than` limit (line 169). No two-limb gadget is then needed. MIL/2 does not state a mantissa width; it should.

### E4. Funding as a period-indexed ratchet with a lazy cumulative index

**Rule sketch.** Add a cell `fundingIndex(I) = {n, cum: signed, markId_n}`. The transition `FUND(I, n)` requires all of:
- `n = pre.n + 1` (no skipping) and a `replay(I,n)` mark (no replay);
- `after(t0 + n·Δ)`;
- the mark or index observation has `observedAt ∈ window_n`;
- `rate = clamp(lit)`;
- `cum' = cum + rate`.

Each account then settles lazily: `pay = pos · (cum − fundIdxSeen)`, rounded by the payer/receiver roles, with the remainder to the reserve. The payer side becomes an obligation under E1. Lazy settlement matters because it avoids writing every account per period, which the 32-cell footprint cap (line 280) forbids.

**Counterexample.**
- A keeper skips period `n` when its rate hurts them. The `n = pre.n + 1` check rejects this.
- A late `FUND(n)` uses today's mark instead of the period-`n` mark. The window check rejects it. This also follows TP05: a late call is not evidence that the period was never charged.

The existing accrual ratchet is unilateral and has a fixed rate (R4:282). It is the right template for the counter, but not for the bilateral, signed index.

**External practice (not a Moriarty rule).** dYdX v4 charges funding hourly; "when the price is too high, longs pay shorts" (https://docs.dydx.xyz/concepts/trading/funding).

**Placement.** U0: the reserved cell tag. U3: the time-gated continuation with its counter. U4: variable `pos`. U6: the rate formula.

### E5. Liquidation keeps residual debt; `enforce` is bounded by the maintenance predicate

**Rule sketch.**
- `LIQ(M)` is authorized by `enforce` (line 193) only when `pre: Eq < MM`, the mark is anchored and fresh, and the close size ≤ a signed fraction.
- Proceeds pay obligations in `ClaimClass` order.
- Any deficit becomes an `Obligation` against `M` (L1) until a later insurance or social-loss stage discharges it under signed authority. It is never erased (REPORT-RECONCILIATION:61).

**Race.** A top-up and a liquidation land at the same head. Ledger order decides which applies first. The second stage is re-checked against the new `pre`, so the loser simply fails. No `priority` field is needed. This differs from escrow, where both guards may hold at once.

**Counterexample.** A liquidation closes the position with a deficit of −40 and sets the account to zero. Without the deficit obligation, (E1) still holds but debt vanishes.

**Placement.** U3: the residual-debt transition. U6: the waterfall, ADL and insurance.

## 3. Core versus library (recommendation)

**Core:**
- the margin and funding-index cell tags;
- the E1 non-spendability rule;
- the E2 scope partition inside (K1);
- the rounding-role enum and a reserve cell;
- one mark identity per instrument per stage;
- the generic time-gated continuation with a no-skip, no-replay counter;
- residual debt on `enforce`.

**Library (U6):**
- funding-rate formulas and clamps;
- IM/MM fractions and haircuts;
- liquidation incentives and close fractions;
- ADL ranking, insurance and social-loss policy;
- portfolio scenario grids.

A scenario grid (worst of k scenarios × n positions) needs `k·n` nonlinear nodes, far beyond the Φ₁ caps. It belongs to a later profile with a declared scenario cap. Keep `equity` out of Φ as an atom (line 156). It should be a library-derived term with declared roles.

## 4. Smallest slice and evidence pair

**Slice (U3, Φ₀, single domain, one signer).** One isolated scope. One instrument, with the position size fixed as a signed-template literal `+1` ETH-perp and entry `2000`. Collateral is in the quote asset (USDC, 6 decimals), so no conversion is needed. No funding. One transition, `WITHDRAW(w)`:
- guard: `10·post(coll) + 10·(mark − 2000) ≥ 1·mark`, which is IM = 10% in cross-multiplied form;
- `mark : Price ! {anchored@d}` with `fresh(mark, 5m)`;
- effects: shrink the encumbrance by `w`, and one transfer custody→owner of `w`;
- the footprint declares both balance cells, the encumbrance, the margin cell and the observation.

**Positive.** Collateral is 1,000 USDC and the anchored mark is 1950, so upnl = −50. For `w = 700`: `10·300 + 10·(−50) = 2500 ≥ 1950`, so the stage accepts. Custody −700 and owner +700 (Σ = 0 = Δsupply). The encumbrance goes 1000 → 300, and (K1) holds.

**Hostile (a single mutation).** Same witness, but the anchored fresh mark is 1700: `3000 − 3000 = 0 < 1700`, so the stage must reject. The rejection comes from the semantic guard, not from a malformed envelope (ROADMAP.md:40).

**Suggested extra control.** Mark 1950 supplied as `imported`. This must reject by provenance.

## 5. Explicit disagreements

1. **With 04-derivatives.md:18.** It says Φ₀ covers only "one bounded scalar threshold". With a literal position size, a variable-mark isolated margin test is fully Φ₀ and needs no rounding.
2. **With 04-derivatives.md:52's order (option first, margin later).** The report sets no mandatory order between them. REPORT-RECONCILIATION:64 is a mandatory challenge row, and the option slice never exercises cross-scope (K1). I recommend the §4 slice run alongside the option slice, or before it.
3. **With R4:628–629.** R4 says generalising the accrual ratchet makes funding "library over an existing form". The counter is generic, but the signed, bilateral cumulative index and its conservation are core.
4. **With D2's phrase "exactly one settlement for each period" (04:16).** Exactly-once should apply to the index update. Settlement per account is lazy and may cover many periods at once.
5. **Boundaries.** I **do not** move Φ₁ to U0. I **do** add to U0 the reserved cell tags, the rounding-role enum and the reserve cell. The reason is that U0 freezes the vocabulary and rejects unknown tags.

## 6. Residual assumptions

- The mark or index oracle is honest. This remains a named trust premise (TP03).
- Keepers are live. `fresh` bounds a mark's age, not whether any keeper arrives (DESIGN-MIL2.md:219).
- Funding sums to zero only for matched long/short open interest. A pool-counterparty perp needs a pool custody term in (E1).
- The price mantissa width (≤ `u64`) is my assumption, not a MIL/2 rule.
- The reserve mechanism is absent today (`numeric-profile.json:56–58`). Every floor/ceil remainder in E3 and E4 is unposted until it exists.
- The caps (8 positions per scope, 32 footprint cells) are unmeasured proposals (DESIGN-MIL2.md:270).
- Cross-domain collateral and imported marks remain U4. Global cross-domain rollback is not promised, and the federated kernel stays outside the acceptance relation (tex:59).