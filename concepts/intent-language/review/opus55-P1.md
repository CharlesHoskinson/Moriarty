# MIL/1 review — lens P1, type system and metatheory

**Reviewer:** Claude Opus 5.5. **Date:** 2026-09-28. **Mode:** read-only; this file is the only write. Nothing was executed except `moriarty_dev/cli.py status --json` (capability `SP01.6 loan-swap-subset`, five missing-evidence rows, seven pending Preview transactions — unrelated here). Every claim is derived by reading; none is a green check.

## 1. Verdict

The sorts are close to right and the shape is right: outcome-first intent, one predicate language, structured `Asset`, an evidence class on observations, footprints as a carrier, typed holes. Keep all of it. It is **not ready to freeze**: three load-bearing claims are false as written rather than incomplete — the anchoring rule is not a typing rule (§3.4:96), monotone completion is not enforced by the polarity check described (§7:201), and Φ is neither total nor usefully decidable for the obligations §6 puts on it (§4:123 vs §6:187). Two more are ordinary type errors: `mulDiv` carries two incompatible signatures in one document (:79 vs :130), and `Cell` (:114) does not cover `Effect` (:212) or Φ's own state-reading terms, which silently defeats linearity across an Episode. Before the freeze: make evidence a domain-indexed label with a lattice and a propagation rule; replace the single antitone rule with a beneficiary-indexed polarity over holes drawn from bounded lattices; make `Cell`, `Effect` and Φ mutually exhaustive. Freeze the sorts, the footprint notion and the evidence label at U0, and move §6's exhaustiveness check and §7's polarity check to U1 with a named decision procedure — both are compile-time obligations over nonlinear bit-vector arithmetic and neither is budgeted anywhere.

## 2. Findings

### 2.1 The anchoring rule is an information-flow discipline and is not written as one

§3.4:89-92 makes `Evidence` a **field of the `Obs<T>` record**, and §4:127 gives the term `obs.value : T`, so projection erases the label. §3.4:96 says a position requiring `anchored` rejects an `imported` term — but a position holding `min(p_anchor.value, p_import.value)` holds neither; it holds a compound term with no stated label. There is no propagation rule, no subsumption rule and no label on composite terms, so goal 4 (:19, "the evidence class is part of the type") is not realised. The rule needed, formally:

```
ℓ ::= anchored(Domain) | attested(Issuer,k,n) | imported(Policy)
types  T @ ℓ                     order  anchored(d) ⊑ attested ⊑ imported   (⊑ = weaker)
terms  Γ ⊢ tᵢ : Tᵢ @ ℓᵢ  ⟹  Γ ⊢ op(t₁..tₙ) : T @ (⊔ᵢ ℓᵢ)      -- weakest argument wins
check  a position demanding ℓ_req accepts t : T@ℓ iff ℓ ⊑ ℓ_req
stage  a predicate evaluated on domain d discharges anchored(d′) only if d′ = d
```

This is a graded/coeffect (taint) discipline, not subtyping and not an effect system — and it is the only one of the three that composes through `min`, `max`, `mulDiv`, `k_of_n` and cross-multiplied comparison with no extra rules.

### 2.2 `anchored` is not domain-indexed, so an Episode edge launders it

§3.4:89 defines `anchored` as "authenticated by *the executing domain*" — a relative property — while the syntax makes it a nullary constructor. Take an Episode (§2:35) with S₁ on `ethereum.mainnet` and S₂ on `midnight.preview`. In S₁, `obs_eth` is authenticated by Ethereum and labelled `anchored`, correctly. S₂ reads it across an Episode edge; nothing in §2, §3.4 or §7 re-labels it, because the label is a record field that travelled and `anchored` has no argument to change. S₂ then contains `require anchored(obs_eth)` and the check passes: a fact authenticated only by Ethereum has been presented to a Midnight stage as anchored, which is the exact sentence §3.4:96 exists to forbid. §6:189's "no predicate can assert an anchored fact about a foreign domain" and CATEGORY-MAP.md:190's "Refused, by construction" both fail.

The fix is one token — `anchored of Domain` — plus the stage rule above. The same gap exists statically: `Obs` has a `domain` field *and* an evidence class with no stated relation, so `Obs{domain: ethereum.mainnet, evidence: anchored}` is well-formed inside a Midnight stage.

### 2.3 Holes launder evidence, and Φ is not closed under hole substitution

§7:194-196 declares holes with a **type and a value refinement, no label**. If a solver fills `hole mark : Price<ETH,USD,6> where mark ≤ X` from an imported feed and `mark` occupies a position the intent required anchored, nothing rejects it: §7's only completion obligation is monotonicity, and `where` constrains magnitude, not provenance. The missing preservation lemma — `Γ,h:T@ℓ ⊢ Φ ok` and `Γ ⊢ σ(h) : T@ℓ′` with `ℓ′ ⊑ ℓ` imply `Γ ⊢ Φ[σ] ok` — cannot be stated until hole declarations carry `@ ℓ`.

Worse: Φ is **not closed under the substitution hole-filling performs at all**. §7's own examples declare holes of type `Route` and `Program`, and neither is a sort in §4's term grammar, so `failed(route)` (used at :256) is a proposition over a value Φ cannot denote. Either Φ gains a sort for solver artefacts — with its own evidence label, since "the route failed" is a claim someone must authenticate — or holes of non-Φ type are barred from Φ positions. The design does both and neither.

### 2.4 Counterexample to the monotone-completion polarity check

§7:201 states the rule as `Φ_guarantee[σ] ⟹ Φ_guarantee`. That is ill-formed: the right side is open. The three repairs fail differently. Under the ∀-reading (`⋂_σ ⟦Φ[σ]⟧`) no filling satisfies it unless the guarantee ignores the hole; under the ∃-reading (`⋃_σ`) every filling satisfies it and the check is vacuous; only the **extremal reading** — compare against the filling at the top of the refinement region — is non-degenerate, and it requires each hole's admissible set to be a bounded lattice with a top, which `venue ∈ approved_set` and `Route` do not have.

The counterexample, under the operational form as stated ("a hole may not appear in a position where increasing it widens the admissible set of a guarantee"):

```
hole  h : Qty<A>  where h <= 2_000_000 A
escrow E { fund 11 A from owner
           release  delivered(B, >= 20 B, to owner)
           refund   after(validity.end) or (realized_loss >= h)
           deadline validity.end + 1h }
```

`h` occurs **antitone** in `realized_loss >= h`: increasing `h` shrinks the states in which `refund` holds. The polarity check therefore *permits* the occurrence — it forbids only monotone ones. The solver fills `h := 2_000_000 A`. Every stated check still passes: the filling respects `where`; the occurrence is in a permitted position; §6:187's `release ∧ refund` is still unsatisfiable and `release ∨ refund ∨ after(deadline)` still valid, since the deadline disjunct is untouched. And the signer's early exit is gone: funds lock until `validity.end` in every scenario short of a two-million-unit loss. **A filling satisfied the syntactic check and strictly weakened a guarantee.**

The cause is a sign error. Antitonicity is measured on a clause's truth set, and shrinking it protects only clauses that are *obligations on the counterparty* (`release`, `net >=`, `delivered`). For the signer's *entitlements* — `refund`, `recover`, challenge and dispute windows, `seized` bounds — shrinking is the harm. The rule needs a beneficiary index: a hole may occur antitone in `obligation(signer)` clauses, monotone in `entitlement(signer)` clauses, and nowhere in a clause that is an obligation for one party and an entitlement for another unless both directions are pinned. That third case is not hypothetical — the liquidation policy at CATEGORY-MAP.md:57-59 (`trigger mulDiv(collateral_value, ltv_max, 1) < outstanding(loan)`) is the liquidator's entitlement and the borrower's obligation; a hole in `ltv_max` sits antitone in the trigger and either extreme harms one party. §5's `enforce` right makes two-party clauses first-class, so a single global direction cannot be right.

Two further gaps: §7 never delimits which clauses are "guarantee clauses" (`budget`, `release`, `refund`, `require`, `policy`, `deadline` are all candidates and do not share a direction); and it needs a per-constructor variance table — `k_of_n` is antitone in `k` although `k` sits where a naive sign analysis reads positive, `fresh` is monotone in its `Duration`, `within(W)` is monotone in width and antitone in start, `mulDiv` is antitone in its third argument and undefined at zero.

### 2.5 Φ is not total, and §6 needs a decision problem nobody budgeted

§4:123 asserts "Total, decidable, bounded". Three operators are partial. **`mulDiv` by zero**: the share form is `mulDiv(assets, shares, totalShares(P))` and CATEGORY-MAP.md:200 claims empty-pool bootstrap is expressible — that is `totalShares(P)=0`, the ERC-4626 first-depositor state, and Φ has no value for it. **Checked `−` on `Qty`**: if it rejects, Φ is three-valued at runtime while §6's checks are two-valued, so the exhaustiveness proof does not transfer to execution; if it saturates, it is not the frozen numeric profile's subtract and §4:147's "one evaluator, one correspondence argument" is false. **`fresh` has no lower bound** (:138): written naturally, a future-dated observation is permanently fresh, so `require fresh(price, 5m)` is satisfied by a feed timestamped a year ahead; written as unsigned subtraction it underflows. Require `0 ≤ t_stage − obs.observedAt ≤ d`.

The design also conflates two decision problems. §4:145's static costing concerns *evaluating* a clause and is fine. §6:187 and §7:201 are *compiler* obligations — unsatisfiability, validity, polarity — over a theory containing `mulDiv` and cross-multiplied comparison with all four factors variable: nonlinear integer arithmetic, decidable only because `u128` is finite, with bit-blasted 128-bit multiplication as the procedure. "Decidable" is true and operationally empty. Validity is also *modulo state invariants* (exits are exhaustive only given `funded`, roll-forward, conservation) and the design never says which the checker may assume. A compiler answering `unknown` is acceptable — but §11:284's outcome vocabulary must then cover the compiler's own `unknown`, and §6's "rejected at compile time" becomes "rejected, or admitted with a recorded proof obligation".

### 2.6 Concrete type errors

- **`mulDiv` has two signatures.** §3.3:79 gives `Qty<A> × Share<P> × Share<P> → Qty<A>`; §4:130 gives three arbitrary terms plus a `rounding` argument the first lacks; CATEGORY-MAP.md:57 and :83 use it as `Qty × ratio × 1`. The §3.3 form is also too narrow for the price-weighted case it was introduced to type.
- **No scalar/rate sort.** `Price<B,Q,s>` is asset-indexed, so interest rates, LTV, bonus and fee tiers are untypeable (CATEGORY-MAP.md:59, :70). *[df]* `/home/charl/projects/defiformal/lean/DefiKernel/Typed/Types.lean:41-52` carries `scalar` beside `amount` and `price` for exactly this. Nor does anything state that `a·d ≤ b·c` (:80) requires `dim(a)·dim(d) = dim(b)·dim(c)` or that the intermediate is `u256` — and that form is the *only* admitted way to write a limit.
- **`Signed<A>` has no instrument index.** CATEGORY-MAP.md:109 types a short call as `Signed<ETH> = −1 ETH` while :227's invariant is "Σ long = Σ short per instrument", so a short call nets against long spot. Positions need `Signed<I>` with a declared underlying.
- **Balance deltas have no type.** §9:221 sums "balance deltas" while balances are unsigned and only `supply.delta` is `Signed` (:214). Needs `Δ<A> = Signed<A>` with a checked coercion.
- **`Share<P>` and `Pool` are unlinked.** Nothing ties `P` to its assets, so `mulDiv(q : Qty<USDC>, s : Share<P_eth>, totalShares(P_eth))` typechecks; `Cell` (:114) has no `pool(Id)`, so two pools sharing an account share a cell and `totalShares(P)` binds to none.

### 2.7 Linearity is defeated by the only composition mechanism provided

§3.5:106 makes `Receipt` linear; §3.6:119 makes footprint disjointness the independence criterion. But a `Receipt` is not a `Cell`, and neither is an `Encumbrance`. So two Episode branches consuming the same receipt have **empty, hence disjoint, footprints**, are admitted as independent, and both accept — the receipt is spent twice with no rule broken. The same argument gives a double `seize`: §9:216's `encumber{lock|release|seize}` writes a cell `Cell` cannot name. Φ compounds it — `allowance(auth)`, `totalShares(p)` and `obs.value` (:127-129) read state `Cell` cannot name, so "every operation declares one" is false for three of Φ's term forms. A coherent substructural story is cheap: every linear resource carries a *consumer set* and is consumed exactly once by exactly one member — `Receipt` `{owner}`, permission affine `{holder}`, encumbrance linear `{owner-under-discharge, authority-under-enforce}`. Seizure becomes a disjunction of principals rather than a fourth discipline, and needs a linear-context split at forks with disjointness at joins, which §2 lacks.

### 2.8 Conservation and the liability/supply separation

The separation mostly holds — `Obligation` has no coercion into `Qty`, and `oblig{create}` moves no balance. Three leaks. (1) **The roll-forward lost its component law**: §3.5:109 states only the aggregate, while the U0 contract MIL/1 must land inside carries the split as L7 (`UNIFIED-PROPOSAL.md:89`), so `allocation AccrualFirst` (CATEGORY-MAP.md:53) has nothing to check against. (2) **Nothing couples `encumber{seize}` to `oblig{discharge}`**: with the bound at `outstanding·(1+bonus)`, a seizure that discharges nothing — or that zeroes an obligation with no discharge line — is well-typed, which is the "erased debt" hostile control at `ROADMAP.md:40`. (3) **Par redemption is typed as supply**: CATEGORY-MAP.md:93 realises it as escrow-plus-burn and :223 gives `Share<P>` the `issue` right, but a redeemable-at-par claim is a liability, and typing it as supply is the collapse `docs/MORIARTY-PRODUCT-CONTRACT.md:43` forbids. Pro-rata claim → `Share`; fixed-nominal claim → `Obligation`.

### 2.9 Smaller defects, stated once

§3.1:50 forbids a bare instant and gives no way to compare instants on two clocks, which every cross-domain escrow needs — and the bridge example writes `deadline now + 6h` (CATEGORY-MAP.md:177) against a release on another domain; cross-clock comparison needs a signed skew premise. `Window` (:52) has no `indefinite` form although :268 uses one. `complete by any solver budget affine 11 A` (:266) never says whether the budget is per-intent or per-holder — an authority double-spend when the holder is `any`. And §4's grammar does not admit the design's own examples: `failed` (:256), `never`/`none` (CATEGORY-MAP.md:49-50), `not` (:108), `exercised`, `equity` (:111), `collateral_ratio_ok` (:82) and `delivered(…, on domain) via imported(policy)` (:174) are absent from :126-143. The last is not cosmetic: without `on domain` and `via`, `delivered` is a bare anchored assertion and §6:189 fails on its own worked example.

## 3. Defects

Wrong, not merely incomplete. Each is argued above; D1 and D2 are the two I would most like a second reviewer to try to refute.

- **D1. The anchoring rule does not hold.** `anchored` is not domain-indexed, so an Episode edge carries an Ethereum-authenticated fact into a Midnight stage that demands `anchored` and the check passes (§2.2). §6:189 and CATEGORY-MAP.md:190 are therefore unearned.
- **D2. The polarity check admits a weakening filling.** A hole antitone in a `refund` clause passes §7:201 and deletes the signer's early exit (§2.4). Antitonicity is the wrong order for signer entitlements and for two-party clauses.
- **D3. Evidence is erased by projection, and holes carry no label** (§2.1, §2.3), so laundering needs no Episode at all.
- **D4. Φ is not total** — `mulDiv(_,_,0)` on an empty pool, unsigned `−`, and `fresh` with a future-dated observation (§2.5).
- **D5. Φ is not closed under hole substitution**: `Route` and `Program` are not Φ sorts (§2.3).
- **D6. Linearity is unenforced.** Receipts and encumbrances are not `Cell`s, so two Episode branches consuming one receipt are certified independent (§2.7).
- **D7. `mulDiv` has two incompatible signatures** (:79 vs :130 vs CATEGORY-MAP.md:57), and `Signed<A>` mistypes every derivative position (§2.6).
- **D8. Par redemption is typed as supply**, violating "debt is not token supply" at the one asset the rule was written for (§2.8).

## 4. Missing

**Must be in MIL/1:** the evidence label with lattice, domain index and propagation; labels on hole declarations; a scalar/rate sort with a product-dimension rule; `pool(Id)`, `encumbrance(Id)`, `allowance(AuthId)`, `receipt(Id)` and an observation cell in `Cell`; the linear-context discipline at Episode forks and joins; clause roles for §7; a definedness rule for `mulDiv`. **Library:** aggregation and TWAP, fee tiers, insurance funds, proposal lifecycles — correctly placed. **Later milestone:** the cross-clock skew premise (U4), the liquidation-latency bound (CATEGORY-MAP.md:124 — it needs the liveness notion the design rightly disclaims), n-party clearing.

## 5. Metatheory to demand

| Theorem | Status |
|---|---|
| Preservation/progress for Φ | Plausible in K *after* partial operators get preconditions. False today: `mulDiv(_,_,0)` is stuck. |
| Conservation as an invariant | Plausible in K; needs `Δ<A>` and Cell/Effect correspondence first. Finite, non-recursive — the right first mechanised result. |
| Roll-forward with components | Plausible; restore L7. |
| Monotone completion `A(I[σ]) ⊆ A(I)` | **False as stated** (§2.4). Provable only beneficiary-indexed, with holes on bounded lattices with a top. |
| Non-laundering of evidence | Most needed, least present. A noninterference property needing a *labelled* operational semantics and an erasure lemma across Episode edges. Not obtainable from a K rewriting reference alone, and not implied by any check in §3.4. |
| Linearity across an Episode | False today (§2.7). Provable with a linear context split at forks, disjointness at joins. |
| Escrow exhaustiveness/disjointness | Decidable only in the finite-width sense; needs a named procedure, an assumable invariant set, and an `unknown` outcome. |

## 6. Disagreements

**The five declared-open items are the wrong five.** CATEGORY-MAP.md:249 lists n-party clearing, flash loans, liquidation latency, competing-slash ordering and policy state; the genuinely open items are evidence propagation, hole labelling, the polarity sign, Φ's partiality and the Cell/Effect correspondence. Flash loans are correctly excluded. Competing-slash ordering is a *type* question answerable today — an encumbrance against several obligations is a shared linear resource with a consumer set — not an open one. **I disagree that the anchoring rule "implements the prose as a type error" (§3.4:96) and that global atomicity is "refused by construction" (CATEGORY-MAP.md:190)**: both are true of the intended design and false of the specified one. **I disagree with "one predicate language" as justified at §4:147** — I support the goal, but Φ is presented as closed while every worked example needs atoms it lacks. The honest form is a closed core plus a typed extension point: uninterpreted authenticated predicates carrying an evidence label, footprint and cost. Without it the first library family forces a grammar break inside a hash-bound freeze — the failure §12 argues against for the twelve-field intent. **I agree with the design against `UNIFIED-PROPOSAL.md:90`**: §9:221's general conservation law is right and E1 should be amended, not frozen. **Remove `unbounded_within`** (:157). Convergent outside evidence: *[df]* `lean/DefiKernel/Typed/Authority.lean:8-11` independently reaches `changeSupply(domain, asset)`, supporting §5's `issue`.

## 7. Top three changes

1. **Make evidence a domain-indexed label with a lattice and propagation.** Replace §3.4:89-92 with `ℓ ::= anchored(Domain) | attested(Issuer,k,n) | imported(Policy)`; give observed values `T @ ℓ`; make every Φ operator take the join of its arguments' labels; add `@ ℓ` to hole declarations (:194) and re-run the check after substitution; add the stage rule that a predicate on domain `d` discharges `anchored(d′)` only when `d′ = d`. This makes §3.4:96, §6:189 and CATEGORY-MAP.md:144/:185/:190 true rather than intended.
2. **Replace §7:201's antitone rule with a beneficiary-indexed polarity, and constrain hole domains.** Tag clauses `obligation(π)` / `entitlement(π)`; require antitone in signer-obligations, monotone in signer-entitlements, pinned in two-party clauses; restrict hole types to bounded lattices with a top; state the theorem as `A(I[σ]) ⊆ A(I)` against the extremal filling; publish the per-constructor variance table. Without this, §7 admits the §2.4 counterexample.
3. **Make `Cell`, `Effect` and Φ's terms mutually exhaustive, and put linear resources in `Cell`.** Add `pool(Id)`, `encumbrance(Id)`, `allowance(AuthId)`, `receipt(Id)` and an observation cell to :114; require every `Effect` constructor and every state-reading Φ term to name a cell; state the Episode fork/join context rule. This closes the double-spent receipt and the double seizure, and cannot be retrofitted after the freeze — for the reason §3.6 itself gives about footprints.
