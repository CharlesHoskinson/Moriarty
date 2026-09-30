Status and inputs are read; I'm writing the recommendation now.

# MIL/2 AMM and exchange: intent completion and market structure

*Read-only review. Status checked (`status --json`: `SP01.6 loan-swap-subset` is blocked for implementation, which does not affect this review). No file was edited. Labels: **[fact]** is a source or repository fact, **[inf]** is a design inference, **[rec]** is a recommendation.*

## 1. Verdict

**[inf]** MIL/2 fits the **trader side** of an AMM trade: exact assets, gross, fee and net caps, recipient, validity and escrow. It does not yet fit the **market structure** around that trade. Four gaps are the reason:

1. **No surplus rule.** A solver can legally keep any output above the owner's floor. `fees <= 1 A` caps the input side only (`DESIGN-MIL2.md:299`), and the example leaves ordering and MEV exposure in unparsed residue (`:324`).
2. **Route is contradictory and invisible.** `Route` "may be referenced only by the effect grammar, never inside a predicate" (`:156`). Yet the same document bounds it with `where route.hops ≤ 3` (`:224`, `:301`). Nothing names the venue or a per-hop effect. The semantics draft hides "route-internal choices designated as holes" from the signer's projection π_I (`MIL2-PROPOSED-SEMANTICS.tex:109`) without saying whether intermediate transfers count as choices or as effects.
3. **No partial-fill quantity model.** Escrow states are only full release or full refund (`DESIGN-MIL2.md:203`).
4. **RFQ and resting orders have no single-signer carrier.** Everything two-sided is sent to post-U4 multi-signer admission (`:336`, `:351`).

**[rec]** Most of RFQ, limit orders and partial fills can be carried at U3 **without** giving up `|S| = 1`. True uniform-price batch clearing cannot, and should stay post-U4.

## 2. Top five changes, ranked

### R1. Replace residue with a signed surplus and completer clause (core; U0 schema, U2 enforcement)

```
completion {
  completers  any | allow {key…} | exclusive key        -- replaces `complete by any solver`
  surplus     to owner | to completer | split num/den   -- REQUIRED, no default in canonical form
  completer_fee <= q X                                   -- per asset, counted in `fees`
}
```

Normative rule, per output asset B. Let `out_B` be the sum of route output credited to accounts other than the pools and venues, and `floor_B` the signed net floor. Then:

- `surplus_B = out_B − floor_B`. The checked subtraction rejects on underflow.
- `to owner` requires `Δbalance(owner,B) = out_B − completer_fee_B`.
- `split n/d` requires `completerTake_B × d ≤ surplus_B × n`. This is Φ₀ because the coefficients are literals. Rounding favours the owner: the completer's floor.
- Any B credit to a party that is not listed rejects under `EffectComplete`.

**Counterexample addressed:** the route yields 21.93 B, 20 B goes to the owner and 1.93 B goes to the solver's address. Every current MIL/2 cap passes this. **[inf]**

**Why core:** the owner's authorization is incomplete without a residual claimant. The prior R1 study treats this as a required law (`R1-amm-exchange.md:187`). `completers` gives the "solver-set structure" a typed home and binds the `complete` right (`DESIGN-MIL2.md:191`). The MEV and disclosure regime stays declared rather than enforced (see §6).

### R2. Give Route a fixed finite projection and mandatory per-hop effect records (core grammar; U0 schema, U3 enforcement)

```
Route ::= [Hop]≤3
Hop   ::= { venue: ProgramId@Domain, pool: PoolId, in: AssetId, out: AssetId,
            amtIn: Qty<in>, amtOut: Qty<out>, lpFee: Qty<in> }
```

- **[rec]** Keep `Route` out of Φ as a sort. Add a closed set of **projection terms** that are Φ₀: `route.hops : u8`, `route.venues : Set<ProgramId>`, `route.assets : Set<AssetId>`, and `route.lpFees(A) : Qty<A>`. Each carries the source set of the completion. This makes `:224` legal and allows `route.venues ⊆ {P1,P2}` and `route.assets ⊆ {A,B,C}`, using existing `⊆` (`:142`).
- **Chain rule:** `hop[i].out = hop[i+1].in` and `hop[i].amtOut = hop[i+1].amtIn`. The first `in` is the funded asset and the last `out` is the floor asset.
- **Venue rule:** each hop's effects are the **prepared effects of that venue program's own accepted transition** (`FinancialPrepared`, `.tex:161`), never solver-asserted numbers.
- **[rec]** Amend `.tex:109`: π_I may hide *which* hop was chosen, never a transfer, fee or custody change that a hop produces. Every hop effect remains in `EffectComplete` and in the derived footprint (F1, `.tex:211`).

**Counterexamples addressed:**

- A route through an unlisted wrapper asset C whose `repr` link is synthetic.
- A venue that "returns" 20 B but leaves an allowance, or takes an undeclared fee from escrow.
- A hop through an attacker-deployed pool that satisfies `net ≥ 20 B` while pulling an unrelated reserve.

**Placement:** within one stage, a route stays single-domain (`.tex:59`). Multi-hop inside one stage needs a Midnight transaction that invokes several contracts atomically in one phase. **[inf, unverified]** U0's target matrix must pin that capability. If it is absent, U3 admits `hops = 1` per stage. A multi-hop route then becomes a ledger-linked episode, and the owner must sign `intermediate { hold C permitted | forbidden }` as an allowed partial outcome. Without that clause, a committed hop-1 stage that strands C is inadmissible.

### R3. Quantity-indexed partial fill with a cumulative ratio invariant (core escrow transition; U3)

Extend the escrow state machine (`DESIGN-MIL2.md:203`):

```
pending(rem, cumIn, cumOut) --fill(f, r)--> pending(rem−f, cumIn+f, cumOut+r)   | released if rem−f < minFill
guard:  minFill ≤ f ≤ rem
        (cumOut+r) × den ≥ (cumIn+f) × num      -- literal limit price, Φ₀, cumulative
        fills + 1 ≤ maxFills  (≤ Episode cap 8)
```

**[rec]** Check the ratio on **cumulative** totals, not per fill. The owner-favouring direction is ceiling on output owed and floor on input released.

**Counterexample addressed:** with a per-fill check, a solver making 1-unit fills can extract a rounding remainder each time. The cumulative check holds on the running totals no matter how the fills are split. `minFill` and `maxFills` cap griefing. **[inf]**

The remaining quantity is a residual duty rolled forward in `obligationRollForward` (`:46`). Refund of the remainder does not reset gross debit (`.tex:189`). This is the partial-fill carrier that U3 needs (`ROADMAP.md:24`), which the category review found missing (`01-amm-exchange.md:16`).

### R4. Standing-offer consumption: RFQ and limit orders under `|S| = 1` (core stage rule; U3, arity m ≤ 1)

Rule: a stage verifies **exactly one fresh signed intent** and may consume **at most m previously funded standing escrows** that carry `completers any` (or an allowlist that includes the fresh signer). For each consumed escrow E:

- E's signed guard (Φ₀, literal price, R3 fill semantics) is evaluated in-circuit against this stage's effects.
- E's head is read from authenticated ledger state.
- E's fill or tombstone is written by this stage.

E's authority was bound when E was funded, in its own single-signer stage. **[rec]** Count consumed standing escrows as **ledger state**, not as signers. `|S|` stays 1.

- **RFQ:** the maker funds `escrow Q { fund 20 B; release_when delivered(A, ≥ f×11/20, to maker); refund_when after(t_q) ∨ maker_cancel }`.
- **Last look** becomes the maker's cancel transition. Cancel racing a fill is settled by `priority` plus the one-shot tombstone (`:216`), exactly like the late-success race.
- **Inventory risk** disappears because Q is funded and locked (K1, `.tex:235`).

**Counterexamples addressed:**

- A stale quote after the maker's inventory moved: impossible, because the funds are locked.
- A replayed quote: the tombstone or remaining-quantity head is consumed once.
- A taker filling at a price better than the maker signed: the guard rejects.

**Limits [inf]:** a maker who wants an *unfunded* signed quote (off-book, credit-based) still needs real `|S| = 2`. That stays post-U4.

### R5. Reserve the clearing relation's public-input shape now; admit it post-U4 (U0 reserve only)

Decision 4 reserves a signer set (`:336`). **[rec]** Also reserve, in the stage public-input schema and version header:

- `signerSetCommitment`
- `clearingKind ∈ {none, uniform, discrete-grid}`
- a per-participant `netDelta` vector (cap 16)
- a per-participant `limitRef`

The uniform-price rule is: for each filled order i, `fill_i × p ≤ limit_i`, conservation holds across the vector, and one price p applies to all. Because p is a solver-chosen variable, `fill_i × p` is **Φ₁** (`:160`). **[inf]** A `discrete-grid` profile with `p ∈ {p1..pk}` and k ≤ 8 reduces each branch to literal coefficients (Φ₀), at the cost of case splits under the `k_of_n`/`or` caps (`:277–278`).

**Counterexample addressed:** retrofitting a shared-write clearing stage after U0 freezes the header. That is the "very high" reversal cost the design itself names (`:336`).

**Explicit non-claim:** the relation cannot prove that the matching is maximal or fair. That needs an optimality predicate outside Φ₀ and Φ₁. It must be disclosed as `market_structure`, not implied.

## 3. Library versus core

| Core (acceptance relation, grammar, public inputs) | Library (source contracts over the core) |
|---|---|
| Surplus and completer clause (R1); `EffectComplete` credit accounting | Constant-product, stable and weighted pool programs; fee tiers; LP share mint/burn |
| Route projections, the chain rule, and "venue effects = venue's prepared effects" (R2) | Router heuristics and quote search; any solver |
| Quantity-indexed escrow fill transition, cumulative ratio, `minFill`/`maxFills` (R3) | RFQ quote formats, maker cancel UX, limit-order books as sets of standing escrows |
| Standing-escrow consumption rule and arity cap (R4) | TWAP/reference-price guards (after bounded observation collections, `:360`) |
| Clearing public-input reservation (R5); checked mul/limb gadget as a certified U1 primitive (`:171`) | Batch-auction matchers (post-U4) |

**[inf]** One distinction the category review blurs: the pool invariant `x'·y' ≥ x·y` is the **venue program's** contract invariant (developer-specified, `MORIARTY-PRODUCT-CONTRACT.md:37`), not a clause in the trader's Φ. Deferring Φ₁ blocks *intents* that multiply variables. It does not block a pool **library** written in source/Core using checked multiplication. The R1 study reports that multiplication in `financial-expression-v1.ts:340` yields a typed amount product (`R1-amm-exchange.md:190`); I did not verify this in code. The trader's intent never needs to state the invariant. It needs only R2's rule that each hop's effects are the venue's accepted effects.

## 4. Smallest implementable slice and evidence pair

**Slice (U2 → U3 boundary):** one hop, exact input, single signer, one domain (`midnight.preview`), against one constant-product pool program P.

- **Numeric bound:** P declares reserve width `u64`, so `(x·1000 + dx·997)·y` stays well under the 253-bit comparison limit (`DESIGN-MIL2.md:169`). This sidesteps the limb gadget for the first slice. The bound is declared, not silent.
- **Intent:** `AcquireB` (`:289`) with the footprint fix, plus `route.venues ⊆ {P}`, `route.hops = 1`, `surplus to owner`, and `completers any`.

**Positive vector [inf, computed here, not executed]:**

- x = 10¹² A-units, y = 2·10¹² B-units, dx = 11·10⁶, fee 3/1000.
- dy = ⌊dx·997·y / (x·1000 + dx·997)⌋ = **21,933,759**.
- The pool check `(x·1000+dx·997)(y−dy) ≥ x·y·1000` holds. The largest intermediate is 91 bits.
- The owner receives 21,933,759 ≥ 20·10⁶, gross debit is 11·10⁶, and the LP fee is embedded at about 33,000 ≤ 1 A.
- Expected: accept, with complete effects on the P reserves, the owner and the escrow.

**Hostile control:** identical valid pool math, but the solver credits 20,000,000 B to the owner and 1,933,759 B to itself.

- Expected: **reject** on the surplus clause (R1).
- Under today's MIL/2 this candidate would be **accepted**. That makes it a discriminating control, not an envelope-malformation reject (`ROADMAP.md:40`).

**Secondary hostile:** pay dy + 1 = 21,933,760. The pool invariant check fails (verified arithmetically above). Expected: reject in P's transition, not in the intent.

## 5. Disagreements

1. **With MIL/2 `:156` vs `:224`/`:301`.** The route bound in the showcase is already a predicate over `Route`. The rule should be "no Route sort in Φ, but a closed projection set is allowed" (R2). Otherwise the showcase is ill-formed.
2. **With MIL/2 placing RFQ with n-party work (`:351`).** Funded RFQ and limit orders reduce to R4 under `|S| = 1` at U3. Only unfunded bilateral quotes and shared-price clearing truly need multi-signer admission.
3. **With the category review's framing of the pool as blocked on Φ₁ (`01-amm-exchange.md:13`).** Φ₁ gates *intent* predicates. The pool invariant belongs in the venue program (§3). The real blocker is the certified checked-multiply primitive at U1, or a declared narrow reserve width.
4. **With MIL/2 residue as the MEV answer (`:324`).** Surplus allocation is enforceable today in Φ₀. Only the ordering and disclosure *exposure* is legitimately residue-plus-declaration.
5. **Agreement kept:** the disjoint-footprint join is the wrong clearing carrier (`01-amm-exchange.md:19`, `DESIGN-MIL2.md:336`).

## 6. Remaining risks and assumptions

- **[assumption]** Midnight can invoke several contracts atomically in one transaction phase. If it cannot, R2 falls back to `hops = 1` per stage and exposes the owner to intermediate-asset risk under the signed `intermediate` clause.
- **[risk]** R4 widens what a single stage writes: other parties' escrow cells. Derived-footprint containment (O6, `.tex:252`) and K1 must cover consumed standing escrows, and the per-stage footprint cap of 32 (`:281`) must be measured with m = 1.
- **[risk]** `surplus to owner` does not make execution best. A solver can pick a worse venue and still meet the floor. Only `route.venues` restriction, an anchored reference-price guard, or competition limits this. Exact-input with an anchored price observation stays Φ₀ (`received × lit ≥ price.value × lit`) only if the input is a literal.
- **[risk]** Ordering and front-running exposure are not enforced by any of this. `completers exclusive` and private intent disclosure *may* reduce exposure on Midnight. **[inf, unverified]** That is a deployment property, not a proof.
- **[assumption]** The u64 reserve bound is acceptable for a first slice. Production pools will need the U1 limb gadget.
- **Comparative practice, not requirement [fact per source notes]:** Uniswap v2's fee-adjusted post-balance check and minimum-liquidity lock (`PRIMARY-SOURCE-NOTES.md:5`) inform P's library design. ERC-4626 rounding directions (`:6`) inform share conversions. Neither is a MIL/2 obligation, and neither shows how Moriarty behaves.