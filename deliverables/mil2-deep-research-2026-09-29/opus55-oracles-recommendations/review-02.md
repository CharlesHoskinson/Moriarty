# Recommendation: MIL/2 Oracles and observations, focused on aggregation and TWAP

**Status:** specified-only design review. Everything below is a recommendation. I claim no implementation, proof, K result, native certificate or ledger acceptance. The startup status command ran and reported `SP01.6 loan-swap-subset` as blocked with no pending transactions. That dispatch block does not affect this read-only review.

## 1. Verdict on category fitness

The category is **fit to build on, but not yet able to express aggregation.** MIL/2 has the right basic parts:
- a source set that propagates through `min`/`max` (`DESIGN-MIL2.md:184`);
- a rule that aggregation cannot turn imported evidence into anchored evidence (`MIL2-PROPOSED-SEMANTICS.tex:83`);
- `fresh` with `observedAt ≤ stageTime` (`DESIGN-MIL2.md:144,187`);
- `k_of_n` (`:143`);
- a `Window(clk)` sort that is declared but never used (`:73`).

It has no bounded collection, no rule that sources must be distinct, no semantics for a missing sample, and no fallback construct (`DESIGN-MIL2.md:360`; `05-oracles.md:20-21`). The stage schema's observation item also has no value field or time field (R5 G1/G2, `R5-oracles.md:294-313`).

The most important point is this: **a median used only in a guard, and a TWAP computed from accumulators, can both be written in Φ₀ without division.** The earlier review placed all aggregation in later library work (`05-oracles.md:33`). That is too late for these two forms. Only **value-form** means and TWAPs need division and therefore Φ₁. Separately, U0 must reserve the collection arity in the schema that gets hash-bound.

## 2. Five ranked design edits

### E1. Bounded, distinct observation collection (core; U0 reservation)

**Rule sketch:**
```
ObsSet<N, T, ε, d, P>   N ≤ cap_obs (proposal: 8), P = signed member policy
members: [(feedId_i, issuerKey_i, seq_i, observedAt_i, value_i : T)] in canonical feedId order

Γ ⊢ S : ObsSet<N,T,ε,d,P>  ⇐  ∀i. feedId_i ∈ P.members
                              ∀i≠j. feedId_i ≠ feedId_j ∧ issuerKey_i ≠ issuerKey_j
                              ∀i. same clock c; observedAt_i ≤ stageTime
src(S) = ⋃ᵢ {id_i}
```
Each slot is either `present(v)` or `absent`. An absent slot is decided by the witness, so it can never make a predicate true. If a member fails freshness, it becomes `absent`; it does not cause a silent drop.

**Counterexample this blocks:** P = {F1, F2, F3}. The prover submits F1 twice (seq 7 at 85, seq 8 at 86) plus F3 at 150. Without the distinctness rule, a median guard counts two votes for 85, and one feed controls the result.

**Placement:** U0 adds the collection sort to the evidence type index, per-sample fields (`feedId`, `issuer`, `seq`, `observedAt`, `value`, `unit/scale`) to `observations[]`, the `cap_obs` header cap, and a reason code `DUP_SOURCE`. This does change a U0 boundary. The reason: `observations[]` is hash-bound at U0 (R5 G1), and adding arity later costs as much as the multi-signer case that decision 4 reserved early (`DESIGN-MIL2.md:336`). The Φ₀ semantics are then admitted at U1/U2.

### E2. Median guard as counted comparison, plus a small value-form `median_lo` (core, Φ₀)

**Rule sketch, guard form (a desugaring, so it adds no new decision procedure):**
```
median_lo(S) ≥ p   ≝  count{ i | present_i ∧ v_i ≥ p } ≥ N − ⌈N/2⌉ + 1
median_lo(S) ≤ p   ≝  count{ i | present_i ∧ v_i ≤ p } ≥ ⌈N/2⌉
```
Here N is the **declared** membership size, not the number of samples present. Counting an absent slot as failing is conservative in both directions. `count` is `k_of_n` over typed comparisons with the same bit-sum lowering (`DESIGN-MIL2.md:143`; lowering at `:265`).

**Value form:** `median_lo(S) : T ! src(S)` is admitted only for N ≤ 3, and only when all members are present:
```
median3(a, b, c) = max(min(a, b), min(max(a, b), c))
```
That uses 4 `min`/`max` nodes, which fits the cap of 8 at `DESIGN-MIL2.md:277`. Larger N uses a witness-checked form: m ∈ S, `count(v_i ≤ m) ≥ ⌈N/2⌉`, and `count(v_i < m) < ⌈N/2⌉`. That needs its own cap row. The lower median needs **no division and no rounding**. The midpoint median of an even N set needs division, so it is deferred to Φ₁.

**Counterexample this blocks:** a liquidation guard "price ≤ 90" over {100, 101, 10}, where 10 is a manipulated low outlier. If the guard only asked whether any one feed is ≤ 90, it would pass. With `median_lo`, only 1 member is ≤ 90 and 2 are needed, so it rejects.

**Placement:** U0 fixes the grammar and the variance table. For polarity, `count ≥ k` is monotone in each member bit. U1 certifies the counter gadget, and U2 can execute it.

### E3. Accumulator TWAP as an anchored cell with a comparison-only guard (core sort; TWAP library)

**External practice, not a Moriarty rule:** Uniswap v2 stores `priceCumulative` and computes TWAP = Δacc/Δt. Its whitepaper, §2.2 "Price oracle" (https://app.uniswap.org/whitepaper.pdf), says the accumulator overflow is intended and that differences are taken modulo 2^256. Uniswap v3 accumulates log-price ticks, which gives a geometric mean (https://app.uniswap.org/whitepaper-v3.pdf, §5). I did not capture either document in this session; both need intake before anyone relies on them.

**Moriarty rule sketch:**
```
Accum<B,Q,s,clk>  width declared (e.g. u192 via the §4.5 limb rule), checked, NON-wrapping;
                  an overflowing update REJECTs ARITH_RANGE.
cells: accumulator(feedId, slot) in a bounded on-ledger ring of checkpoints, read at the CURRENT authenticated head
twap_ge(F, W, p_lit):
    acc(t₂) − acc(t₁)  ≥  p_lit × (t₂ − t₁)
    where  t₂ ≤ stageTime,  stageTime − t₂ ≤ maxLag,  t₂ − t₁ ≥ W,  both checkpoints ∈ ring(F)
```
Both sides are checked subtraction multiplied by a signed literal. That is Φ₀ cross-multiplication (`DESIGN-MIL2.md:140,160`), so there is no solver and no division. The limb rule applies to the product.

**Counterexamples this blocks:**
1. **Wraparound.** An accumulator adopted as-is from v2 wraps. Once wrapped, `acc(t₂) − acc(t₁)` underflows under checked subtraction, so the guard rejects. Silently allowing modular wrap would violate the ban on modular coercion (`DESIGN-MIL2.md:169`).
2. **Short window.** The prover picks t₁ one block before t₂. The `t₂ − t₁ ≥ W` condition rejects this.

**Placement:** U0 adds the `Accum` sort and cell kind. U1/U2 execute it. A **value-form** TWAP (for example, to size a seizure) is Δacc ÷ Δt, which needs division with role-directed rounding, so it goes to Φ₁ (U4). I do **not** move the Φ₁ boundary (`DESIGN-MIL2.md:161`).

### E4. Sample-window TWAP semantics (library over E1 plus a small core window rule)

**Rule sketch (core membership rule):**
```
inWindow(o, W, t_end) ⇔ t_end − W < observedAt_o ≤ t_end,
                        t_end ≤ stageTime,  stageTime − t_end ≤ maxLag
Samples in the window: seq strictly increasing, observedAt strictly increasing,
                       consecutive gap ≤ heartbeat g, count ≥ minSamples.
Missing-sample policy is a signed enum: {reject}. Adding carry-forward is deferred.
```

**Library guard (fixed cadence):** with slots of equal length, `mean ≥ p` becomes `Σᵢ vᵢ ≥ N·p_lit`. Both sides are linear, so this is Φ₀. A time-weighted mean with variable Δtᵢ is `vᵢ × Δtᵢ`, a variable-by-variable product, so it is Φ₁.

**Counterexample this blocks:** the prover picks the 3 samples from inside a 30-minute window that happen to sit on a manipulated spike. The fixed cadence plus the `gap ≤ g` condition forces the samples to be spread across the whole window. Separately, a sample with `observedAt = t_end − W` exactly falls on the excluded endpoint, so it rejects.

**Placement:** U0 fixes the endpoint conventions and the policy enum. The library goes to U6 (`ROADMAP.md:27`).

### E5. Signed fallback may be gated only on an anchored fact (core; U0 tag, U3 execution)

**Rule sketch:**
```
select { primary: G₁ over S₁ ; fallback: G₂ over S₂ when gate }
gate must be a Φ₀ term with src(gate) ⊆ {anchored@d}
   e.g.  stageTime − feedCell(F₁).observedAt > H
If no anchored gate exists:
   Accepted = Accepted(G₁) ∪ Accepted(G₂), and the signing display MUST show the weaker branch.
src(result) = src(gate) ∪ src(S_selected), and the evidence floor is checked per branch.
```

**Counterexample this blocks:** without the gate, the prover claims "primary missing" (which it cannot be forced to disprove), selects an imported secondary feed and liquidates. That is exactly the "unsigned evidence downgrade" forbidden by `MORIARTY-CONSOLIDATED-DESIGN.md:97`. This rule differs from the deleted MIL/1 clause fallback (`DESIGN-MIL2.md:266`). Both branches here are complete relations; the only question is which one applies.

**Placement:** U0 reserves the tag and the gate side condition. U3 executes it, which matches the conditions scope at `ROADMAP.md:24`.

### Also recommended: deviation / circuit-breaker macro (library)

`within_bps(a, b, k)` ≝ `a×10⁴ ≤ b×(10⁴+k) ∧ b×(10⁴−k) ≤ a×10⁴`. It is exact, needs no rounding, is Φ₀, and falls under the limb rule. A breaker is simply a signed guard that rejects the stage. A pause authority is outside the language.

## 3. Core versus library boundary

| Core (semantics, schema, native binding) | Library (macros over core) |
| --- | --- |
| `ObsSet` sort, distinctness, P-membership, absent-slot semantics, `src` union (E1) | `median_ge`/`median_le` wrappers; N-of-M quorum presets |
| `count` over typed comparisons; `median3`/`median_lo` term with its own node cap (E2) | Outlier rejection via `within_bps`; circuit-breaker patterns |
| `Accum` sort, checked non-wrapping update, ring-checkpoint cell, `W`/`maxLag` side conditions (E3) | `twap_ge`/`twap_le`; Tp/Ex/At bond laws L1/L13/L27 (R5 K15-K17) |
| `inWindow` endpoints, seq/heartbeat rules, missing-policy enum (E4) | Fixed-cadence mean guards |
| `select` with anchored gate and display rule (E5) | Fallback templates |
| **Deferred to Φ₁ (U4):** value-form mean, TWAP value, midpoint median — each with role-directed rounding (`ROADMAP.md:21`) | |

The dividing line: anything that changes the acceptance relation, the source set or the public statement is core. Anything that desugars into already-admitted Φ₀ nodes is library.

## 4. Smallest implementable slice and evidence pair

**Slice:** one stage on one domain, anchored evidence only. That avoids in-circuit signatures, which ZKIR lacks for Ed25519 (`DESIGN-MIL2.md:264`). Three on-ledger feed cells F1, F2, F3 on the same clock. The guard is:

```
∀i fresh(Fᵢ, 5m)-or-absent  ∧  median_lo({F1,F2,F3}) ≤ 90   -- i.e. count(vᵢ ≤ 90 ∧ fresh) ≥ 2
```
followed by a fixed liquidation transfer.

- **Positive:** F1 = 89, F2 = 88, F3 = 150 (an outlier, tolerated), all fresh, distinct feedIds. Count = 2, so it accepts, and the public statement binds all three identities and values (`MIL2-PROPOSED-SEMANTICS.tex:240`).
- **Hostile:** identical except the F2 slot carries a second, fresh, correctly authenticated F1 reading (feedId F1, seq + 1, value 85). The envelope is well-formed and every field is valid, so a rejection cannot come from malformation (`ROADMAP.md:40`). Only the distinctness rule rejects it, with `DUP_SOURCE`. Without that rule, the count is 2 and it would accept.

**Secondary hostile cases:** F2 has `observedAt = stageTime − 5m − 1`, so the slot is absent, the count is 1, and it rejects. A member is absent entirely, so the count is 1 and it rejects.

**Placement:** executed first in TypeScript and K. At U2 it is a candidate for the "structurally contrasting program". Otherwise it goes to U3, given that TP03 blocks U2 only for programs that claim an observation (`R5-oracles.md` S0 row).

## 5. Explicit disagreements

1. With `05-oracles.md:21,33`: "`k_of_n` does not compute a median" is true only for the value form. The guard-form median **is** a `k_of_n` once E1 exists, so it does not have to wait for U6.
2. With `DESIGN-MIL2.md:360`, which treats collections as a library prerequisite only: collection arity must be reserved in the **U0 schema**, or the hash-bound `observations[]` shape will have to be retrofitted.
3. With importing the Uniswap v2 accumulator unchanged: its intended modular overflow conflicts with Moriarty's checked-width rule. Moriarty must declare the width and reject on overflow.
4. On the rationale for the `k_of_n` cap ("expands binomially", `DESIGN-MIL2.md:278`): in-circuit it is a linear bit-sum. The binomial blow-up happens only if the authoring checker case-splits. It should use a cardinality encoding in QF-LIA. The cap value can stay; the note should say it concerns the checker.
5. With any reading of `attested(obs, k, n)` (`DESIGN-MIL2.md:145`) as covering aggregation: it lacks distinct issuers and same-value semantics. E1's distinctness rule has to feed it.

## 6. Residual assumptions (named, not proved)

- Feed honesty and market truth stay out of scope (`MORIARTY-PRODUCT-CONTRACT.md:47`). Distinct issuers are not the same as independent issuers, since they may share an upstream source.
- Multi-block manipulation of an anchored on-ledger TWAP is an economic cost assumption. The language does not remove it (R5 K20).
- Prover censorship: the prover can always omit an honest sample. Treating absent as failing bounds the safety damage but not liveness. Manipulation resistance requires f + m < ⌈N/2⌉ (f Byzantine plus m censored members). This is a policy premise.
- The authenticity of the stage clock and of anchored feed cells depends on O2 (`MIL2-PROPOSED-SEMANTICS.tex:248`). Attested (off-ledger) samples wait on U4 imported-evidence verification and on a signature scheme that ZKIR can verify.
- Every cap value (`cap_obs` = 8, `median` N ≤ 3 in value form, ring size) is a proposal to be measured, like the existing caps (`DESIGN-MIL2.md:270`).
- An on-ledger ring-buffer accumulator contract on Midnight is assumed to be feasible. It is not demonstrated.