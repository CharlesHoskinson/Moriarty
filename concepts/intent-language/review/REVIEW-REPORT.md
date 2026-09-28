# MIL/1 nine-reviewer PL review — combined report

**Date:** 2026-09-28
**Status:** specified-only. A review of a proposal. Nothing executed against a compiler, prover or ledger by any reviewer; nothing committed.

**Method.** Nine independent programming-language reviewers, three per lens, one per model family. P1 type system and semantics, P2 expressiveness and the category mapping, P3 compilation, proving and AI ergonomics. Claude Opus 5.5 via agents; GPT-6 Sol via `codex exec -m gpt-6-sol` at xhigh reasoning; Grok 4.7 via `grok -m grok-4.7 --reasoning-effort xhigh`, both read-only sandboxed. Briefs in [`briefs/`](briefs/), reviews in this directory, external logs in [`logs/`](logs/).

**Subject.** [DESIGN.md](../DESIGN.md) and [CATEGORY-MAP.md](../CATEGORY-MAP.md), against Moriarty `8f73784042bd692733c296d0d49f5173be96725e`. Paths under `lean/`, `algebra/`, `corpus50/` and `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` are **defiformal** at `8c5dd103`, marked *[df]*.

---

## 1. Headline

**Unanimous: the shape is right, the document is not freezable.** Nine of nine endorse the architecture — four layers, outcome-first intents, evidence as something a position can reject, debt kept off the supply equation, one conservation law per `(domain, asset)`, escrow as a signed construct, footprints as a per-operation obligation. Nine of nine also say it must not be hash-bound at U0 as written.

The failure is specific and it is the same failure in three places: **the design states theorems it has not checked.** Its three load-bearing formal claims — the anchoring rule as a typing rule, monotone completion via polarity, and the escrow exhaustiveness check — are each *false as written*, not merely incomplete. Two reviewers produced explicit counterexamples; a third showed the central claim is not even a well-formed sentence.

A second, blunter failure: **the surface examples are not in the language.** `failed`, `never`, `not`, `equity`, `collateral_value`, `settle`, `return`, `funded`, `exercised`, `deadline none`, `attested(...)` as a proposition, and `delivered(... on domain) via ...` all appear in DESIGN.md §10 or CATEGORY-MAP.md and none is a production of Φ. The design's own showcase does not parse, and then fails the design's own escrow rule.

## 2. Verdicts

| Reviewer | Verdict |
| --- | --- |
| Opus P1 | Sorts and shape survive; three load-bearing claims false. Freeze sorts, footprints, evidence label; move §6/§7 checks to U1 with a named procedure. |
| GPT-6 Sol P1 | Right direction, not sound enough to freeze. Completion theorem undefined; escrow rule proves neither exit nor entitlement; projections erase evidence. |
| Grok 4.7 P1 | Type layer not sound enough to freeze. Φ is an untyped grammar closed under neither definedness nor evidence. Freeze a sorted S0 fragment; replace §3.4, §6, §7. |
| Opus P2 | Layering and type additions right; expressiveness claim is not. MIL/1 states bounds on the signer's own outcome and nothing else. |
| GPT-6 Sol P2 | Would not freeze as the language contract. Several "Expressible" verdicts rely on operations the types cannot form. |
| Grok 4.7 P2 | Bones right, would not freeze. Concentrated liquidity cannot be said even after the obvious patches, and is absent from the open list. |
| Opus P3 | Direction right; two ideas non-retrofittable. No compilation story at all. Freezable after three edits, not before. |
| GPT-6 Sol P3 | Right boundary, not freezable as written. Freeze a versioned signed-condition format and its enforcement contract only. |
| Grok 4.7 P3 | Right shape, buildable as a specialized single-stage escrow. Three rules wrong. U0 hash-binds a grammar, widths, bounds, digest, loci — nothing more. |

## 3. Confirmed defects

Reviewer counts are how many of the nine raised the point independently. Items marked **[verified]** I checked myself against the files.

### D1 — The escrow meta-rule is wrong. 9/9. **[verified]**

`DESIGN.md:187` requires `release ∧ refund` unsatisfiable and `release ∨ refund ∨ after(deadline)` valid. Five independent objections, all correct:

- It **rejects the design's own showcase**: `delivered(B, ≥20) ∧ after(validity.end)` is a model of `release ∧ refund`.
- It **rejects the design's own lending escrow** (`refund never`, `deadline none`, `CATEGORY-MAP.md:44-50`). Locked collateral with no deadline is a normal product.
- Read as validity, it **forbids the ordinary waiting state** — funded, pre-deadline, nothing delivered.
- Read as satisfiability, `after(deadline)` makes it vacuously true, so exhaustiveness does no work.
- Unsatisfiability of `release ∧ refund` **forbids the late-success/refund race `ROADMAP.md:39` requires U3 to demonstrate**.

And it proves nothing it claims: a deadline is a clock fact, not an exit, not evidence of foreign non-execution, and not a refund entitlement — which is the controlling distinction at `MORIARTY-CONSOLIDATED-DESIGN.md:64`.

**Converged repair.** Replace the meta-rule with an executable escrow *transition relation*: a legitimate pending state; per-witness exclusivity enforced in-circuit by a constrained branch bit plus a ledger tombstone; an explicit late-race priority; and recovery viability stated separately as an acceptance obligation under named assumptions, not derived from a propositional formula. The authoring-time check becomes `after(deadline) ⇒ release ∨ refund`.

### D2 — Monotone completion is false, and not a sentence. 9/9.

Three distinct refutations:

- **Opus P1:** `refund after(validity.end) or (realized_loss ≥ h)` with `hole h ≤ 2_000_000 A`. `h` is antitone, so the check permits it; filling `h` at maximum passes every stated check and deletes the signer's early exit.
- **GPT-6 Sol P1:** `refund = after(h)` is antitone in `h` while removing the owner's earlier recovery opportunity. Antitonicity of a Boolean clause is not monotonicity of the signer's financial protection.
- **Grok 4.7 P3:** variance is sign-dependent. In `a·h ≤ b·c`, raising `h` tightens when `a > 0` and loosens when `a < 0`, and `Signed<A>` is in the language, so the syntactic walk is unsound.

Independently, `Φ_guarantee[σ] ⟹ Φ_guarantee` is **ill-formed**: the right-hand side still contains the hole, so it is not a closed obligation. `Route` and `Program` have no order and are not Φ sorts, so `failed(route)` is undenotable.

**Converged repair.** Define the denotation of an unfilled intent and require `Accepted(completion) ⊆ Accepted(signed template)`, checked per concrete filling. Keep polarity only as a conservative shortcut on a restricted fragment, with a declared operator-variance table that excludes signed factors.

### D3 — Evidence laundering: the anchoring rule is not a typing rule. 9/9.

The design's central honesty claim fails four ways:

- `anchored` is a **nullary constructor for a domain-relative property**. An observation labelled anchored in one domain crosses an Episode edge unchanged and satisfies `anchored(obs)` in another. Nothing re-labels it (Opus P1).
- **Every eliminator erases the label.** Evidence is a record field, not a type index, so `obs.value : T` drops it and `min`, `max`, `+`, `−`, `mulDiv` combine values that no longer carry provenance. Grok P1's concrete laundering program: `guarantee min(a.value, b.value) ≤ limit and anchored(a)` — satisfied, while the quantity inside the inequality is the imported `b` whenever `b` is smaller.
- **Holes carry no evidence label**, so a solver fills an anchored-required position from an imported feed.
- `anchored` is **both a typing rule and a Φ proposition** (Opus P3), and a private witness tag is forgeable anyway — it must be bound in-circuit to an actual authenticated ledger read (Grok P3, GPT P3).

**Converged repair.** Index the observation type by evidence *and* domain, propagate a source set through every operator (`Γ ⊢ e : T ! Sources`), label holes, and bind `anchored` in-circuit to a real authenticated read.

### D4 — Φ is not total, not typed, and not the language the examples use. 9/9. **[verified]**

- **No formation judgments.** `term ≤ term` does not require the same asset, clock or evidence; a NIGHT balance compared to a DUST balance is a well-formed proposition.
- **Not total:** `mulDiv(_, _, 0, _)`, checked subtraction underflow, `Instant + Duration` overflow, `k_of_n(0, _)`, and `fresh` read symmetrically so a feed can pre-timestamp into the future.
- **Missing atoms**, listed above — including `not`, while `DESIGN.md:185` derives the nine workflow states using `not terminal` and the options case needs `not exercised`. Bounded atomic negation is decidable and cheap; banning it costs expressiveness and buys no decidability.
- **No `pre`/`post`**, so the AMM invariant the map credits to "cross-multiplied comparison" has nothing to compare against.
- **No `totalAssets`**, and `mulDiv : Qty × Share × Share → Qty` types redemption but not issuance, so ERC-4626 deposit is unwritable and the empty-pool bootstrap has no definedness rule.
- `decimals` is unrelated to price scale, so an off-by-`10^decimals` intent is well-typed; `now` is used after `:55` declares a bare instant a type error.

### D5 — The decidability claim is wrong, with a precise ladder. 9/9.

"Φ is decidable" and "Φ satisfiability is decidable" are different claims, and §6 needs the second. With products and `mulDiv` over unbounded integers it is undecidable; over fixed width it is QF-BV, decidable but NEXPTIME-complete and in practice an SMT solver.

The repository already measured that cost: one 128-bit division obligation took bitvector `rlimit` **242,607,369** against **1,978** for unbounded `Int`, and at 256 bits the bitvector encoding **exceeded 600 seconds** (`openspec/changes/aeon-refinement-integration/design.md:27-31`).

Converged split: **Φ₀** without products — escrow patterns are quantifier-free difference logic, decided by negative-cycle detection in polynomial time; sums and `min`/`max` lift to QF-LIA, NP-complete but practical with capped case-splits. **Φ₁** with products — SMT, and expensive.

**A compile-time solver does not escape the trusted computing base** as the design has it, because nothing at runtime re-establishes exclusivity: a wrong `unsat` admits a permanently stuck escrow and the per-witness branch check never notices. It escapes only if the circuit enforces the property — a one-shot terminal tombstone, estimated under 100 rows.

### D6 — Cross-multiplied `u128` does not fit the field. 2/9, **[verified independently]**

The BLS12-381 scalar field is **255 bits**; the maximum `u128 × u128` product needs **256**. Lowering `a·d ≤ b·c` as `mul` then `less_than` is silent modular coercion, which `MORIARTY-CONSOLIDATED-DESIGN.md:52` forbids — and `less_than` stops at 253 bits anyway (`wiki/contradictions.md:116`). "A wider intermediate" is not a lowering; it is a two-limb gadget with a cost nobody has bounded.

**This defect is inherited.** `UNIFIED-PROPOSAL.md` specifies the U3 discriminator's ratio limits as cross-multiplied UInt256 inequalities. It needs the same repair there.

### D7 — The footprint cell vocabulary defeats the linearity it was meant to support. 4/9. **[verified]**

`Cell` omits receipts, encumbrances, authority allowance, positions, share balances, observations and pools. So two Episode branches consuming the *same receipt* have disjoint declared footprints and are certified independent. The disjointness rule breaks the substructural story §3.6 introduces.

Compounding it, footprints are **declared only**. *[df]* `lean/DefiKernel/Typed/Transition.lean:80,121` derives required reads from guards and effect expressions and then checks the declaration against them; MIL/1's declared-only check is strictly weaker. A fork also needs a linear-context partition `Δ = Δ₁ ⊎ Δ₂`, and affine authority needs a shared non-duplicable budget across branches.

### D8 — The §12 fallback is unsound. 3/3 on P3.

Committing to the whole tree while evaluating "only the clauses that bind effects" is not a weaker evaluation, it is a **different relation**. Freshness, `anchored`, authority windows and hole-domain restrictions all bind what may happen without being effect lines; drop them and a satisfying witness can be stale, imported-presented-as-anchored, or off-domain. A negative control that mutates only the evaluated subset cannot detect the omission. Delete the fallback, or require a compiler-checked dependency closure with a proof that the evaluated result equals the full signed predicate.

### D9 — The Episode must be ledger-linked, and joins are the U4 risk. 3/3 on P3.

An Episode cannot be one proof: a stage awaiting evidence has no witness yet and ZKIR cannot pause. It should be one proof per stage linked by ledger-authenticated commitments — predecessor head, intent digest, episode id, cumulative budget, obligation roll-forward. That is ledger induction, which the architecture permits for a Midnight-resident lineage and which U2 may ship.

**But a join that verifies two parent proofs in-circuit is ZR09, owned by U4.** DESIGN.md §2 and §13 put joins in U3 without saying which join. The ledger-linked reading keeps U4 in U4; the in-circuit reading drags it into U3. Compensation must be a new stage checked on its own domain — folding inverse deltas into one cross-domain sum is exactly the global rollback the per-domain law exists to refuse.

### D10 — Smaller defects worth fixing

- **In-circuit signature verification is overclaimed.** Authoritative ZKIR v3 has `ec_mul` on Curve25519 and Poseidon/SHA-256/Keccak, but **no SHA-512 and no Ed25519 verifier**; `and`/`or`/`xor`/`sha512` are crate extensions outside the 34 instructions. The sound binding is a specialized stage with a Poseidon digest as a public input and the wallet signing that digest, or a ledger-checked signature over it. **This reaches back into the U0 proposal's T6 decision**, which makes in-circuit authentication primary.
- **`assert` is not a boolean constraint** — in-circuit it enforces non-zero, so every Φ bit needs `constrain_to_boolean` first or a witness of 2 satisfies a clause (ZR06).
- **`unbounded_within(Scope)`** contradicts both bounded episode budgets and the design's own "bounded everything" goal. Delete it.
- **Asset identity is insufficient.** Two deployed tokens can share domain, issuer, symbol, representation and decimals. Use an immutable ledger asset identifier with symbol and decimals as checked metadata. `Share<P>` needs a pool *instance and share class*, not a program; `Signed<A>` is keyed by asset when positions are per *instrument* — split `Delta<A>` from `Position<Instrument>`.
- **One encumbrance, one obligation.** `against: ObligationId` contradicts the map's claim that one encumbrance backing several obligations is expressible.
- **§3.5 drops U0 law L7's AccrualFirst component split**, a regression against the contract MIL/1 must land inside.
- **Two conservation laws want the same freeze.** MIL's general law and `UNIFIED-PROPOSAL.md:90`'s zero form cannot both be frozen; S0 must specialize the general law with its supply delta fixed at zero.
- **Residue can hide a clause.** A model can delete an expressible clause, mention it in residue and still be `satisfied`. Residue must be opaque, digested, unparsed, and must never cover an effect the relation could have carried — complete actual effects remain mandatory.

## 4. The five open items, corrected

| Declared open | Reviewers' verdict |
| --- | --- |
| n-party clearing | **Real, and the proposed carrier is refuted.** Every order writes the pool, so a join over *disjoint* footprints is the maximally wrong mechanism — intersecting writes are interference. Needs a multi-signer atomic stage with one shared write set; reserve the arity now. |
| Flash loans | **Not a consequence — a decision.** ZKIR can unroll a bounded internal trace, and a final repayment invariant is compatible with a finite atomic stage. Two reviewers call the exclusion a defensible product cut; all three say it must be restated as a choice, not an entailment. |
| Liquidation latency | **Not a language property at all.** It is liveness, and depends on chain inclusion and actor arrival. Specify when enforcement becomes *permitted* — safety — and drop the latency framing. |
| Competing-slash ordering | **Forced to be core, not open.** Two `enforce` effects write one cell, so footprint disjointness rejects them outright. A total order on seizes of one id is a core rule; only the policy is a library. |
| Policy state over reachability | **Belongs in the core now**, because amendment, revocation and in-flight protection already depend on it. Governance *process* can stay a library. |

**Missing from the list, found by the review:**

- **Concentrated liquidity.** Not expressible even after adding `pre`/`post`. A Q64.96 square-root price is not a `Price<_,_,s>` with `s ∈ 0..18`; tick boundaries are powers of 1.0001; crossing ticks is a bounded loop over `liquidityNet`, and Φ has no loop or array; and a position `(tickLower, tickUpper, liquidity)` is not `Share<P>`, not `Signed<A>`, and not a supply-∈{0,1} NFT. *[df]* lists it as its own element (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:186`), not a parameter of `x·y ≥ k`.
- **Evidence predicates** for signatures, recipient acceptance, document commitments, challenge records and unknown foreign outcomes — all required by the product's conditional-delivery contract, none in Φ.
- **Bounded observation collections**, without which TWAP and aggregation cannot be libraries.

## 5. Category-map corrections

Wrong "Expressible" verdicts: AMM `x·y ≥ k` (no pre/post, and no "pool's own program" is defined); vault deposit, redeem and empty-pool bootstrap (mulDiv types redemption only, no `totalAssets`); the LTV example (type-errors three ways and mixes dimensions); periodic funding and ADL (need computed and guarded effects the effect grammar lacks); NFT and position rows (`Asset` has no token id; `Signed<A>` is keyed by asset, not instrument); one-encumbrance-many-obligations (contradicts the type); governance policy commitment (one sentence of prose, not a clause or a stage field).

The twelve-asset table is **a useful inventory, not a three-rule architecture**. The reduction fails on rebasing tokens, fee-on-transfer, blacklists and pausable transfers, multi-collateral claims with lien priority, NFT fractionalisation, and off-chain obligors — where `issuer: Policy` identifies neither the obligor nor its consent.

## 6. Converged U0 minimum freezable subset

All nine agree U0 cannot take the whole design. The union of their minimum sets:

**Freeze now:** the versioned canonical encoding, its Poseidon domain tag, unknown-tag rejection and version-migration rule; the digest algorithm and its signature locus; numeric caps as actual numbers (Φ depth, total nodes, nonlinear nodes, `min`/`max` nodes, `k_of_n` width, effect lines, footprint cells, episode length, fan-in); `Qty` width and the limb rule for products that exceed the field; the stage public-input schema (predecessor commitment, budget counters, terminal tombstone, replay id, episode id); the authority right-kind enum including `issue` and `enforce`; the footprint sort with a complete cell vocabulary; the evidence class as a type index; domain-qualified `Instant` with an elaboration for `now`; residue as opaque digested bytes; the operator variance table; and conservation as `Σ deltas = supply delta` with S0 specializing it to zero.

**Defer:** the unsatisfiability checker, polarity-as-theorem, a universal in-circuit interpreter, Episode proof composition, `mulDiv` and general sets, and synthesis.

The rule that decides what must be frozen: **deferring the envelope or the condition commitment breaks signatures when escrow arrives; deferring advanced opcodes under a versioned envelope does not.**

## 7. Findings that reach back into the U0 proposal

Four of these are not MIL/1 defects at all — they are defects in the contract MIL/1 was written to land inside, and they need fixing there:

1. **E1** states conservation in the zero form; it must be the general law with S0 as a specialization (already recorded as CLM-0973).
2. **T6** makes in-circuit intent authentication primary; ZKIR v3's instruction set has no Ed25519 verifier and no SHA-512, so the claim overreaches the target.
3. **The U3 discriminator's cross-multiplied UInt256 inequalities** do not fit a 255-bit field and need the limb rule.
4. **The U0 plan keeps SMT discharge out of U0**, and a compile-time-only solver does not escape that cost unless the circuit re-establishes the property.

## 8. What I verified personally

The escrow rule rejecting both the showcase and the lending example; `totalAssets` absent and `mulDiv` mistyped; `not` used at `:185` with no negation production; the Cell vocabulary omissions; the field arithmetic (255-bit modulus against a 256-bit product); and the missing atoms across §10 and the category map. Everything else is reviewer-reported with the anchors they cite.

## 9. Recommendation

MIL/1 should not be frozen and should not be patched in place. The repairs are structural — three theorems replaced, Φ given formation and definedness judgments, the evidence type re-indexed, the cell vocabulary completed, the escrow rule replaced by a transition relation — and they interact. Cut **MIL/2** against this report.

Five choices in that revision are the owner's, not a reviewer's:

- **Φ₀ or Φ₁ at U0?** Dropping products makes the compile-time checks solver-free; keeping them buys ratio constraints and costs an SMT dependency the U0 plan excludes.
- **Flash loans:** restate the exclusion as a product cut, or admit a bounded intra-stage trace.
- **Joins:** ledger-linked only in U3, or in-circuit and accept U4's recursion dependency moving earlier.
- **n-party clearing:** reserve a multi-signer stage arity now, or defer and accept that batch clearing needs a later breaking change.
- **Concentrated liquidity:** in scope, or declared out — it is not reachable by any incremental patch to Φ.
