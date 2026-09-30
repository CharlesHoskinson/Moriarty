# Stablecoins and synthetic assets: recommendations through the collateral and solvency lens

**Status:** This is read-only design review of specified-only material. Nothing here claims implementation, proof, K execution or ledger acceptance. Items marked **Recommendation** are my proposals. **External practice** is not a Moriarty rule. Guarded CLI status was checked first. Its SP01 dispatch block does not affect this review.

## 1. Verdict

**Not fit for the CDP and solvency part of this category yet. The carriers are sound enough to repair without redesigning them.** MIL/2 already provides several correct pieces:
- nominal asset identity (`concepts/intent-language/DESIGN-MIL2.md:53-61`);
- a separate `issue` right (`:191-193`);
- the general conservation law E1 (`:256`);
- obligations as their own records with debtor consent, separate from supply (`:94-100`);
- the lock-sum obligation (`:102`);
- evidence carried as a type index (`:176-187`).

Four solvency gaps block the category:

1. **Health cannot be written in Φ₀.** A CDP health check needs `collateral × price`, a variable-times-variable product. Φ₀ allows only literal coefficients (`:136,160`), `collateral_value` was removed (`:156`), and Φ₁ is deferred to U4 (`:350`). The category report agrees (`deliverables/.../category-review/03-stablecoins.md:19`).
2. **There is no aggregate ceiling cell and no reservation semantics.** The report notes this (`03-stablecoins.md:20`), and R3 G6 argues that a ceiling must bind the accepting path (`deliverables/u0-study-2026-09-28/defi-coverage/R3-stablecoins.md:440-448`).
3. **There is no rule linking a mint to the debt it creates.** Without it, "debt separate from supply" can drift into "debt unrelated to supply."
4. **There is no oracle or policy-version discipline, and no backing relation.** The report lists reflexive backing as Open (`03-stablecoins.md:27`), and R3 G9 treats it as a carrier defect (`R3-stablecoins.md:470-480`).

## 2. Five ranked design edits (recommendations)

### Edit 1: Add an exact collateralization atom with narrow width limits, and require health only on risk-increasing transitions

**Rule.** Add one Φ₀ atom (call it Φ₀ᶜ):

```
Γ ⊢ q : Qty<C> ! S₁   Γ ⊢ p : Price<C,D,s> ! S₂   Γ ⊢ u : Qty<D> ! S₃
Nr, Dr literals ≤ 2^16, p.mantissa ≤ 2^64, s ≤ 18
──────────────────────────────────────────────────────────────────────
Γ ⊢ collateralized(q, p, u, Nr/Dr) : Prop ! S₁∪S₂∪S₃
   ≜  q · p.m · Dr  ≥  u · Nr · 10^s          (exact; no rounding)
```

- Both sides fit in at most 208 bits, which is under the 253-bit `less_than` bound (`DESIGN-MIL2.md:169`). No limb gadget is needed. A mantissa above the cap rejects with `ARITH_RANGE`.
- The escrow authoring check treats the atom as an opaque bounded variable. That is a sound over-approximation: a formula it certifies is valid, and a valid formula it cannot certify is rejected, so it fails closed.

**Transition rule.** Health is required only after a transition that increases risk:

`riskIncreasing(e) ≜ Δprincipal_o > 0 ∨ Δlock_enc < 0`, and `riskIncreasing(e) ⇒ collateralized(post(locked), p, post(outstanding(o)), LR)`.

Repaying debt and adding collateral are always admissible. **External practice:** MakerDAO's `frob` has the same shape, `require(either(both(dart <= 0, dink >= 0), tab <= _mul(urn.ink, ilk.spot)), "Vat/not-safe")` (https://github.com/makerdao/dss/blob/master/src/vat.sol).

**Counterexamples.**
- Without the atom, no CDP guard can be written.
- If health were required on every transition, an underwater debtor could not repay or add collateral, so liquidation would become the only exit.
- If the comparison used a floor-rounded `valueOf`, 1 extra unit could pass at the boundary.

**Rounding.** Rounding appears only when a quantity is materialized, such as maximum mintable or seize amounts. It follows the numeric profile's D2 rule: ceil for amounts owed and collateral charges, floor for amounts received (`deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json:231-245`).

**Placement and why this changes the Φ₁ boundary.** Reserve the constructor and its width caps in the U0 version header. Certify it in U1, and first use it in a U2 slice. This moves one product shape out of the deferred Φ₁ set. It is justified because it has a bounded width with no limb, needs no SMT, and has no division. General products and `sharesFor`/`assetsFor` stay in Φ₁.

### Edit 2: Add an aggregate ceiling cell, checked by the ledger, with reservations

**Rule.**
- Add the cell `aggregate(id) = {value, reserved, cap, kind ∈ {debtPrincipal, supply}}`.
- A creating transition must satisfy `post.value = pre.value + Δ` and `post.value + post.reserved ≤ cap`.
- The locus must be named: either the ledger re-checks it against the authenticated current state at inclusion, or the proof binds `pre` to the current head and the ledger rejects on a mismatch.
- A proof over a snapshot `pre` that the ledger does not re-check is forbidden.
- Cross-stage reservations follow K1 (`MIL2-PROPOSED-SEMANTICS.tex:229-237`): `reserve → commit | release`, with tombstones.
- Ceilings gate only *creation*. Accrual and discharge are never rejected because of a ceiling.

**External practice:** Maker gates only positive `dart`: `either(dart <= 0, both(... <= ilk.line, debt <= Line))`.

**Counterexamples.**
- Two solvers each prove `0 + 60 ≤ 100` against a stale snapshot, and 120 is minted.
- In a two-stage Episode, stage 1 locks collateral and competitors then use up the headroom. Stage 2's mint fails and the collateral stays stranded with no issuance.
- If a ceiling rejected accrual, that interest would never be recorded, which breaks the rule that debt cannot disappear (`docs/MORIARTY-PRODUCT-CONTRACT.md:45`).

**Placement.** U0: cell kind and footprint vocabulary (`DESIGN-MIL2.md:239-243`). U2: single-stage use. U3: reservations.

### Edit 3: Link issuance to debt in the same stage, keeping them separate records

**Rule.** For an asset whose policy is `collateralized`, a stage with `Δsupply(d,U) > 0` must also:
- create an obligation `o` with `creation_o = Δsupply` under consent (L1, `MIL2-PROPOSED-SEMANTICS.tex:180-187`);
- create or update an encumbrance with `o ∈ against`;
- satisfy Edit 1 at `post`.

For burn-repay, only the principal part `d_p` is burned. The accrued part `d_a` is *transferred* to a named surplus account, which E1 accounts for. That gives a named roll-forward obligation:

`supply'(U) = supply(U) + Σcreation − Σ burned principal + authorizedNonDebtIssue`,

where the last term needs its own `issue` budget. Debt stays in L1 and never appears inside E1.

**Counterexamples.**
- 100 USDm minted against an obligation of 1 unit.
- A burn that reduces both supply and debt by the accrued portion, which silently waives fee income.
- A mint with no encumbrance, so nothing is pledged against the obligation.

**Placement.** U0 reserves the `mint`/`burn` effect constructors, the missing refusals (`unauthorizedSupply`, `issuanceDebtMismatch`) and this obligation. S0 remains empty-with-rejection (`R3-stablecoins.md:321`). The first executed use is in U2.

### Edit 4: Make the price policy a signed, versioned policy cell and pin risk terms to the version the debtor consented to

**Rule.**
- Define `PricePolicy{feedCell, orientation Price<C,D,s>, maxAge Duration(c), maxDeviation Nr/Dr, delayedCell?, writerAuthority}`.
- For the first profile, the price is an **anchored authenticated read of a Midnight oracle cell**, not imported or attested evidence. In-circuit signature verification is unavailable (`DESIGN-MIL2.md:264`), and `imported` is a trust premise until U4 (`:187`). The honesty of the oracle writer remains a named premise (TP03).
- Risk-increasing transitions use `min(spot, delayed)`, which is Φ₀. **External practice:** Maker's OSM delay.
- Liquidation needs `fresh` plus a deviation bound, written in cross-multiplied Φ₀ form.
- Missing or stale evidence gives `Reject`. There is no fallback price.
- Risk-reducing transitions need no price.
- `enforce` evaluates health under the `policy(id)` version referenced by `Obligation.consent` (`DESIGN-MIL2.md:193`), or under a later version only if the consented amendment rule allows it (for example, a signed delay).

**Counterexamples.**
- A stale low print liquidates a healthy vault.
- A solver fills a price hole from a wider source set, which the rule at `:185` already forbids.
- Governance raises the liquidation ratio from 150% to 300% and liquidates in the same block.

**Placement.** U0: policy cell and evidence index. U2: anchored price. U4: imported or attested feeds and k-of-n aggregation, which needs `§16.6` collections.

### Edit 5: Add backing edges to asset metadata, with a bounded acyclicity guard on collateral admission

**Rule.**
- Add checked metadata `Asset.backs : {AssetId}` for issued assets and `Asset.endogenous : bool`.
- An `amend` transition that adds collateral class `C` to the policy of issued asset `U` must satisfy `U ∉ closure_k(backs, C)`. Here `k ≤ 4` and the edges are read from authenticated asset-descriptor cells.
- An `endogenous` collateral class must declare its own `aggregate` ceiling (Edit 2) and a haircut `LR`.
- An unresolvable edge gives `unknown`, which rejects.

**Counterexamples.**
- A Terra-style 2-cycle: USDm backed by GOV, with GOV declaring USDm backing. This is expressible today and invisible (`R3-stablecoins.md:219-224`).
- A cycle of depth 5 or more. This is honestly out of reach, and it becomes a residual assumption, not a guarantee.

**Placement.** U0 reserves the metadata field. The guard ships in U6.

## 3. Core versus library boundary (recommendation)

**Core:**
- the Φ₀ᶜ atom and its width caps;
- `mint`/`burn` effects and refusal codes;
- the issuance–debt link obligation;
- the `aggregate` cell with creation-only gating and reservations;
- price-policy version binding and `enforce`/consent pinning;
- the `backs` metadata field.

Each of these must bind the accepting path. A library can only strengthen a guard, never supply one.

**Library (U6):**
- liquidation auctions, bonuses and keeper incentives;
- the loss waterfall, backstop and socialized loss;
- stability fee and savings-rate schedules;
- peg-swap modules and redemption templates;
- dust thresholds;
- the reflexivity *risk scoring*;
- synthetic shared debt pools.

Emergency shutdown stays **Open**. It needs authenticated completeness over all claims, which is a U4 capability.

## 4. Smallest slice and evidence pair

**Slice (proposed U2 structurally contrasting program).** A single-collateral CDP draw on `midnight.preview`:
- lock `C`;
- originate obligation `o` with consent;
- `mint U` to the borrower;
- `aggregate(ilk).value += Δ`;
- anchored price read.

It has one signer, no liquidation and no burn. Liquidation and burn follow as the second step.

**Positive case.**
- Collateral 150 C (6 decimals, so 150·10⁶).
- Price `Price<C,U,6>` with m = 10⁶ (1.00).
- LR = 150/100.
- Draw 100 U: `150e6·1e6·100 = 1.5e16 ≥ 100e6·150·1e6 = 1.5e16`, which holds at equality.
- Aggregate pre 900, cap 1000, post 1000.
- Obligation creation equals the supply delta (100e6).

**Hostile case.** The same state, price read and policy, with a draw of 100e6 + 1 units. This is a well-formed, feasible witness. Only the health atom fails, by 1 unit at the boundary.

It isolates exact arithmetic and must reject in the evaluator, in K and natively. Rejection only because the envelope is malformed would not count (`ROADMAP.md:40`).

**Follow-up hostile controls:**
- aggregate pre = 901;
- obligation principal 99e6 against a mint of 100e6;
- price older than `maxAge`;
- a price hole filled from an imported source.

## 5. Explicit disagreements

1. **With `DESIGN-MIL2.md:160,350`:** I disagree that all variable products belong in Φ₁ at U4. One bounded collateralization product should be certified at U1. Without it, the category has no health guard until U4.
2. **With `03-stablecoins.md:19` (and implicitly the design):** a health check does not need a "complete collateral-value expression." An exact cross-multiplied inequality avoids division and rounding entirely.
3. **With `R3-stablecoins.md:470-480` (G9):** reflexive backing is not only a carrier defect that core fixes. Core carries the edges and a bounded acyclicity guard. Global reflexivity cannot be decided in a bounded stage, so its risk policy belongs in the library.
4. **With `03-stablecoins.md:23` (and R3's attested-evidence direction):** for the first slice, the price should be an anchored Midnight oracle cell, not `attested`. `attested` evidence has no in-circuit verifier.
5. **With "caps as signed intent bounds":** debt ceilings are protocol state (`aggregate` cells), not user intent budgets. The intent may reference a ceiling but cannot own it.

## 6. Residual assumptions

- The Midnight contract state model can re-check the aggregate update against the current state at inclusion. This is unverified and must be pinned in the U0 target matrix.
- A 64-bit price mantissa and 16-bit ratio literals are enough for the admitted assets. The widths are proposals, not measurements.
- The oracle writer's honesty and liveness remain premise TP03. No proof covers them.
- The acyclicity depth bound k = 4 is arbitrary. Longer cycles stay undetected.
- Treating Φ₀ᶜ as an opaque variable in the authoring check may reject valid escrow templates. That failure is accepted because it fails closed.
- K1, L1, E1 and the six stated obligations (`DESIGN-MIL2.md:365`) are still unproved. Every edit above depends on them.
- The MakerDAO citations are external comparative practice from the `dss` master branch. They are not pinned to a commit and are not adopted as Moriarty rules.