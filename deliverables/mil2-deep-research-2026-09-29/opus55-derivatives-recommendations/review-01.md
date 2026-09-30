# MIL/2 Derivatives: instrument and payoff semantics (independent recommendation)

**Status:** This is read-only, specified-only design review. Nothing here claims an implementation, a proof or ledger acceptance. `status --json` reports SP01.6 as blocked on stale evidence. That block does not affect this review.

## 1. Verdict on category design fitness

**Not fit as specified for any derivative payoff. It becomes fit for one bounded slice with five targeted edits.**

MIL/2 has sound carriers:
- a signed `Position<I>` separate from `Delta<A>` (DESIGN-MIL2.md:65–76)
- clock-indexed time (:71–73)
- a provenance-typed `Obs` (:175–187)
- a `position(acct,instrument)` cell (:240)
- a transitional escrow (:199–219)

On instrument semantics, four defects block every option:

1. **`Instrument` has no denotation.** It is an "opaque identifier (underlying, expiry, strike, side-convention)" (:58). It is unclear whether identity is nominal or structural. The terms are unbound, so they can change under an open position. `side-convention` duplicates the sign that `Position` already carries.
2. **`exercised(opt)` has no producer** (:149). `opt` has no sort and no cell, and no effect sets it. `not exercised(opt)` is therefore unconstrained (category report 04-derivatives.md:17).
3. **There is no fixing.** `fresh(obs,Δ)` (:144) bounds an observation's age. It does not choose which observation fixes the payoff.
4. **The natural payoff rejects.** Checked subtraction rejects on underflow (:124), so `max(S − K, 0)` returns `Reject` for every out-of-the-money option. Acceptance then fails closed, and an expired worthless option can never settle.

## 2. Five design edits, ranked (recommendations)

### E1. Deterministic write-once fixing (highest priority)

**Rule sketch.** Add a cell `fixing(I) ∈ {unfixed, fixed(v, obsId, t)}`. The instrument signs a fixing key `κ_I = (feedId, t_fix, policy)` at issuance.

```
FIX:  pre(fixing(I)) = unfixed
      o.id = key(κ_I)    o : Obs<Price<U,Q,s>, anchored, d>    o.observedAt = t_fix
      stageTime ≥ t_fix
   ⟹ post(fixing(I)) = fixed(o.value, o.id, t_fix); write the tombstone
```

- Selection is by key equality, not by freshness. The circuit binds `o` to the authenticated ledger read at `κ_I` (DESIGN-MIL2.md:185).
- If no fixing exists by `t_fixDeadline`, only a **signed** fallback branch applies: an alternate key, or a return of premium and collateral. It is never inferred, because a timeout is not evidence (MORIARTY-PRODUCT-CONTRACT.md:47).

**Counterexample.** A call fixes at 17:00 under `require fresh(px, 5m)`. The feed publishes 3100 at 17:00 and 3150 at 17:02, and both are fresh at 17:03. The holder's solver presents 3150, and every current clause accepts it. The same gap lets a writer's solver pick the lower print.

**Placement.** The cell sort and key encoding go in **U0**, because the cell vocabulary freezes there (:345). The FIX transition goes in **U3**. Imported or attested fixings follow in **U4** (:350).

**U0 boundary change, and why.** It adds `instrument(id)` and `fixing(id)` to the U0 cell vocabulary. Omitting them would force a version-header migration later. Recording `observation(feedId)` (:241) is not enough: it names a feed, not a fixing event.

### E2. Instrument as a nominal, immutable, terms-digested record, with an exposure-conservation law

**Rule sketch.**

```
Instrument ::= { id: InstrumentId, termsDigest, kind: call|put, style: european,
                 settle: cash(Q), underlying: U, strike: Price<U,Q,s>, cap: Price<U,Q,s>,
                 unit: lit, expiry, κ_fix, t_fixDeadline, fallback, collateral: Q }
```

- Delete `side-convention`. Long or short is the sign of `Position`.
- **P0 (exposure conservation):** for each `I`, `Σ_acct post(position(acct,I)) = 0`.
- **OPEN:** writes `+n` / `−n`, transfers the premium, and encumbers the short's collateral of `n·unit·(cap − K)/10^s` against `I` (K1, MIL2-PROPOSED-SEMANTICS.tex:229–235).
- `amend` may not change `termsDigest` while open interest is nonzero.

P0 is an exposure law, deliberately separate from E1 supply. A position is not token supply, just as debt is not (MORIARTY-PRODUCT-CONTRACT.md:43).

**Counterexamples.**
- Two venues list (ETH, 17:00, 3000, call). Under a structural reading, one `position` cell merges them. This is the MIL/1 defect repeated at instrument level (:76).
- A `policy(id)` amendment changes the strike after opening. The holder's signed premium then buys a different claim.

**Placement.** The record's canonical encoding and P0 as a judgment go in **U0**. OPEN goes in **U2/U3**.

### E3. Total positive part and bracket rounding in Φ₀ (the payoff equation)

**Rule sketch.**
- Define `pos(a, b) := max(a, b) − b`. It is total, because `max(a,b) ≥ b`.
- Add a lint that rejects `max(a − b, 0)` in payoff positions.
- For a cash call spread of `n` contracts (`n` a signed literal in the settlement intent), with `S` the fixing:

```
X := min(pos(S, K), C − K) × (n·unit)          -- Price×lit: Φ₀, limb gadget if > field
P × 10^(s + dec_U − dec_Q) ≤ X  <  (P + 1) × 10^(s + dec_U − dec_Q)
```

The payout hole `P : Qty<Q>` is thus fixed to its floor by two literal-coefficient inequalities, which is Φ₀ (:160). No division primitive is needed.

- **Rounding beneficiary:** the collateral pool. The floor then gives `Σ P ≤ locked`, and the remainder goes to the writer as residual.
- The put payoff is `pos(K, S) = K − min(S, K)`.
- Also fix the ambiguous "base-per-quote" gloss (:69). For payoffs, `Price<U,Q,s>` must mean Q per U, as the U0 numeric profile requires (ROADMAP.md:21).

**Counterexamples.**
- At S = 2900, K = 3000, the `max(S − K, 0)` form rejects, and the writer's collateral is locked forever.
- With only an upper bound `P×10^k ≤ X`, a writer-side solver settles `P = 0`. Only the two-sided bracket determines `P`.

**Placement.** The operator, the lint and the rounding-beneficiary rule go in **U0**. The bracket gadget certificate goes in **U1** (ROADMAP.md:22). Position-size × payoff with `n` read from state is variable × variable, so it stays in **Φ₁/U4**. Requiring `pre(position) = n` as a Φ₀ equality keeps the slice in Φ₀.

### E4. Replace `exercised(opt)` with a produced, consumable exercise mark; auto-exercise for European cash options

**Rule sketch.**
- Type the atom as `exercised(acct, I)`, reading a `receipt(ex(acct, I))` cell.
- It is produced only by an `EXERCISE` effect, whose guard is `within(I.exerciseWindow) ∧ pre(position(acct,I)) > 0`.
- The mark is consumed once by SETTLE.
- For `style: european, settle: cash`, exercise is **automatic**. SETTLE's guard depends only on `fixing(I) = fixed(·)`, and there is no exercise action at all.

This follows ACTUS, where the Exercise Date "marks the observed timestamp of fixing … not necessarily the timestamp of settling" (external practice: https://github.com/actusfrf/actus-dictionary/blob/master/actus-dictionary-terms.json, terms `XD`, `XA`, `STP`).

**Counterexample.** Today a writer's refund branch `after(expiry) ∧ not exercised(opt)` is satisfiable when the holder's exercise stage is merely late. That is the lazy non-exercise hazard (R4-derivatives.md:440–447). Under auto-exercise, no holder action exists to miss.

**Placement.** Typing the atom, or removing it from Φ₀ until it has a producer, is a **U0** change. Freezing a producer-less atom into the digest is worse than deferring it. EXERCISE goes in **U3**. American, Bermudan and physical exercise go in the **U6** library.

### E5. Settlement as a fixed liability, not a same-stage transfer

**Rule sketch.**
- At FIX, for each open pair, the settlement stage creates `Obligation{debtor: writer, creditor: holder, principal: P, rank, consent: OPEN's signature}`. This is ACTUS `XA` fixed at `XD`.
- SETTLE discharges it within `t_fix + STP` from the encumbrance under L1 (tex:178–187).
- If settlement is not done, the duty persists and `enforce` may seize.
- Positions go to 0, and a terminal tombstone is written.

**Counterexample.** Suppose the payout is a direct escrow release, and settlement is delayed past the fallback deadline. The refund branch then fires and the holder's fixed entitlement disappears. That is erased debt (ROADMAP.md:40).

**Placement.** This goes in **U3**. It reuses the Obligation/L1 carriers, so no new core primitive is needed.

## 3. Core versus library boundary (recommendation)

**Core (U0–U3):**
- the nominal `InstrumentId` and terms-digest encoding
- the `instrument`, `fixing` and `receipt(ex)` cells
- the P0 exposure law
- the FIX transition with deterministic key selection and a signed fallback
- the total `pos` operator and bracket rounding with a declared beneficiary
- the exercise-mark producer/consumer rule
- the obligation-at-fixing hook

These are generic. Insurance triggers, prediction-market resolution and tranche loss events reuse them.

**Library (U6, ROADMAP.md:27, 48):**
- call/put/spread/digital formulas
- exercise styles and auto-exercise thresholds
- settlement periods and physical delivery
- margin, funding, liquidation and ADL
- volatility-surface pricing
- listing governance
- claim tokenization through `issue`

## 4. Smallest implementable slice and evidence pair

**Slice (recommendation).** One domain, one writer, one holder, `|S|=1`. An anchored fixing. A cash-settled European **call spread**: K = 3000.00, C = 3200.00, s = 2, Q = USD with 6 decimals, unit = 1, n = 1. Three stages, within the 8-stage cap (:281):

1. **OPEN:** premium 5 USD; lock `200_000_000` Q; positions ±1.
2. **FIX** at `κ = (feed "eth/usd", 17:00)`.
3. **SETTLE:** payout `P` by the bracket, residual to the writer, positions to 0, tombstone.

Everything stays Φ₀ except the declared limb gadget.

**Positive control.** The fixing at `κ` is 3100.07 (`310007`).
- `X = 10007 × 1`, and the bracket gives `P = 100_070_000`.
- Holder +100.07 USD, writer +99.93 USD, lock released.
- E1 holds per asset, P0 holds, and the obligation is created and discharged.

**Hostile control (a feasible witness, not a malformed envelope, per ROADMAP.md:40).** Everything is identical except one input: the solver presents an authenticated, fresh-within-5m observation from the same feed at 17:02, with value 3150.00 and `P = 150_000_000`. All budget and conservation clauses still hold. Only FIX's key equality `o.id = key(κ)` fails, so native acceptance must reject.

A second hostile control keeps the correct fixing with `P = 100_070_001`. The bracket's upper inequality fails.

Controls deferred to the E4 slice: double exercise, early exercise, and an out-of-the-money settlement (S = 2900, which must succeed with P = 0).

## 5. Explicit disagreements

- **With the category report (04-derivatives.md:52).** It recommends a "fully collateralized, cash-settled European call". An uncapped cash call cannot be fully collateralized in the quote asset, because the payoff is unbounded in S. It needs a cap, which makes it a spread, or collateral in the underlying, which needs division by S and so Φ₁. Its "exercise cells" and "double/early exercise" controls do not apply to European cash auto-exercise. They belong to a second slice.
- **With R4 (R4-derivatives.md:289) and D9/D15 framing.** R4 treats contingent claims as supply mint/burn. I recommend non-supply exposure cells under P0, with tokenization as an optional library through `issue`. This keeps exposure, debt and supply as three separate quantities.
- **With DESIGN-MIL2.md:149 as U0 Φ₀ content.** `exercised(opt)` should not be frozen without a sort and a producer.
- **With DESIGN-MIL2.md:58.** `side-convention` inside `Instrument` should be deleted.

## 6. Residual assumptions

- Midnight can expose an authenticated read of a feed value at a specific key `(feedId, t)`. This is unverified, and feed honesty remains trust premise TP03 (trust-premises.json:104).
- `price × unit × 10^k` fits a declared two-limb width. Gadget cost and the §12 caps are unmeasured proposals (:270).
- A settlement actor arrives before `t_fix + STP`. Safety does not supply liveness (tex:204–206).
- The ACTUS mapping (XD/XA/STP) is used as comparative practice, not as a Moriarty rule. Only the dictionary terms were checked; the full ACTUS option state machine was not.
- Everything is single-domain. Cross-domain fixings and multilateral netting stay at U4 and post-U4 (:350–351).