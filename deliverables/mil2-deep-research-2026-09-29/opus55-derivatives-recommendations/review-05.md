# Derivatives recommendation brief: adversarial controls and release plan

**Status:** Read-only design review. Nothing here is implemented, proved or accepted by a ledger. `status --json` reports the blocked SP01.6 dispatch, and that block does not affect this review.

## 1. Verdict on category design fitness

**Not fit for a derivatives slice as written. It can be repaired with a few bounded additions.** MIL/2 has the right building blocks: signed `Position<I>` (DESIGN-MIL2.md:68), evidence indices (176–187), one-shot escrow tombstones (216), the signed seize order (193), and liability roll-forward (L1, MIL2-PROPOSED-SEMANTICS.tex:180–187). Every hostile case in my assigned lens still passes the current rules:

- **Stale mark or wrong fixing.** `fresh` only bounds age, with `observedAt ≤ stageTime` (DESIGN-MIL2.md:144, 187). A 16:59 print still counts as fresh at 17:03 for a 17:00 fixing.
- **Double exercise.** `exercised(opt)` has no producing effect and no cell (DESIGN-MIL2.md:149, 239–242).
- **Sign errors.** E1 counts custody and reserve accounts inside the sum (tex:171–177). A payout that credits *both* sides out of custody still conserves.
- **Unformable payoff.** Φ₀ has no typed Price→Qty conversion, and comparisons across sorts are type errors (DESIGN-MIL2.md:120, 136). So no cash-settled payoff or margin threshold can be written in Φ₀.
- **Positive part rejects.** The obvious `max(S − K, 0)` hits checked-subtraction underflow and returns `Reject` when S < K (tex:95). That blocks the writer's out-of-the-money exit.
- **Margin race.** No rule orders a top-up against a liquidation, and no rule stops a keeper from picking the most favourable of several fresh marks.

## 2. Five ranked design edits (recommendations)

### E1. Write-once fixing cell with a first-in-window rule (wrong fixing, stale mark)

*Rule sketch.* Add a `FixingPolicy { feed, issuer, Tf: Instant(c), w: Duration(c), finality }`. Add a cell `fixing(instrument, eventId)`, and add `obs.seq` to the evidence terms.

```
q_fix = unset   o ∈ Obs<Price,anchored,d>   o.feed = P.feed
Tf ≤ o.observedAt ≤ Tf + w   final(o)
pred(o).observedAt < Tf        -- authenticated predecessor on the feed sequence
────────────────────────────────────────────
fixing' = (o.id, o.value, o.seq);  tombstone'(fixing)
```

Settlement reads `fixing(...)` and never reads `obs` directly.

*Counterexample.* Two prints, 17:00:02 at 2150 and 17:00:40 at 2190, both lie in the window and both pass `fresh`. The holder picks 2190. The `pred(o)` clause forces the first print.

*Placement.* The cell, `obs.seq` and the policy sort go in **U0**. The cell vocabulary and evidence index are U0 items (DESIGN-MIL2.md:345), and adding them later changes the digest. The rule is enforced at **U3**. Imported fixings stay **U4** (351).

### E2. Exercise and redemption bound to a burned claim (double exercise)

*Rule sketch.* Add a cell `claim(instrument, holder)` and a one-shot `exercise(instrument, claimId)` mark:

```
before(Texp) ∧ n ≤ pre(claim) ∧ ¬consumed(exercise(claimId,epoch))
⇒ post(claim) = pre(claim) − n ∧ consumed' ∧ settlement duty recorded
```

The burn happens in the *same* stage as the exercise. Stage time is the authenticated inclusion time, not the time the holder signed.

*Counterexample.* A holder exercises, transfers the unburned claim, and the new holder exercises again. The replay id differs, so per-id replay alone does not stop it. The burn does.

*Placement.* The cells go in **U0**. The gate runs at **U2** for the auto-exercise form and **U3** for elective exercise that races the writer's reclaim, using the escrow `priority` field (DESIGN-MIL2.md:206).

### E3. Side-indexed unsigned legs, a zero-sum law per instrument, and a typed conversion with explicit rounding (sign errors)

*Rule sketch.*
- Replace the bare sign of `Position<I>` with `Position<I, side ∈ {long, short}>` over an unsigned magnitude. The direction of each payout comes from the side index, never from a sign bit.
- Add a Φ₀ conversion `notional(p: Price<B,Q,s>, lit N, round: toWriter|toHolder) : Qty<Q>`. The divisor is a literal power of ten and the rounding direction is chosen by role. Because the multiplier is a literal, this stays Φ₀. Variable contract size stays in Φ₁.
- Add a law for each instrument settlement event: `Σ_long credits = Σ_short debits`, with fees separate and custody draw ≤ that instrument's collateral subaccount.
- Admit the positive part only in the non-underflowing form `max(S, K) − K`.

*Counterexample.* A feed quotes funding with the opposite sign convention. Signed `rate × notional` makes the receiver pay. With side-indexed legs plus a declared feed convention, the mismatch is a type error. Crediting both sides from custody breaks the zero-sum law, even though E1 is satisfied.

*Placement.* This is a **U0** numeric-profile change. The roadmap already puts per-primitive rounding direction and beneficiary policy in U0 (ROADMAP.md:21). The polarity lint's exclusion of signed factors (DESIGN-MIL2.md:234) becomes moot for positions.

### E4. Margin-state mark ratchet and a signed top-up/liquidation priority (margin race)

*Rule sketch.* Add a `margin(acct, scope)` cell holding `{lastMarkSeq, lastChangeHead}`. A liquidation must:
- read `margin`, `balance` and `position` in its footprint, so any intervening top-up makes its predecessor head stale;
- use a mark with `seq ≥ lastMarkSeq` and pass `fresh(mark, Δ)`;
- decide eligibility from authenticated `pre(...)` state inside the stage, never from a Boolean computed by the host;
- write `lastMarkSeq'`.

Top-up versus liquidation at the same head is settled by a signed `priority` field (top-up wins until a liquidation is accepted), following the escrow model.

*Counterexample.* The price recovers at seq 42, and a keeper liquidates with a fresh-but-older seq 40 mark. Or a top-up lands, and the keeper's witness still references the balance before the top-up.

*Placement.* The rule is designed at **U3**. The margin model and scope (isolated, cross, portfolio) are **U6** library content. Portfolio stress grids need the bounded observation collections that remain open (DESIGN-MIL2.md:360).

### E5. Mandatory residual-shortfall liability (residual liabilities)

*Rule sketch.* A close or liquidation where `owed > seized` must emit `Obligation{debtor = trader, creditor = instrument loss class, rank, consent = origination enforce policy}`. L1 must hold, and the winner's unpaid claim stays open against that class. Only an `authorizedForgiveness` effect with a named authority, such as insurance or a social-loss allocation, can reduce it (tex:180–187). A stage whose closing liabilities omit the shortfall is rejected.

*Counterexample.* A liquidation zeroes the position, seizes all collateral, and pays the winner 80 of 100. The 20 disappears with no liability recorded. The current relation has no rule forcing it to be recorded; the product contract says debt is not supply and cannot vanish (MORIARTY-PRODUCT-CONTRACT.md:43, 45).

*Placement.* The rule sits in **U3** LiabilityRoll. Waterfall order and ADL are **U6** library content. For a fully collateralized slice, add a static check at issuance, `collateral ≥ notional(C − K, N)`, which makes a shortfall impossible by construction.

## 3. Core versus library boundary (recommendation)

**Core:**
- the fixing cell and first-in-window rule;
- claim cells and exercise marks with burn-on-exercise;
- side indices and the zero-sum law per instrument;
- `notional` with role-directed rounding;
- `obs.seq` and the mark ratchet;
- the mandatory residual liability.

These belong in Core because each one either changes the frozen vocabulary or digest, or is a safety law that no library may weaken.

**Library (U6, ROADMAP.md:27, 48):**
- payoff shapes (capped call, futures, spreads);
- funding formulas and schedules;
- margin models and scope;
- liquidation selection, waterfall order, ADL ranking, insurance;
- listing governance.

## 4. Smallest implementable slice and evidence pair (recommendation)

**Slice:** one domain, one signer, anchored fixing, a **capped, cash-settled, auto-exercised European call** with a literal contract size and no dynamic margin.

It has two stages:
1. **Open.** Premium goes to the writer. The writer puts collateral `notional(C − K, N)` into program custody, and the position cells for both sides are written.
2. **Settle.** Record the fixing (E1), pay `min(max(S, K) − K, C − K)` converted by `notional(·, N, toWriter)` to the holder, send the remainder to the writer, and write the tombstone.

This uses only Φ₀ plus E1–E3, with no elective-exercise race.

**Worked numbers.** USDC has 6 decimals and the price scale is s = 2. K = 200000 and C = 220000, so collateral is 200 USDC.

- **Positive control.** The first print in the window is at 17:00:02, S = 215000. The holder receives 150 USDC and the writer 50 USDC; custody changes by −200 and the zero-sum law holds.
- **Hostile control.** Identical except that the fixing witness is the 17:00:40 print at S = 219000, which is also final, fresh and anchored, but not first. It must reject with `FIXING_NOT_FIRST` at the native relation, not through a malformed envelope (ROADMAP.md:40).

**Secondary controls:**
- S = 195000 must *accept*, paying the holder 0 and the writer 200. This catches the underflow trap.
- Swapping the recipients must reject on the zero-sum and side laws.
- Resubmitting the settle stage must reject on the tombstone.
- A 16:59 print must reject with `FIXING_BEFORE_WINDOW`.

## 5. Explicit disagreements

1. **With 04-derivatives.md:52.** A "fully collateralized … European call" is not possible without a cap. Uncapped call payoff has no upper bound, so the slice must be a capped call or a call spread.
2. **Also with 04:52.** I would put elective exercise (early-exercise and double-exercise controls) in a *second* slice. Auto-exercise removes the inclusion-time race from the first release, and double settlement is still tested through the tombstone.
3. **With 04:18.** "Φ₀ literal arithmetic may express a hand-written threshold" is too strong. No typed Price→Qty formation rule exists (DESIGN-MIL2.md:120), so a mark-based threshold does not typecheck.
4. **With 04:23 (D9 partial) and DESIGN-MIL2.md:256.** General conservation is not a sign-error defence while custody sits inside the sum. A zero-sum law per instrument is needed.
5. **With DESIGN-MIL2.md:144.** `fresh` is the wrong predicate for a settlement fixing. Fixings need identity and first-in-window selection; freshness is only for marks.
6. **U0 boundary change, stated.** I add cells (`fixing`, `claim`, `exercise`, `margin`), `obs.seq`, a position side index and the `notional` primitive to U0. The footprint vocabulary, evidence index and numeric rounding profile are already U0 hash-bound items (DESIGN-MIL2.md:344–346; ROADMAP.md:21), so adding them after the freeze would break digests. Φ₁ stays deferred to U4. Nothing here needs variable×variable products.

## 6. Residual assumptions

- A Midnight-anchored feed exposes a monotone, authenticated sequence with provable predecessor timestamps. This is unverified, and without it E1 falls back to a named attester trust premise.
- Stage time is the authenticated ledger inclusion time.
- The ledger serializes stages that write the same cell by predecessor head.
- Feed honesty, keeper arrival and inclusion latency remain named assumptions. A missed call or a timeout is never evidence of nonexecution (MORIARTY-PRODUCT-CONTRACT.md:47).

**External practice:** ACTUS models option exercise and settlement as separate contract events ([ACTUS technical specification](https://www.actusfrf.org/techspecs)). That supports keeping the fixing, exercise and settlement stages separate, but it is not a Moriarty rule. The captured copy (`source-text/actus-techspec.md`) is only the landing page, so I did not check the event-level text.