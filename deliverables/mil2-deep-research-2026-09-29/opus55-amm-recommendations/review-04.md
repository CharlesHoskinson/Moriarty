# AMM recommendation: compiler and native feasibility (Opus 5.5, independent lens 4)

Startup: I loaded the develop skill and ran `status --json`. It reports that SP01.6 dispatch is blocked because of stale inputs. That block does not apply to this read-only review. I edited, compiled and proved nothing.

## 1. Verdict

**Design inference:** MIL/2 is fit to act as the *trader-side envelope* for one exact-input swap. It is not yet fit as an AMM design, and the main reason is where it puts pool arithmetic, not that the arithmetic is missing. MIL/2 files pool math under Φ₁, the signed predicate language (`DESIGN-MIL2.md:89`, `:158-165`). Φ₁ is deferred to U4 (`:350`). That merges two different questions:

- **Signed predicates (Φ)** need an *authoring-time* decision procedure. Examples are the escrow exhaustiveness check (`:218`) and the refinement lint. The measured SMT blow-up cost (`:163`) is a cost of that static reasoning.
- **Contract transition arithmetic** such as `x'·y' ≥ x·y` is checked *per concrete witness* inside the circuit. Nobody quantifies over it, so it needs no solver. Its only real limits are field width and a witnessed division.

**Source fact:** ZKIR v3 has `Mul`, `LessThan(a,b,n)` and `ConstrainBits(v,n)` with `n < FR_BITS = 255`. It has no division instruction (`source-text/zkir-v3-spec.md:295-305`, `:498`, `:572-583`). So floor division has to be a witnessed quotient and remainder, bound by range checks.

**Recommendation:** Keep Φ₁ deferred for *signed intents*. Admit one certified, fixed-function pool transition as a **protected operation** at U1/U2, under a narrow width precondition. This gives the U2 path an AMM stage without pulling SMT into the authoring trusted base.

## 2. Top five changes, ranked

### C1. Move pool arithmetic out of Φ and into certified protected operations

**Edit:** In `DESIGN-MIL2.md:89`, replace "These are Φ₁ terms" with this rule:

> `sharesFor`, `assetsFor` and constant-product pricing are **protected operations** of a pool program, each with a primitive certificate. They are not Φ terms. A signed intent may constrain only their *effects* (debit, receipt, fee lines), in Φ₀.

The transition sketch below is a recommendation. Here x and y are the pool's A and B balances, and γ/1000 is the fee tier.

```
PoolSwapExactIn(p, A→B, dx; x, y, γ)
  pre   x = balance(d,pool(p),A), y = balance(d,pool(p),B)   -- authenticated reads
        dx > 0, x,y,dx < 2^112, γ ∈ policy(p).tiers, γ < 2^10
  wit   q, r                                  -- ConstrainBits(q,112), ConstrainBits(r,124)
  num   = dx·γ·y          den = x·1000 + dx·γ         -- all < 2^234
  check q·den + r = num ∧ r < den             -- floor, receipt role
  post  balance(pool,A) += dx; balance(pool,B) −= q;
        balance(trader,A) −= dx; balance(trader,B) += q
  fee   feeLine(A, dx − ⌊dx·γ/1000⌋, beneficiary = pool(p))
```

- **Counterexample addressed:** Under current MIL/2, the U2 AMM stage cannot be stated before U4, because `x'y' ≥ xy` is Φ₁ (category review `01-amm-exchange.md:13`, `:25`). The only swap that exists today is fixture-pinned (`u0-study.../R1-amm-exchange.md:169`).
- **Why this keeps the boundaries (design inference):** It stays inside the "small semantic basis" and jet-certificate model (`MORIARTY-PRODUCT-CONTRACT.md:27`, `:31`). Φ₀ is unchanged. No host Boolean is introduced: the equality and range facts are native constraints.
- **Placement:** U0 names the operation and its numeric-role policy. U1 certifies `witnessedFloorDiv`, `witnessedCeilDiv` and the bounded product compare. U2 runs one pool swap stage.

### C2. A pool width profile that removes limbs from the first slice

**Edit:** Add to §4.5 (`:167-171`):

> A pool program declares `reserveWidth ≤ 112` and `feeWidth ≤ 10`. Deposits and swaps whose post-balance exceeds `reserveWidth` return `Reject(ARITH_RANGE)`. Every intermediate product has a statically computed bound below 2^248. If a pool's declared widths exceed that bound, the compiler must emit the limb gadget or reject.

- **Counterexample addressed:** With u128 reserves, `dx·γ·y` needs about 266 bits. `Mul` computes `a ⊗ b` in the field (`zkir-v3-spec.md:579`), so the value wraps silently, and a wrapped product can make a false invariant check pass. This is the ZR-class failure that `:169` already forbids.
- **Arithmetic check (by construction):** 112 + 10 + 112 = 234 bits. That is below the `LessThan` bound (bits < 255) and below 248, so it leaves room for the `DivModPowerOfTwo` and `ReconstituteField` paths.
- **Comparative practice:** Uniswap v2 stores reserves as uint112. I know this from the protocol code, not from the corpus notes (`PRIMARY-SOURCE-NOTES.md:5` only points to that file). It is precedent, not a MIL/2 requirement.
- **Placement:** U0 (numeric profile). U1 certifies the bound-derivation pass.

### C3. Reserves are custody balances, and the LP fee is an explicit effect line

**Edit:** Add to §9 and §10 (`:236-258`):

> `reserve(p,a) ≡ balance(d, poolAcct(p), a)`. There is no separate reserve cache. The `pool(p)` cell holds only immutable parameters (asset pair, fee tier, width, bootstrap rule) and the share asset id. Every pool operation emits `feeLine(asset, amount, beneficiary)`. The signed `fees ≤ …` bound quantifies over fee lines. Exact-in output rounds **floor** (receipt). Exact-out input rounds **ceil** (obligation). The remainder beneficiary is `poolAcct(p)`.

- **Counterexamples addressed:**
  1. If the fee is implicit in price, the `AcquireB` `fees <= 1 A` cap (`:299`) is vacuous for a pool trade. A solver can route through a 1% tier and stay under the cap.
  2. A cached reserve that diverges from actual balances (by donation, or through sync or skim in the comparative v2 design) lets the invariant be checked against a number the ledger does not hold.
  3. The numeric profile's `reserveMechanism` is marked `absent`, so every directed rounding step lacks a remainder recipient (`R1-amm-exchange.md:176`). The pool account supplies that recipient for these primitives. **Design inference.**
- **Placement:** U0 (effect schema and rounding roles). U2 (native readback).

### C4. A stage public-statement schema for a pool stage, with authenticated reads and a budget head

**Edit:** Specialise `MIL2-PROPOSED-SEMANTICS.tex:239-240` for the pool operation. The locus column is my recommendation. The premises it rests on are marked.

| Field | Locus |
|---|---|
| semantic/numeric version, program and circuit id, `poolId`, domain | public input, bound to the verifier key |
| `intentDigest` (Poseidon, `:263`) | public input. The ledger checks the signer's signature over it (**premise**: no in-circuit Ed25519, `:264`) |
| `x, y` pre-balances | **public-transcript reads** (`PublicInput`, `zkir-v3-spec.md:585`), validated by the ledger against contract state (**premise**: Midnight ledger transcript semantics, not in the corpus) |
| budget head `(authId, spent)`, replay id | transcript read and write. Enforces `spent + dx ≤ 11 A` and `fees ≤ 1 A` natively |
| `dx`, `q`, fee line, four balance deltas | `Impact` outputs (`:589`) and ledger writes. Readback compares them against the prepared descriptor |
| `q, r`, branch bits | private witnesses, each with `ConstrainBits` or `ConstrainToBoolean` (`DESIGN-MIL2.md:265`) |

**Rule:** Any value that feeds a financial effect is a public-transcript read, a public input, or a range-constrained witness that a native equality ties to one of those two. A value that is only `PrivateInput` with no such tie is a compile error.

- **Counterexample addressed:** If `x, y` are private witnesses, a prover can supply `y = 10^30` and the `q·den + r = num` check still passes, which drains the pool. This is also exactly the anchoring-by-private-tag forgery that `DESIGN-MIL2.md:185` warns about.
- **Witness premises (source fact):** ZKIR statement soundness assumes producer and `WShape` conditions (`MORIARTY-PRODUCT-CONTRACT.md:29`, `MIL2-PROPOSED-SEMANTICS.tex:240`). The compiler must discharge the range conditions for adversarial witnesses, not just honest ones.
- **Placement:** U0 (schema). U2 (native readback plus hostile controls).

### C5. Route semantics: one hop is one stage until the `trace` extension

**Edit:** In §8 (`:223-224`):

> A `Route` completion lowers to a bounded Episode of single-pool stages, each with its own C4 statement. Intermediate assets are held in intent-owned escrow. The Episode head accumulates `cumulativeGrossDebit` and `cumulativeFees` (`:46`). The intent must state a permitted intermediate outcome (hold, refund policy, or `priority`) for each hop boundary.

- **Counterexample addressed:** `route.hops <= 3` (`:301`) currently has no effect semantics. If a two-pool route runs inside one stage, that is an internal trace, which decision 2 excludes (`:332`). If it runs across stages, hop 1 can commit while hop 2 goes stale. There is no global rollback (`MORIARTY-PRODUCT-CONTRACT.md:82`), so the trader holds the intermediate asset and nothing signed says whether that is acceptable.
- **Placement:** U0 (lowering rule). U2 (hops = 1). U3 (multi-hop Episode with an intermediate hold).

## 3. What can be a library and what must be core

**Must be core (recommendation):**
- the protected-operation form with its certificate slot (C1)
- the width-bound derivation and the rejection on overflow (C2)
- reserve-as-balance, fee lines and role rounding (C3)
- the statement and transcript binding rule (C4)
- route-to-Episode lowering (C5)
- the share mapping below

**Share mapping:** `Share<P,C>` should elaborate to a nominal `AssetId` whose `issue` right is held only by the program of pool P, with `totalShares ≡ supply(d, shareAsset)`. This resolves the mismatch between the `Share` sort and an `issue` right scoped by `(domain, asset)` (`01-amm-exchange.md:14`). It also stops `totalShares` from drifting away from the sum of holder balances. Share supply is supply; debt stays separate (`MORIARTY-PRODUCT-CONTRACT.md:43`).

**Library (U6):**
- concrete CPMM pool contracts and the fee-tier registry values
- the bootstrap convention (minimum locked shares or a virtual offset), which each pool declares (`:89`)
- proportional deposit and burn, built on the certified `floorDiv` and `min`
- route search and solver heuristics
- TWAP once bounded observation collections exist (`:360`)

**Later profile, not library:** weighted pools (powers and roots, `PRIMARY-SOURCE-NOTES.md:7`), stable-swap iteration, concentrated liquidity (`:338`), and multi-signer RFQ or batch clearing (`:336`).

## 4. Smallest implementable slice and its evidence pair

**Slice (recommendation, U1 → U2):** One fixed-parameter CPMM pool on `midnight.preview`. Its reserves are the pool account's A and B balances, with widths ≤ 112 and γ = 997. The trader signs `AcquireB` with a single hop, and the footprint is corrected to include all four balance cells plus `allowance`, `replay` and `pool(p)`. That correction also repairs the showcase failure at `MIL2-PROPOSED-SEMANTICS.tex:218-227`.

- **U1 certifies:** checked u128 add/sub, `LessThan` at ≤ 248 bits, `witnessedFloorDiv` (completeness for honest q, r; soundness against any other q, r), and a cost relation for all of these.
- **U2 shows:** actual pinned ZKIR, native verification and complete effect readback.
- **No LP mint or burn in this slice.** The pool is seeded at deployment, and that seeding is stated as a trusted genesis premise.

**Positive control (arithmetic checked with a local Python computation; no protocol behaviour was run):**
- Pool: x = 10⁹ (1000 A), y = 2.2·10⁹ (2200 B). Trade: dx = 1.1·10⁷ (11 A).
- num = dx·997·y, den = x·1000 + dx·997.
- q = 23,865,665 (23.87 B ≥ 20 B), r = 251,945,000,000 < den.
- Fee line = 33,000 units (0.033 A ≤ 1 A). Gross debit = 11 A ≤ 11 A.
- Fee-adjusted invariant `(x'·1000 − 3·dx)·y' ≥ x·y·1000` holds.
- **Expected result:** accepted, with four balance deltas, one fee line, the budget head incremented by 11 A and the replay id consumed.

**Hostile control:** The same statement with `q' = q + 1`, which pays the trader 1 unit more than floor. Two cases:
- The honest remainder becomes r − den < 0. No `r' < den` in range satisfies `q'·den + r' = num`.
- A prover who drops the range check on r' could satisfy the equation with a field-wrapped r'. The certificate must therefore show that `ConstrainBits(r',124)` excludes it.

The independent invariant check also fails at q + 1 (computed). **Expected result:** native proof generation or verification rejects. Rejecting because of a malformed envelope does *not* count (`ROADMAP.md:40`).

**Second hostile control (recommended):** a stale pre-balance, where the transcript `y` differs from ledger state. It must reject at the ledger, and I label that expectation a **premise** until it is observed.

## 5. Disagreements

1. **With MIL/2 (`:89`, `:137`, `:350`):** Pool arithmetic does not belong in Φ₁ or U4. Per-witness nonlinear checks are cheap to decide and are bounded by width, not by a solver. Only *signed* variable products need Φ₁. This changes the milestone placement, not the Φ₀/Φ₁ decision itself.
2. **With the category review (`01-amm-exchange.md:35-37`):** It asks for a "reusable pool transition" as a *library contract* first. I recommend the reverse order: certify the core protected-operation form and the width rule first, because the library cannot be correct without them. Exact-out swaps can follow in U1 at little cost (`witnessedCeilDiv`, reject when `dy ≥ y`). They should not be deferred behind a library.
3. **With `DESIGN-MIL2.md:301`:** A hop cap without hop semantics should not be in the showcase until C5 is adopted.
4. **With MIL/1 (`CATEGORY-MAP.md:30-34`, superseded):** It called `x·y ≥ k` "Expressible" through cross-multiplication. Neither the width nor the field-wrap question was addressed there.

## 6. Remaining risks and assumptions

- **Premise (unverified):** Midnight ledger checks the contract's public transcript reads and writes against current state atomically. C4 depends on this. It must be confirmed against the pinned ledger at U0/U1.
- **Premise:** The signature over `intentDigest` is checked by the ledger or the wallet, not in the circuit (`:264`). U1 must measure this.
- **Design inference:** Every swap writes the pool account, so throughput is one swap per pool per ledger head. Concurrent traders serialise, which is the same contention that makes n-party clearing core work (`:336`). Min-out protects each trader but does not help liveness.
- **Unmeasured cost:** About 6 `Mul`, 3–4 wide range checks and around 10 transcript fields per stage. No row counts are claimed. Given the R3 k17 exhaustion (`AGENTS.md:79`), U1 must measure this before U2 commits.
- **Economic gaps that remain open and are out of this lens:** price-impact bounds against pool depth, surplus allocation and ordering or MEV. The `residue` clause (`:324`) still carries them.
- **The 112-bit cap is a product limit.** Pools with more than 2^112 smallest units need the limb gadget, which is deferred.
- **No claim here** that any paper, protocol source or graph edge establishes Moriarty behaviour. All transitions above are specified-only.