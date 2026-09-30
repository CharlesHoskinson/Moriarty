# Lending and borrowing: recommendation on collateral and oracle risk

**Status.** I ran the guarded `status --json` first. It reports capability `SP01.6 loan-swap-subset`. Implementation is blocked by stale binding and candidate input, missing current accounting and unavailable resource state. There are no pending transactions. This is read-only design review: I claim no implementation, proof, compilation or ledger acceptance. Everything below is a **recommendation** unless marked as a repository fact.

## 1. Verdict

**The design is not yet fit for price-conditioned collateralized lending (recommendation).** MIL/2 has the right pieces: debt separate from supply, `Encumbrance.against` as a set, an `observation(feedId)` cell, and source-set typing (`DESIGN-MIL2.md:94-102`, `:239-243`, `:175-187`). But three gaps make its central lending safety claims unenforceable as written.

1. **The lock invariant cannot be checked locally.** The design requires `Σ active locks ≤ balance` (`DESIGN-MIL2.md:102`), and the paper's K1 rule counts reserved locks too (`MIL2-PROPOSED-SEMANTICS.tex:229-237`). But no cell holds the per-owner lock total. A stage would have to read every encumbrance the owner has, which the 32-cell footprint cap forbids (`DESIGN-MIL2.md:280`). The transfer footprint rule F1 also omits locks from `AuxCells` (`.tex:209-216`).
2. **Freshness trusts a time the prover chooses.** `fresh` checks only `observedAt ≤ stageTime` (`DESIGN-MIL2.md:144`, `:187`). Nothing ties `stageTime` to the ledger.
3. **Health checks are nonlinear.** Accruing debt multiplied by an observed price is a variable×variable product. That is Φ₁ (`DESIGN-MIL2.md:160-161`), capped at 0 at U0 (`:276`). `collateral_value` was removed (`:156`). So no accruing collateralized loan with a price condition can be stated before U4.

## 2. Five ranked design edits

### E1. Authenticated lock total and lock state machine (most severe)

**Rule sketch (recommendation):**
- Add a cell `lockTotal(d, owner, asset)`.
- An encumbrance moves through states `reserved(holder, expiry) → committed → {released | seized(n) | cancelled}`. A reservation also moves directly to `expired` after its deadline.
- Every lock transition writes both `encumbrance(id)` and `lockTotal`:
  - `reserve`/`commit` of `n`: requires `lockTotal + n ≤ balance`; sets `lockTotal' = lockTotal + n`.
  - `release`/`cancel`/`expire`: sets `lockTotal' = lockTotal − n`.
  - `seize(s)`: sets `lockTotal' = lockTotal − s` and `balance' = balance − s`.
- Every debit from `balance(d, owner, asset)` derives a read of `lockTotal` and requires `balance' ≥ lockTotal'`. F1's `AuxCells` gains `lockTotal`.
- K1 then follows by induction from these local checks.

**Counterexample (current design):** Alice holds 1.5 ETH. Two solvers each lock 1.0 ETH under new ids `x1` and `x2`. Each stage reads `balance` and writes only its own `encumbrance(xi)`. The write sets are disjoint, so both are accepted and Σ locks = 2.0 > 1.5.

**Placement:**
- The cell sort goes in **U0**. This changes the U0 cell vocabulary (`DESIGN-MIL2.md:345`). The reason: the vocabulary is hash-bound through canonical encoding (`:262`), and adding a cell later forces a digest and version migration.
- The transitions go in **U2**. Reservation expiry goes in **U3** ("reserved work", `ROADMAP.md:24`). Durable concurrent reservations go in **U5** (`ROADMAP.md:26`).

### E2. Current price bound to authenticated time

**Rule sketch (recommendation):**
- `PriceObs = {feedId, base: AssetId, quote: AssetId, scale s, value > 0, observedAt, seq, policyDigest}`.
- Admission requires:
  - an authenticated read of `observation(feedId)` **at the stage's predecessor head**;
  - `seq = current(feedId).seq`, i.e. the current value, not any historical one;
  - base and quote nominally equal to the signed `Price<…>` indices;
  - `txValidityUpper − observedAt ≤ maxAge`, where `txValidityUpper` is a public input the ledger's validity rule enforces.
- `stageTime` is never a free witness.
- **Fix the price orientation.** "Base-per-quote" (`DESIGN-MIL2.md:69`) contradicts the usual BASE/QUOTE convention (quote units per base unit). Fix it with the typing rule `Qty<A> ⊗ Price<Q per A> → Qty<Q>`. U0 must record a canonical price orientation anyway (`ROADMAP.md:21`).

**Counterexamples:**
- *Stale price:* the price was observed 3 hours ago. The prover sets `stageTime = observedAt + 1m`, and `fresh(price, 5m)` holds.
- *Price shopping:* two prices within the age window (600, then 500). The keeper chooses 500 to trigger liquidation.

**Placement:** the public-input field and the orientation rule go in **U0**. A certificate for the clock comparison goes in **U1**. The slice goes in **U2**. The feed's writer authority stays a named trust premise, as with `imported` (`DESIGN-MIL2.md:187`).

### E3. A certified exact product comparison for typed health

**Rule sketch (recommendation):**
- Admit exactly one certified nonlinear atom, `prodLe(a·b·k₁ ≤ c·d·k₂)`: variable terms `a, b, c, d`, literal coefficients `k₁, k₂`, and a declared limb width under the limb rule (`DESIGN-MIL2.md:167-171`).
- Health is a **derived proposition**, never a stored state field (R2 A23, `R2-lending.md:235`):
  `maintOK(o, enc, p) := prodLe(outstanding(o) · 10^(dA+s) · 10⁴ ≤ amount(enc) · p.value · 10^dQ · maintBps)`, with decimal scaling folded into the literals.
- An exact comparison needs no rounding.
- Amounts that would need division are supplied as witness holes and checked by a bounding inequality. The allowed direction of the bound is the rounding role:
  - Seize: `prodLe(seize · p · 10⁴ ≤ repay · (10⁴ + bonusBps) · scale)`. This rounds toward the debtor.
  - Borrow capacity: collateral is valued down; debt is valued up.
- This follows the external ERC-4626 practice of role-directed rounding that favors the protocol ("favor the Vault", `source-text/erc4626.md:631`; https://eips.ethereum.org/EIPS/eip-4626). That is external practice; the Moriarty rule is the witness-and-bound form above.
- The escrow authoring check treats `prodLe` as an opaque Boolean. If it cannot decide the result, the answer is `unsupported` and fails closed (`.tex:106`).

**Counterexample (current design):** debt accrues from 1,000 to 1,010. With both quantities as signed literals, the guard "price ≥ literal" remains Φ₀. But it now uses 1,000 instead of the real debt, so a position that is actually unhealthy passes.

**Placement:** this **changes a deferred Φ₁ boundary**. The U0 cap becomes "0 generic nonlinear nodes; at most 2 `prodLe` nodes," with the atom's tag reserved in **U0**. It is certified in **U1** as a two-limb comparison with completeness and soundness against adversarial witnesses (`ROADMAP.md:22`), and admitted in **U2**. General `mulDiv` and share math stay Φ₁ and **U4**. Without this change, there is no price-conditioned loan with accruing debt before U4, even though U2 keeps the existing loan requirement (`ROADMAP.md:23`).

### E4. Liquidation transition with residual debt and an impaired state

**Rule sketch (recommendation):** `liquidate(o, enc, repay, seize)` requires:
- `enc.state = committed`, `o ∈ enc.against`, and a keeper `enforce` right bounded by `o.consent` (`DESIGN-MIL2.md:193`);
- `¬maintOK(o, enc, p)` at an E2-current price;
- `repay · 10⁴ ≤ outstanding · closeBps` (linear, Φ₀);
- the E3 seize bound;
- `seize ≤ enc.amount`.

Effects:
- the keeper's repay goes to the creditor;
- AccrualFirst discharge;
- `enc.amount −= seize`, with the E1 `lockTotal`/`balance` update;
- `seize` moves from owner to keeper;
- if `enc.amount = 0 ∧ outstanding > 0`, then `o.status := impaired` and debt stays, per the L1 rule (`.tex:178-187`).

**Serialization:** use compare-and-swap on the encumbrance version at the authenticated head. A *signed* order (`DESIGN-MIL2.md:193`) cannot name keepers who are unknown at signing time. Keep that signed order only for forks inside one episode.

**Counterexamples:**
- A keeper claims `seized(enc)` with seize = all collateral for a 300 repay. The Boolean atom does not constrain the amount.
- A zero-collateral residual gets silently closed, erasing the debt.

**Placement:** fixed-bonus partial liquidation in **U2**. Partial progress and the impaired continuation in **U3**. Auctions and loss waterfalls go to the **U6** library.

### E5. Market scope, isolation, aggregate health and a utilization cap

**Rule sketch (recommendation):**
- Add `market: PoolId` to both `Obligation` and `Encumbrance` (canonical fields).
- Adding `o` to `enc.against` requires `o.market = enc.market`, or a consented basket in `policy(id)`. It also requires the **aggregate** health condition `Σ_{o∈against} debtValue ≤ collValue · ltv`, stated as linear sums of `prodLe` terms under caps.
- Utilization cap: `borrowed · 10⁴ ≤ totalAssets · capBps` is linear, so Φ₀. Here `totalAssets` means cash plus receivables, and receivables are never counted in the supply rule E1.

**Counterexample (current design):** Alice's 1.5 ETH lock backs `o1` in isolated market M1. Then `o2` in M2 is added to `against`. Σ locks is unchanged, so K1 passes while the isolation is broken. This is hazard X8 (`R2-lending.md:143`).

**Placement:** the fields go in **U0** so they are in the canonical encoding. The same-market rule goes in **U2**. E-mode baskets, rate curves and reserve factors go in **U6**.

## 3. Core versus library (recommendation)

**Core:**
- the `lockTotal` cell and lock states, with K1;
- `PriceObs` binding, authenticated time bounds and price orientation;
- `prodLe` and rounding roles;
- health as a derived proposition;
- the liquidation invariants: debt persists, seize is bounded, the impaired state exists;
- market fields and the same-market rule;
- linear utilization caps.

**Library:**
- specific LTV, maintenance threshold, close-factor and bonus values;
- auctions, interest-rate curves and index accrual (Φ₁), e-mode baskets, reserve factors;
- oracle aggregation such as median or TWAP. This needs bounded observation collections, which are still open (`DESIGN-MIL2.md:360`);
- confidence and deviation policies.

## 4. Smallest slice and an evidence pair

**Slice:** one domain; one fixed-rate loan with accrual held at 0; one committed encumbrance; one E2-current price; one E3/E4 fixed-bonus partial liquidation.

**Positive case:**
- Alice has 1.5 ETH locked, and `lockTotal` = 1.5.
- Debt is 1,000 USDC. Maintenance is 8,000 bps, close factor 5,000 bps, bonus 500 bps.
- Feed ETH→USDC is current at `seq = 42`, price 600, age within `maxAge`.
- Health: 1,000·10⁴ = 10⁷ > 1.5·600·8,000 = 7.2·10⁶, so the position is unhealthy.
- Repay 300 ≤ 500, so it is within the close factor.
- Seize bound: `seize · 600 · 10⁴ ≤ 300 · 10,500` gives seize ≤ 0.525 ETH. The keeper takes 0.525.
- Post-state: outstanding 700, encumbrance 0.975 ETH, `lockTotal` 0.975, balance 0.975.

**Hostile cases.** Each differs from the positive case in exactly one field and is otherwise feasible (`ROADMAP.md:40`):
- **(a)** seize = 0.526 ETH, which rounds in the keeper's favor. Must reject: seize bound.
- **(b)** The current price is `seq = 43` at 900, so the position is healthy (1.5·900·8,000 = 1.08·10⁷ ≥ 10⁷). The prover submits the still-in-age `seq = 42` at 600. Must reject: observation not current.

A companion two-solver trace must reject the double-lock counterexample from E1.

## 5. Explicit disagreements

- **Φ₀ health thresholds.** The category review calls a price threshold "plausible in Φ₀" (`02-lending.md:20`). That holds only when both quantities are literals. With accruing debt the check is variable×variable, which is Φ₁ (`DESIGN-MIL2.md:160`).
- **K1 is a design gap, not only a proof gap.** The review lists K1 as a proof obligation (`02-lending.md:19`). Under concurrency it cannot be proved with the current cell vocabulary; E1 is needed first.
- **The Σ rule does not prevent double pledging.** The design says it does (`DESIGN-MIL2.md:102`). Extending `against` to another market reuses collateral without increasing Σ (E5).
- **Liquidation serialization.** The design makes a signed total order core for competing seizes (`DESIGN-MIL2.md:193`). For liquidations by unknown keepers, ledger compare-and-swap is enough; a signed order cannot name them.
- **Broken links.** The category report's links to R2 point to `../../../u0-study-2026-09-28/…`, which resolves to the repository root. The file is at `deliverables/u0-study-2026-09-28/defi-coverage/R2-lending.md`.

## 6. Residual assumptions

- **Oracle honesty and feed writer.** These remain a named trust premise. Who may write `observation(feedId)` is unspecified.
- **Ledger time bound.** I assume Midnight exposes a transaction validity-time bound that can be a public input; this is unverified.
- **Limb cost.** Products of u128 values with literal factors may exceed 256 bits, and `less_than` stops at 253 bits (`DESIGN-MIL2.md:169`). The limb count and cost must be measured in U1.
- **Liveness is not guaranteed.** Keeper arrival, liquidation latency (`R2-lending.md:478-483`) and exit liquidity are not safety properties.
- **Arithmetic assumptions.** The worked example assumes decimals and scaling fold exactly into literals, and that caps permit the aggregate health sums.
- **Nothing here is verified.** None of the edits has been checked by K, TypeScript, ZKIR or the ledger.