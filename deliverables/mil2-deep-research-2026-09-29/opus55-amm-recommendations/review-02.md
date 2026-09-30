# Opus 5.5 AMM recommendation: arithmetic and pool economics

**Scope.** This is a read-only design review. I read `DESIGN-MIL2.md`, the AMM category review, `MIL2-PROPOSED-SEMANTICS.tex`, `ROADMAP.md`, the product contract, the superseded MIL/1 map and the primary-source notes. I ran the guarded status check (`SP01.6`; dispatch blocked; this does not block the review). I ran one Python calculation to derive the test vectors in §4. Those numbers are arithmetic, not evidence about any Moriarty implementation.

## 1. Verdict

**Design inference:** MIL/2 is fit as the *trader envelope* for an AMM trade. It is not yet fit as an AMM language. The main problem is where the pool arithmetic lives, not how much arithmetic it allows. MIL/2 puts `sharesFor`, `assetsFor` and `mulDiv` into Φ₁, the intent predicate language (`DESIGN-MIL2.md:80-89`, `:137`, `:158-165`). The category review therefore marks the pool swap and LP rows as "deferred arithmetic" that waits for Φ₁ at U4 (`01-amm-exchange.md:13-14`).

**Recommendation:** A pool invariant is a *contract property* of the pool program. It is not a clause of a signed intent. The acceptance relation already separates contract correctness from intent refinement (`MORIARTY-PRODUCT-CONTRACT.md:39`). If pool arithmetic moves to the contract side with a small certified basis, a constant-product AMM becomes reachable in the U1 to U2 range. Φ₀ stays frozen at U0 and Φ₁ stays deferred, both unchanged.

**Source fact:** The obstacle is real at the Core level. Core `Mul` performs a "checked integer product in underlying type before any parent operation" (`experiments/moriarty-language/spec/successor/expression-signatures.json:905-921`). A u128 `FloorDiv(Mul(y, e), x + e)` therefore rejects with `ARITH_RANGE` whenever `y·e ≥ 2^128`, which is ordinary for real reserves. `FloorDiv`/`CeilDiv` exist with Euclidean semantics (`:942-1014`), but no fused wide operation does.

## 2. Top five ranked changes

### R1. A minimal certifiable pool arithmetic basis, verified rather than computed (core; U0 names it, U1 certifies it)

**Recommendation.** Add a *pool operand width* of **u126** to the U0 numeric profile. Pool arithmetic operands carry the precondition `v < 2^126`. `Qty` stays u128 everywhere else.

**Design inference.** For `a, b < 2^126` we have `a·b < 2^252`. The design records the BLS12-381 scalar field as 255 bits and `less_than` as stopping at 253 bits (`DESIGN-MIL2.md:169`). So a native field `mul` cannot wrap, and a 253-bit comparison decides the product exactly. With this precondition, pool arithmetic needs **no two-limb gadget**. The §4.5 limb rule stays in force for general u128 × u128 products and for Φ₁.

The basis is three certified primitives plus verify-relations built from them:

```
B1 range(k, v)            : v < 2^k,  k ∈ {126, 128}
B2 mulNF(a, b) = a·b      : pre a,b < 2^126  ⇒ product < 2^252, no wrap
B3 le253(u, v)            : pre u,v < 2^253
-- derived verify-relations (the witness supplies q, r; the circuit checks):
divFloor(n, d) = q  ⇔ d > 0 ∧ q·d ≤ n < q·d + d
divCeil(n, d)  = q  ⇔ d > 0 ∧ q·d ≥ n > q·d − d
isqrtFloor(n)  = s  ⇔ s·s ≤ n < (s+1)·(s+1)
```

**Counterexample addressed.** Take `y = 2^70` and `e = 2^60`. Current Core rejects this legitimate quote. Under B1 to B3 it is accepted, and every product stays below 2^253. In the other direction, if an operand is ≥ 2^126, B1 rejects before any `mul`. That closes the silent modular coercion the design forbids (`DESIGN-MIL2.md:169`).

**Placement.** U0 records the width, the preconditions and each relation's rounding direction (`ROADMAP.md:21` requires "per-primitive rounding direction"). U1 certifies B1 to B3 and the three verify-relations, with soundness against adversarial witnesses (`MORIARTY-PRODUCT-CONTRACT.md:31`). The derived relations are jet-style compositions with correspondence obligations (`:27`).

### R2. One constant-product acceptance rule for both exact input and exact output, with the fee explicit (pool program; U2)

**Recommendation.** The pool's stage checks an inequality. It does not recompute the quote.

```
CPSwap(pool P, fee f/F, pre (x, y), in dx of A, out dy of B):
  x > 0 ∧ y > 0 ∧ 0 < dy < y ∧ dx > 0           -- zero-liquidity and no-op trades reject
  φ = divCeil(dx·f, F)          -- fee rounded UP, retained in pool
  e = dx − φ                     -- checked subtraction
  x' = x + dx < 2^126 ; y' = y − dy
  (x + e)·y' ≥ x·y               -- B2 + B3
  post(pool) = (x', y') ; effects: A trader→P dx, B P→recipient dy ; fee descriptor φ
```

- **Source fact (comparative practice only):** Uniswap v2 checks fee-adjusted post balances against pre reserves (`PRIMARY-SOURCE-NOTES.md:5`).
- **Design inference:** Using an explicit `φ = ⌈dx·f/F⌉` removes v2's `F²` scaling, so both products fit R1's bound. Because `φ ≥ dx·f/F`, the check is at least as strict as the exact rational rule, so it favors the pool.
- **Exact input vs exact output.** They differ only in *which leg the signed intent fixes*. The pool rule is identical for both. The quotes are solver or library functions and never enter acceptance:
  - exact input: `dy = ⌊y·e/(x+e)⌋`
  - exact output: `e* = ⌈x·dy/(y−dy)⌉`, then `dx = ⌈e*·F/(F−f)⌉`

**Counterexamples addressed.**
- A solver claims `dy` one unit above the floor quote. The inequality fails.
- A quote that uses floor for the exact-output input leg underpays the pool by one unit per trade. The inequality fails.
- The legacy example's fixture-pinned output (`01-amm-exchange.md:13`) becomes a general transition.
- `R1-amm-exchange.md:180` records "no exact-output example … no statement of which direction each leg must round". This rule answers that.

**Placement.** U2 authors CPSwap as the "structurally contrasting program" (`ROADMAP.md:23`). The trader is the only signer and the pool is a program, so the `|S|=1` boundary is kept (`MIL2-PROPOSED-SEMANTICS.tex:59`).

### R3. Pool-owned reserves, a nominal share asset and directed-rounding LP rules (core semantics plus library; U0 tags, U6 conformance)

**Recommendation, three core edits:**

1. **Reserve cells are writable only by the pool program's transitions.** An unsolicited transfer lands in a separate `unsolicited(P, asset)` cell that never enters pricing.
2. **`Share<P,C>` elaborates to `Qty<shareAsset(P,C)>`.** This is a nominal `AssetId` with `repr = share`, and its `issue` right is held by the pool program. This resolves the mismatch the review found between `Share<P,C>` and `issue`, which is scoped to `(domain, asset)` (`01-amm-exchange.md:14`; `DESIGN-MIL2.md:189-193`). Shares are supply and never debt, so debt stays separate from supply.
3. **LP rules, with the role written next to each rounding:**

```
Bootstrap (S = 0): s0 = isqrtFloor(dx·dy); require s0 > MIN;
                   mint MIN to lock(P) (counted in S), s0−MIN to depositor
Mint (S > 0):  s  = min(divFloor(dx·S, x), divFloor(dy·S, y))   -- shares DOWN
               ux = divCeil(s·x, S) ≤ dx ; uy = divCeil(s·y, S) ≤ dy  -- inputs UP; only ux, uy move
               require s > 0
Burn:          ox = divFloor(s·x, S) ; oy = divFloor(s·y, S)     -- outputs DOWN
               require ox + oy > 0
LP safety (mint, burn, swap):  x'·S ≥ x·S' ∧ y'·S ≥ y·S'   -- per-share reserves never fall
```

**Design inference:** `s ≤ ⌊dx·S/x⌋` implies `⌈s·x/S⌉ ≤ dx`, so the only-take-what-is-used rule is always satisfiable. Every rounding remainder stays in the pool, which matches the design's `retained-in-pool` rule (`DESIGN-MIL2.md:89`) and benefits the LPs.

**Counterexample addressed: first-depositor inflation.** An attacker mints a small position, then donates reserves so that a victim's deposit floors to zero shares.
- The donation path is removed by edit 1.
- A victim deposit that would floor to zero shares rejects (`s > 0`) instead of silently taking the assets.
- MIN lock makes the attacker pay for any residual inflation path. **Source fact:** v2 permanently locks a minimum share amount (`PRIMARY-SOURCE-NOTES.md:5`).

**Recommendation on offsets.** Admit the bootstrap tag `isqrtLocked(MIN)` for two-asset pools. Allow `virtualOffset` only for one-asset vault conversions. Virtual reserves inside a constant-product pool shift its marginal price. See §5.

**Placement.** U0 adds the bootstrap-policy tag and the reserve-cell authority rule to the cell vocabulary (`DESIGN-MIL2.md:239-243`). U6 owns mint/burn conformance, since the roadmap assigns AMM libraries there (`ROADMAP.md:27`).

### R4. A rounding-role table and fee attribution in the U0 numeric profile (core; U0)

**Recommendation.** U0 must freeze a table rather than prose:

| Role | Direction | Beneficiary |
|---|---|---|
| swap output, burn outputs, minted shares | floor | pool / existing LPs |
| swap input quote, mint inputs used, fee φ | ceil | pool / existing LPs |
| bootstrap shares | isqrtFloor − MIN | locked |
| vault `sharesFor` / `assetsFor` | floor, per ERC-4626-style operation | vault |

**Fee attribution.** A pool stage emits the fee descriptor `φ`. An intent's `fees ≤ c` budget counts **explicit solver fees plus venue fees** by default.

**Counterexample addressed.** `AcquireB` signs `fees <= 1 A` (`DESIGN-MIL2.md:299`). Take a three-hop route through 1% pools with a 0.9 A solver fee. Counting only explicit fee lines gives 0.9 A, which is accepted. Counting venue fees gives about 1.23 A, which is rejected. The repository rule is that "fees count against net goals" (`AGENTS.md:83`). An intent may opt into `fees scope explicit`, but that choice is signed, never defaulted.

**Placement.** U0 (`ROADMAP.md:21`).

### R5. Reserve, but do not admit, a single wide-comparison atom (U0 header tag; admit no earlier than U2)

**Recommendation.** Reserve the Φ tag `t₁ ⊗ t₂ ≤ t₃ ⊗ t₄`, with operands under R1's u126 precondition. It is not admitted at U0. The trader intent needs it only for price impact against *live* reserves, `dy·x·100 ≥ 99·y·dx`, which is variable × variable. A signed minimum output does not bound that (`01-amm-exchange.md:15`).

**Design inference:** §4.4's cost argument concerns the *authoring-time* SMT check (`DESIGN-MIL2.md:163`), not in-circuit evaluation, which is R1-cheap. The escrow authoring check can treat each `⊗` atom as an uninterpreted Boolean. That is sound, because validity with the atoms abstracted implies concrete validity, and it fails closed on incompleteness.

**Justified change.** This does not move Φ₁. It reserves a header slot now because a slot is costly to add once U0 hash-binds the header (`DESIGN-MIL2.md:262`, `:332`). The atom may be admitted only after R1 is certified.

## 3. Library versus core

**Core** (semantics plus certificates; cannot be supplied by a library):
- B1 to B3 and the verify-relations `divFloor`, `divCeil`, `isqrtFloor`
- pool operand width
- rounding-role table and fee attribution
- reserve-cell write authority and the `unsolicited` cell
- share-asset elaboration and pool-held `issue`
- zero-output rejection
- the bootstrap-policy tag set
- the reserved `⊗` tag

A library cannot create write authority or certify non-wrapping arithmetic.

**Library:**
- quote functions (exact input, exact output)
- CPSwap and the mint/burn programs themselves (they are programs over core primitives)
- fee-tier choice (one immutable `f/F` per pool instance, bound in the pool's policy digest)
- routers
- protocol-fee mint
- TWAP, which also needs bounded observation collections (`DESIGN-MIL2.md:360`)
- one-asset vaults
- weighted and stable pools, which need later power/iteration profiles (`PRIMARY-SOURCE-NOTES.md:7`)

## 4. Smallest implementable slice and evidence pair

**Slice.** One stage on one domain with one signer. Exact-input `CPSwap` against an existing pool whose head is an authenticated legitimate genesis. Fee `30/10000`. Needs only B1 to B3 and `divCeil`. Mint/burn is the second slice.

**Positive vector (my own calculation, specified only).** Reserves `x = 1,000,000,000 A`, `y = 2,000,000,000 B` in smallest units at 6 decimals. Input `dx = 11,000,000`, which is AcquireB's 11 A.

- `φ = 33,000`
- `e = 10,967,000`
- `dy = 21,696,059`

`(x+e)(y−dy) ≥ xy` holds. The intent also holds: gross 11 A, fee 0.033 A ≤ 1 A, net 21.696 B ≥ 20 B.

A wide companion case forces a large product: `x = 2^100`, `y = 3·2^100`, `dx = 2^90`. Here `x·y` is 202 bits, so this legitimate quote exceeds u128 and today's Core `Mul` rejects it; B1 to B3 accept it.

**Hostile vector.** Same pre-state, but the proof claims `dy = 21,696,060` (one unit more). It must reject at the inequality with a specific code, and the envelope must be otherwise valid (`ROADMAP.md:40`).

Further hostile checks:
- `φ = 32,999` (fee not rounded up) must reject.
- `x' ≥ 2^126` must reject with `ARITH_RANGE` before any `mul`.
- **Exact-output twin:** for `dy = 20,000,000`, `dx = 10,131,406` is accepted and `dx = 10,131,405` fails. This shows the ceiling input quote is minimal.

## 5. Disagreements

1. **With the category review:** Φ₁ is not the blocker for a pool swap or for LP shares (`01-amm-exchange.md:13-14`, `:25`). Those are contract-side computations. **Recommendation:** remove `sharesFor`, `assetsFor` and `mulDiv` from the Φ grammar (`DESIGN-MIL2.md:137`). Keep them as Core program operations. Φ₁ is left for genuine intent predicates over variable products.
2. **With MIL/2 §3.3:** "virtual offset or first-depositor convention" (`DESIGN-MIL2.md:89`) should not be a free choice for two-asset constant-product pools. **Design inference:** virtual reserves change the pricing curve. Restrict offsets to one-asset vaults.
3. **With the category review's evidence order** (`01-amm-exchange.md:37`, step 2): putting bootstrap, fee, exact input and exact output into one library contract is too big a first step. Exact-input swap alone exercises the full basis. Exact output reuses the same pool rule, so it adds no new core surface.
4. **Refinement, not disagreement:** the §4.5 limb rule stays for general u128 arithmetic. R1 makes it unnecessary on the AMM path.

## 6. Remaining risks and assumptions

- **Assumption:** the 255-bit field and 253-bit `less_than` figures come from `DESIGN-MIL2.md:169`. I did not re-check them against the pinned ZKIR surface. R1 depends on them, and U1 must measure them.
- **Assumption:** the pool pre-reserves are an authenticated ledger read bound in the stage statement. Under concurrency, a proof bound to a stale reserve head should fail closed. Whether Midnight re-reads reserves at application time is unverified here. If it does not, a busy pool needs re-proving under the same signed intent. Holes allow that, but liveness is not established.
- **Risk:** the u126 cap excludes assets whose realistic reserves exceed about 8.5·10^37 smallest units. Such pools would need the limb rule.
- **Risk:** dust swaps (for example `dx = 1`) pay a 100% fee because the fee rounds up. The zero-output rule and the trader's minimum output catch these, but interfaces should warn.
- **Open:** a minimum output does not address sandwich/MEV and surplus allocation (`01-amm-exchange.md:21`). R5 only reserves a price-impact atom.
- **Open:** protocol fees, fee-growth for ranges, stable/weighted families and concentrated liquidity stay outside this basis (`DESIGN-MIL2.md:338`).
- **Comparative practice is not a requirement.** Uniswap v2 and ERC-4626 behaviors (`PRIMARY-SOURCE-NOTES.md:5-6`) informed R2 and R3. They do not establish Moriarty behavior. The rules above remain proposals until U1 certificates and U2 native evidence exist.