# MIL/2 against the AMM and exchange category: core semantics and typing recommendations

**Scope and status.** This is a read-only review. I loaded the develop skill and ran `status --json`, which reports SP01.6 dispatch as blocked. That block does not apply to this task. Nothing here is implemented or proved. Every arithmetic value below is my own hand calculation and has not been executed in any repository tool.

## 1. Verdict

**Design inference: MIL/2 is fit to express a trader's intent, but it cannot yet express a pool, and its type system has gaps that would let a prover take value from LPs.**

The bounded envelope works: nominal assets, `Qty` in smallest units, separate gross/fee/net limits and a Φ₀ limit price (`DESIGN-MIL2.md:53-78,158-160,299`). The AMM failure is structural, not only arithmetic:

- **The stage relation has no clause for the program's own transition rule.** `Stage` in `MIL2-PROPOSED-SEMANTICS.tex:131-137` checks intent, evidence, effects, conservation, liabilities, locks, footprint, history and failure. Nothing requires the pool program's rule to hold. The product contract, by contrast, requires contract correctness to be verified along with intent refinement (`MORIARTY-PRODUCT-CONTRACT.md:37-39`).
- **The semantic state has no pool or share cells.** The state definition (`tex:64`) omits pools, shares and positions, although the cell vocabulary has them (`DESIGN-MIL2.md:239-242`).
- **Pool arithmetic is typed loosely.** `sharesFor`/`assetsFor` take a runtime `Rounding` argument and an unconstrained asset index (`DESIGN-MIL2.md:85-86`), yet the section is tagged `[checked]`.
- **`pre`/`post` are admitted everywhere in Φ** (`DESIGN-MIL2.md:130,154`). The semantics draft restricts `post` to `Ensure` (`tex:83`), and so does source/5 (`static-semantics.md:96-98`).
- **The category review's invariant `x'·y' ≥ x·y` (`01-amm-exchange.md:13`) is too weak.** In the vector in §4 it lets a prover keep the whole LP fee.

**Recommendation:** fix these in U0 as schema and typing changes. Put the constant-product arithmetic into certified ground operations (U1) and the program's `ensures`, not into Φ₁. The first AMM stage can then land at U2 without admitting Φ₁ or multiple signers.

## 2. Top five changes

### Change 1 — Add `ProgramValid` to `Stage` and treat the pool as program custody (U0 schema; enforced from U2)

**Rule (recommendation):**
```
Stage(I,σ,P,s,o,e,s') ≜ … ∧ ProgramValid(P,s,o,e,s')
ProgramValid ≜ Core_P(pre=s|R, args, obs) ⇓ ExpressionPrepared(post, desc, _, L)
             ∧ e = Effects(desc) ∧ s' = s ⊕ e ∧ every Ensure_P holds on (s, s')
```

- **What it binds:** it makes the chain `FinancialPrepared → StageCandidate` (`tex:159-166`) part of acceptance, so the program's effects must equal the stage's effects exactly.
- **Map to U0:** the U0 `stage` key already covers "contract properties" (`tex:145`). Add this clause to that key rather than creating a seventh key.
- **Counterexample addressed:** Alice signs `net ≥ 19,000 B`. A prover submits `dy = 19,801` against the vector in §4. `SignedBound`, `Conserve` and `GuardsTrue` all pass, and nothing checks the pool's rule, so 58 B of LP value leaves the pool.
- **Single-signer boundary (design inference):** the pool authorizes through its program, not a signature. An AMM swap therefore stays inside `|S| = 1` (`DESIGN-MIL2.md:336`), and no change to the first-profile signer boundary is needed.

### Change 2 — Evaluate ground nonlinear arithmetic in certified operations, separately from Φ₁; require a fee-adjusted invariant (U0 numeric profile; U1 certificates; U2 use)

**Design inference:** §4.4 deferred Φ₁ because of solver cost: the 128-bit division rlimit and the 256-bit timeout (`DESIGN-MIL2.md:163`). Those costs arise when an *authoring check* quantifies over unknowns. Evaluating a closed product at proof time is not a decision problem. It needs only the §4.5 two-limb gadget.

**Rule (recommendation):**
```
op ::= mulDivFloor(a,b,c) | mulDivCeil(a,b,c) | prodGe(a,b,c,d)   -- two-limb, certified jets
Positions: effect amounts, program Ensure, `let` in program bodies.
Forbidden: intent guards, hole bounds, and any formula given to an authoring check (those remain Φ₁, U4).
```

CPMM exact-input as a library program (fee γ = γn/γd):
```
dy  = mulDivFloor(dx·γn, y, x·γd + dx·γn)                              -- receipt: floor
ensure prodGe(x'·γd − dx·(γd−γn), y'·γd, x·γd, y·γd)                   -- fee-adjusted invariant
```

- **Fee counting (recommendation):** the operation emits a `fee{role: lpFee, beneficiary: reserve(p), amount: dx − floor(dx·γn/γd)}` descriptor. This follows the rule that fees count against net goals (`AGENTS.md:83`).
- **Counterexample addressed:** the raw invariant accepts `dy = 19,801`. The fee-adjusted invariant rejects `19,744` (§4).
- **Why this is not a Φ₀/Φ₁ change:** Φ₀ is untouched, and the nonlinear-node cap of 0 still applies to Φ (`DESIGN-MIL2.md:276`).

### Change 3 — One account namespace; derive aliases; give cells modes (U0)

**Rule (recommendation):**
```
Acct ::= party(pk) | custody(escrowId) | reserve(poolId) | feeSink(poolId)
balance(d, Acct, AssetId)                    -- the only cell that holds tokens
escrow(E)  holds q_E and the tombstone only;  pool(p) holds {assets, γ, bootstrap, seq}
totalAssets(p,a) ≜ balance(d_p, reserve(p), a)   -- sugar, not a separate cell
mode : Cell → {linear, affine, shared-serial}
shared-serial c ∈ R ⇒ pre(c) is an authenticated read at the stage's ledger head,
                       and W ∋ pool(p).seq
Fork: Δ = Δ₁ ⊎ Δ₂ over linear/affine cells only. A shared-serial cell may occur in
      both branches only if each branch re-reads it at its own head.
```

**Counterexamples addressed:**
- **Alias divergence:** `transfer … from E` (`DESIGN-MIL2.md:313`) ill-types `E` as an account. The declared footprint `escrow(E)` also hides the balance writes (`tex:218-227`).
- **Split cells:** with separate `totalAssets` and `balance(reserve)` cells, E1 checks one cell while pricing reads the other. A donation or skim then makes them diverge.
- **Over-strict partition:** a linear partition of `pool(p)` forbids two independent episodes that each trade against the same pool. Without a mode, the pool escapes accounting entirely.
- **Stale concurrent reads:** two traders reading one `pre(pool)` are serialized by `seq`. The second is rejected and re-proved, which is outcome-neutral under a signed floor.

### Change 4 — Stratify `pre`/`post`; add a `net` term (U0)

**Rule (recommendation):**
```
Guard, hole bound, release_when/refund_when :  pre(c) only        -- post ⇒ TYPE_POST_SCOPE
Program Ensure, intent `ensures` block      :  pre(c), post(c), net(p,a)
Order: guards → Choose_priority → effects e → post = pre ⊕ e → ensures
post(c) for c ∉ W_decl is rejected (no implicit frame read)
net(party, a) : Delta<A> ≜ Σ_{e ∈ effects, asset a} signed(e, party)    -- not post − pre on Qty
```

- **Counterexample addressed:** `release_when post(balance(owner,B)) ≥ …` makes branch selection depend on the effects of the branch it selects. `net` also avoids underflow when `post − pre` on `Qty` goes negative.
- **`delivered` (recommendation):** this also fixes `delivered(B, ≥20 B, …)` in a guard. A same-stage delivery becomes `ensures net(owner,B) ≥ 20 B`. A prior-stage delivery becomes `consumed(receipt)` on a linear receipt cell.
- **Consistency:** this reconciles `DESIGN-MIL2.md:130,154` with `tex:83` and with source/5's rule.

### Change 5 — Make shares nominal assets; derive rounding from each role; tie asset indices to the pool (U0 typing; conversions in U2 library)

**Rule (recommendation):**
```
shareAsset(p,c) : AssetId, repr = pool-share(p,c); issue(d, shareAsset(p,c)) held only by P's program
Σ ⊢ pool p : {assets = {A,B}, classes = {c}}      A ∈ assets(p)
──────────────────────────────────────────────  (no Rounding argument)
mintExact(p, s) : (Qty<A>, Qty<B>)  = (mulDivCeil(s,x,S), mulDivCeil(s,y,S))     -- pool-favouring
burn(p, s)      : (Qty<A>, Qty<B>)  = (mulDivFloor(s,x,S), mulDivFloor(s,y,S))
S = 0 ⇒ declared bootstrap ∈ {virtualOffset(vs,va,vb), lockedFirstMint(m)}; else Reject(BOOTSTRAP_UNDECLARED)
```

- **Rounding (source fact):** the U0 numeric profile already records floor for receipts and ceil for obligations (`R1-amm-exchange.md:175`). The role table should drive rounding directly.
- **Counterexamples addressed:**
  - `sharesFor(qty of a foreign asset C, …)` currently type-checks.
  - An author can pass `ceil` to redeem.
  - A share mint currently falls outside E1 (`tex:171-177` quantifies over balances only). As a nominal asset, a share mint enters `Δsupply` and needs the pool-scoped `issue` right. That closes the share↔`issue` gap the category review raised (`01-amm-exchange.md:14`).
- **Two-asset mint:** the exact-shares form (`mintExact`) avoids a `min` over two ratios and has no donation remainder.

## 3. Library versus core

| Core (language, schema, certified basis) | Library (versioned programs over core) |
|---|---|
| `ProgramValid` clause; account namespace; cell modes and `seq`; `pre`/`post` stratification; `net` | CPMM exact-input and exact-output programs, γ values, fee tiers and registry |
| Two-limb `mulDivFloor/Ceil`, `prodGe` jets with certificates (U1) | Bootstrap choice per pool; LP mint and burn wrappers |
| Role→rounding table; fee-descriptor counting rule | Routers as route-hole effect sequences over pool operations (U3) |
| Share-as-asset identity and pool-scoped `issue` | RFQ as two single-signer stages: the maker funds a quote custody in their own stage, and Alice's stage consumes it (design inference; last look and cancellation still need a spec) |
| n-party clearing relation (post-U4) | TWAP (needs bounded observation collections, `DESIGN-MIL2.md:360`); stable and weighted pools (need bounded iteration) |

## 4. Smallest implementable slice and evidence pair

**Slice (recommendation, U2):**
- One `moriarty-intent/2` stage on one domain, with Alice as the only signer.
- One CPMM pool program with γ = 997/1000, exact input only.
- No escrow, no holes and no imported evidence.
- The pool's `pre(balance(reserve(p),·))` values are authenticated reads at the head.
- **Alice's intent:** `gross_debit ≤ 10,000 A`, `fees ≤ 1 A` (1,000,000 units at 6 decimals), and `ensures net(alice,B) ≥ 19,700` units.
- **Program:** the Change 2 formula plus `seq' = seq + 1`.

**Positive control:**
- **Inputs:** `x = 1,000,000`, `y = 2,000,000`, `dx = 10,000`.
- **Output:** `dy = floor(19,940,000,000,000 / 1,009,970,000) = 19,743`, with remainder 162,290,000.
- **Repository fact:** this matches the value recorded for the legacy formula (`R1-amm-exchange.md:169`). That record is not acceptance.
- **Effects:**
  - `A: alice −10,000, reserve +10,000`
  - `B: reserve −19,743, alice +19,743`
  - `fee{lpFee, 30 A}`
- **Invariant check:** the fee-adjusted left side is 1,009,970,000 · 1,980,257,000 = 2,000,000,162,290,000,000. That is ≥ 2·10¹⁸, so the stage accepts. The operands need about 138 bits each, so the product needs the limb gadget.

**Hostile control:**
- **Mutation:** keep everything the same but set `dy = 19,744`. This is a valid, feasible witness: conservation holds and Alice's floor holds.
- **Fee-adjusted invariant:** the left side becomes 1,999,999,152,320,000,000, which is below 2·10¹⁸. The stage must reject (ENSURES_FAILED).
- **Discrimination check:** under the raw `x'y' ≥ xy` (2,000,058,560,000 ≥ 2·10¹²) the same witness would be **accepted**. The raw invariant would accept anything up to `dy = 19,801`. So this pair tests the semantic rule, not a malformed envelope (`ROADMAP.md:40`).
- **Second hostile control:** a stale `pre` (an old `seq`) should also reject.

## 5. Disagreements

1. **With the category review (`01-amm-exchange.md:13,25`) and MIL/2 §15 (`DESIGN-MIL2.md:350`):** a CPMM stage does not need Φ₁ at U4. Φ₁ gates intent-level predicates over unknowns. The pool rule is a ground evaluation inside the program (recommendation).
2. **With the category review's invariant:** `x'·y' ≥ x·y` "with a declared fee" should specify the fee-adjusted form. As written, the raw form leaks the LP fee (vector above).
3. **With the RFQ verdict (`01-amm-exchange.md:18`):** a maker-funded custody stage removes the need for `|S| > 1` in basic RFQ. Last-look and quote-cancellation semantics stay open.
4. **With MIL/2's `[checked]` tags:**
   - §3.3 (`DESIGN-MIL2.md:89`) should be `[obligation]` until Change 5's typing is adopted.
   - §4.3's unrestricted `pre`/`post` conflicts with `tex:83`.
5. **With `tex:59`:** "vault conversion outside this profile" should read "outside Φ in this profile". The conversion can live in the program layer.

## 6. Remaining risks and assumptions

- **Assumption (unverified):** a Midnight contract can hold custody of both reserve assets, mint a share token, and expose reserves as authenticated reads inside the proof. Shielded and unshielded token representation may change this.
- **Unmeasured cost:** the two-limb gadget, the certificates and the per-stage row cost are all unmeasured. The R3 exhaustion at k17 (`AGENTS.md:79`) is a warning sign.
- **Contention:** `seq` serialization makes pool contention a liveness problem. Safety does not guarantee inclusion.
- **Surplus:** requiring formula equality removes solver discretion over surplus but forbids price improvement. Surplus and MEV allocation remain open (`DESIGN-MIL2.md:324`).
- **Stage size:** for the single hop and fixed γ of the §4 slice, footprint derivation stays within the §12 caps. A multi-hop route may exceed 16 effect lines or 32 cells.
- **Out of scope:** concentrated liquidity, hooks and flash liquidity remain declared out of scope (`DESIGN-MIL2.md:332,338`).
- **Status of these claims:** none of this establishes Moriarty behavior. Each rule above is a recommendation that needs TypeScript/K differentials and native hostile controls before it can be accepted.