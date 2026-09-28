I'll review MIL/1 as a type-system and semantics design, starting from the design, category map, and the short constraint documents.The design and the three constraint documents are in. Next I’ll check the U0 contract, the coverage reports, and the DeFi Kernel types the design claims to sit beside.The type layer is not sound enough to freeze. The S0 fragment is buildable; the rules that would make MIL/1 honest are not the rules written in §3.4, §6 and §7.

## Verdict

MIL/1 names the right sorts and the right honesty goals: unsigned balances against signed positions, debt apart from supply, evidence as something a position can reject, escrow as a signed condition, holes that a solver may only narrow. Those goals match the product contract and the U0 laws. The metatheory that would make them true is not the metatheory in the text. As specified, Φ is an untyped grammar closed under neither definedness nor evidence; the anchoring rule is a tag check erased by every term former; the escrow "validity" rule rejects the canonical example and still does not force a closure; the polarity check accepts fillings that falsify the implication written next to it. Shares, positions and encumbrances have sorts and no effects, so the conservation laws in the category map are not invariants of any transition. I would not hash-bind this document at U0. I would freeze a sorted S0 fragment (exact `Qty`, obligation roll-forward, cap inequalities, affine authority, derived footprints) and replace §3.4, §6 and §7 with the rules below before any larger clause is part of the digest. Nothing here was executed. This is a reading of the specification.

## Findings

### Formation is missing, so the grammar is not a type system

§4 gives productions, not judgments. `term ≤ term` (DESIGN.md:133) does not require both sides to be `Qty<A>` for the same `A`, the same clock, or the same evidence. `balance` of NIGHT against `balance` of DUST is a proposition. So is `obs.observedAt ≤ balance(...)`. The sample's budget block (`net >= 20 B to owner`, DESIGN.md:242) is not in that grammar either: `to party` occurs only on `delivered`. One predicate language is a goal (DESIGN.md:17). The sample already has a second surface, and the category map adds a third (`failed`, `never`, `not exercised`, `equity`, `default`), none of which is a production.

The comparison kernel does not have this hole in the same way. *[df]* `lean/DefiKernel/Typed/Types.lean:40-52` indexes a value by `Unit`, and a failed unit match is `argumentUnit` (`:86`). *[df]* `lean/DefiKernel/ConcentratedLiquidity/FullMath.lean:24-28` types `mulDiv` as `Except Failure`, with `divisionByZero` a result, not a Boolean. MIL/1 puts `mulDiv` in the term grammar (DESIGN.md:130) and then says division is absent (DESIGN.md:144). Both sentences cannot hold. Binary `min`/`max` do not range over a set, so "empty min" is not a totality bug. These are:

- `mulDiv(x, y, 0, _)` has no value. Empty-pool bootstrap (CATEGORY-MAP.md:201) is a precondition comment, not a definedness rule.
- `term − term` on `Qty` underflows. Checked subtraction is a failure, and a failure is not a truth value. §11's outcome vocabulary is not the evaluation relation of Φ.
- `term · term ≤ term · term` (DESIGN.md:134) says "a wider intermediate" (DESIGN.md:81) and never names the width. UInt128×UInt128 needs 256 bits; a price mantissa already at that width needs another. Without a width, this is not checked finite-width arithmetic.
- `fresh` (DESIGN.md:138) is "within d of stage time" (DESIGN.md:94). That reading is symmetric: `observedAt` in the future by less than `d` is fresh. A feed can pre-timestamp.
- `k_of_n(0, _)` is unspecified and, on the ordinary reading, true.
- `Duration` is not clock-indexed (DESIGN.md:51) while `Instant` is (DESIGN.md:50). `fresh` subtracts a foreign `observedAt` from the stage clock. Either that subtraction is ill-typed, and imported observations cannot be fresh, or the clock index is erased, and §3.1's ban on bare instants stops at literals. The sample uses `now` (DESIGN.md:232) after declaring a bare instant a type error (DESIGN.md:55). An elaboration `now : Instant(intent.clock)` is the obvious repair. It is not stated.
- Asset `decimals : u8` (DESIGN.md:61) is not related to price scale `0..18` (DESIGN.md:73). The surface literal `11 A` has no elaboration into smallest units. Off-by-`10^decimals` is a well-typed intent.

Ground evaluation of a closed, defined formula is structurally recursive, so it terminates if every partial operator returns a failure. Validity of an open formula is a different problem. U0's judgment language is linear integer arithmetic (UNIFIED-PROPOSAL.md:119). Φ adds `mulDiv`. Over unbounded integers that theory is not the linear fragment, and satisfiability is not decidable by the argument the design gives (ban unbounded quantifiers and negation, DESIGN.md:144). Over a fixed bit width it is QF_BV validity: decidable, and not costed by walking the clause. §6's exhaustiveness check is the validity problem. "Every clause is statically costed" (DESIGN.md:123) covers evaluation only if a versioned bounds record exists (nodes, set cardinality, `k_of_n` arity, widths, repr depth). No such record is in the language. The canonical-form sentence (DESIGN.md:205) defers the numbers to "the language version" and does not say they are part of the typed syntax.

Negation is the wrong thing to spend that budget on. On a finite width, atomic negation does not change decidability. It does change polarity, which is tractable. The derivatives example needs `not exercised` (CATEGORY-MAP.md:108). The workflow gloss needs `not terminal` (DESIGN.md:185). The positive fragment cannot say that an event did not happen without a closed-world atom the grammar also does not have. The ban costs the category and does not buy the decision procedure.

Φ is syntactically closed under substituting a term for `hole`. It is not closed under the substitution solvers perform. A filler can change definedness (`d ↦ 0`), clock, asset, or evidence, and a where-clause can mention another hole (`a ≤ b`, `b ≤ a`). No simultaneous-substitution or definedness-preservation rule is stated.

### The anchoring rule does not track evidence

Write the rule the prose (DESIGN.md:96) actually is:

```
Γ ⊢ o : Obs<T>     o.evidence = anchored
────────────────────────────────────────
Γ ⊢ anchored(o) : Prop
```

That is a tag test on a field (DESIGN.md:89-91), not a subtyping relation and not an effect. Subtyping would be worse: `anchored <: ⊤` is a forgetful coercion, which is laundering. An index `Obs<T, ε>` is the rule that matches the claim. It is not what is written, because `ε` is not a parameter of `Obs` (DESIGN.md:84) and every eliminator drops it:

```
Γ ⊢ o : Obs<T, ε>
─────────────────
Γ ⊢ o.value : T          -- ε is gone
```

`min`, `max`, `+`, `−` and `mulDiv` then combine values that no longer have an evidence. Concrete laundering, well-formed in the grammar as written:

```
observe a : Price<B,Q,6>  evidence anchored
observe b : Price<B,Q,6>  evidence imported(p)
guarantee min(a.value, b.value) ≤ limit  and  anchored(a)
```

`anchored(a)` is satisfied. The quantity inside the inequality is `b` whenever `b` is the smaller price. The compile-time rejection never fires.

A hole of result type is the same erasure:

```
hole v : Qty<B>  where v ≤ cap
guarantee delivered(B, ≥ v, to owner)  and  anchored(mark)
```

The filler copies an imported print into `v`. `delivered` carries no evidence (DESIGN.md:141). The anchored observation is a different variable. The lending trigger is this shape already: `fresh(price) and anchored(price)` is a side condition on `price`, and the inequality is over `collateral_value`, which does not occur in the observation (CATEGORY-MAP.md:57-58). Nothing forces `collateral_value = balance · mark`.

Filling a hole with `imported` where the proposition says `anchored(o)` makes the proposition false, so that plan fails. That direction is closed. The open direction is the tag itself:

```
observe foreign : Qty<BTC>
  domain ethereum.mainnet
  evidence anchored
guarantee anchored(foreign)
```

executed on `midnight.preview`. DESIGN.md:189 says no predicate can assert an anchored fact about a foreign domain. The formation rule never mentions `o.domain = executing domain`. The solver does not need to smuggle `imported`. It selects the `anchored` constructor. `attested` is not a proposition at all: Φ can require `anchored` and `final` (DESIGN.md:139) and cannot require `attested(k, n)` or `imported(policy)`. The bridge line `via imported(policy = light_client_v2)` (CATEGORY-MAP.md:174) is not a Φ formula, and writing `anchored` there would reject the import the bridge needs.

An episode edge does not by itself satisfy `anchored(foreignObs)` with a later `balance` read: `balance` is not an `Obs`. The edge does launder numbers, because `policy(Id)` is an untyped cell (DESIGN.md:115). Stage 1 may store an imported price there; stage 2 reads a `Qty`. There is no sort to forbid it.

Residual is a party (DESIGN.md:180), not a term, so it is not an evidence channel. The `on_release` effects are. Nothing requires an effect amount to be built from observations the release condition actually depends on.

The rule that would match §6 is:

```
δ = executing domain
Γ ⊢ o : Obs<T, anchored>
Γ ⊢ o.domain = δ
────────────────────────────────
Γ ⊢ use(o) : T ⟨anchored⟩

ε₁ ⊓ ε₂ = ⊥   if either is not anchored-on-δ
────────────────────────────────────────────
min/max/mulDiv/cross-multiply are ill-formed
```

Arithmetic is admitted only under that meet. `imported` and `attested` are requirement forms with the same index, so a bridge can demand them. A host Boolean `anchored = true` is still not the authenticated read the product contract requires. The index is the static half of that obligation.

### Polarity, as stated, is not the implication written beside it

§7 (DESIGN.md:201) states: for any filling `σ`, `Φ_guarantee[σ] ⟹ Φ_guarantee`, and the compiler rejects a hole in a position where increasing it widens the admissible set. The implication is not a formula. `Φ_guarantee` still contains the hole. The check and the implication also disagree. This program passes the check and falsifies the implication.

```
hole q : Qty<A>  where q ≤ 11 A
guarantee (q · quote) ≤ (11 A · base)
```

Increasing `q` grows the left product and shrinks the set of states satisfying `≤`. The check allows the hole. The filling `q = 0` is inside the where-clause, and the guarantee becomes `0 ≤ 11 A · base`, true for every defined non-negative product. Relative to `q = 11` the ratio constraint disappears. `Φ[0]` does not imply `Φ`.

The same check allows a divisor hole if every term former is treated as covariant, which is what a polarity walk has to assume when the grammar gives it no variance:

```
hole d : Qty  where d ≤ 10000
guarantee mulDiv(amount, 10000, d, floor) ≤ cap
```

`mulDiv` is antitone in its third argument. Increasing `d` shrinks the quotient and widens `≤ cap`. A covariant walk classifies `d` as narrowing and accepts it. The filling `d = 10000` weakens the cap. A walk that knows the variance rejects it. §7 does not state the variance. The two walks decide different languages.

`k_of_n` is the evidence variant of the same hole:

```
hole k : u64  where k ≤ 3
guarantee k_of_n(k, [anchored(o1), anchored(o2), anchored(o3)])
```

Increasing `k` strengthens a threshold, so the check allows it. The filling `k = 0` is vacuously true and drops every `anchored` atom.

Refund, which §7 never classifies as a guarantee or as anything else:

```
hole t : Instant  where t ≤ deadline
refund after(t)
```

Increasing `t` shrinks the refund set. The check allows it. The filling `t = clock@0` makes refund immediate. The signer authorized a deadline and got a race they lose at once.

Equality is non-monotone in both sides. A hole in `net = h` with `h ≤ 20` is not covered by an "increasing widens" test at all. The filler chooses `h = 0`.

I may be misreading `Φ[σ] ⟹ Φ` as a predicate implication when a refinement reading was intended (the where-clause is the whole authorization, and any point inside it is fair game). Under that reading the check is too strong: it forbids `fees ≤ h` even when `h ≤ 11 A` is the signed cap, so a solver cannot narrow a cap either. Under the reading where the signed guarantee is the strengthening end of the interval, the check is too weak, and the four fillings above are weakenings. The two sentences in DESIGN.md:201 cannot be true together for this grammar.

A sound restriction, and the one U0 can actually freeze, is that `release`, `refund` and every guarantee clause contain no holes. Holes are `route`, `venue` and amounts checked by `≤` or `⊆` against a signer literal. A polarity calculus is a later language, and only with an explicit variance: `+` covariant, `−` contravariant on the right, `mulDiv` contravariant in the divisor, `k_of_n` antitone in `k` and covariant in the list, `fresh` covariant in the duration, `=` and `after` rejected unless the where-clause is a singleton, negation flipping variance.

### Escrow exhaustiveness does not mean a closure path

DESIGN.md:187 requires `release ∧ refund` unsatisfiable and `release ∨ refund ∨ after(deadline)` valid. "Valid" means tautologous. Before the deadline, `after(deadline)` is false, so every pre-deadline state would already satisfy `release` or `refund`. No waiting escrow can meet that. Read as "the disjunction is satisfiable", `after(deadline)` alone supplies the witness while both branches stay false, and the funds sit. Read as "the clause typechecks", `refund never` typechecks and never fires.

The sample fails the semantic reading. `delivered(B, ≥ 20, to owner)` and `after(validity.end) ∨ failed(route)` (DESIGN.md:254-255) are jointly satisfiable: delivery can happen after the end. `failed` is not in Φ. So the central example is either ill-formed or the check is not semantic. Collateral is in the same corner: `refund never`, `deadline none` (CATEGORY-MAP.md:48-49) is not in the syntax, and a loan that waits for discharge has no pre-deadline tautology. The construct the lending section calls expressible is rejected by §6.

The rule that matches the product contract (a timeout authorizes a transition and does not prove the other branch) is:

```
⊨  ¬ (release ∧ refund)
⊨  after(deadline) ⇒ refund
refund's deadline disjunct is anchored on the executing clock,
contains no hole, and does not depend on a foreign observation
```

`after(deadline) ⇒ refund` is what the sample almost does, with the deadline shifted an hour past `validity.end`, and what §6 does not check. Perpetual collateral is then an explicit refusal or a separately signed renewal episode, not `deadline none`.

### Linearity is three sentences and a fourth budget that contradicts both

Receipts are linear, spending permission affine, encumbrances "linear-and-seizable" (DESIGN.md:106-109, CATEGORY-MAP.md:227-228). `Budget` adds `unbounded_within(Scope)` (DESIGN.md:157). The architecture's episode bound is a finite cumulative authority limit (MORIARTY-CONSOLIDATED-DESIGN.md:83). An `issue` right that does not decrease, inside an indefinite window (the sample's `indefinite` recovery, DESIGN.md:267, copied onto a budget form that is not restricted to recovery), is a perpetual spending right. The consolidated design refuses to imply one (MORIARTY-CONSOLIDATED-DESIGN.md:68).

The join bug is independent of that. Independence is footprint intersection (DESIGN.md:117). Cells are `balance`, `supply`, `obligation`, `escrow`, `replay`, `policy` (DESIGN.md:114-115). Receipts, encumbrances, allowances, positions and shares are not cells. Footprints are declared by the operation, not computed from the effect. Two branches declare `{balance(x)}` and `{balance(y)}`, both consume receipt `r` or both `seize` and `release` the same encumbrance, and the intersection test admits the join.

One judgment covers the three regimes:

```
Δ affine budgets, split additively, consumptions summed ≤ budget
Γ linear names (Receipt⟨ι⟩, Encumbrance⟨ι⟩), split multiplicatively, disjoint
Λ persistent (Obligation, balances, supply); no weakening that drops debt

seize  : Γ, enc ⟨ι⟩ ; enforce(π) ⊢ ·
release: Γ, enc ⟨ι⟩ ; owner-path ⊢ ·
```

A join whose branches both mention `ι` fails the multiplicative split. `unbounded_within` is not a form of `Δ`. Declared footprints are checked equal to a map `ℱ⟦effect⟧`, not chosen. Until receipts and encumbrances are in `ℱ`, linearity across an episode DAG is not a property of the program.

### Conservation separates debt from supply only on the effect forms that exist

Per `(domain, asset)`, `Σ balance deltas = supply delta` (DESIGN.md:221) is the right law, and it is the law *[df]* states as `Accounted` (`lean/DefiKernel/Core.lean:69-70`) and proves for `applyEffect` (`:129`). *[df]* also puts debt in the asset index (`Core.lean:20`, `debt` is an `Asset`; the comment at `:38` says debt is not negative cash, then the sort still treats it as a balance cell). MIL/1's disagreement is right: `Obligation` is not an `Asset`, and `supply` cannot take an `Obligation`. That separation lives in the effect grammar (DESIGN.md:211-216) and nowhere else. `outstanding` and `balance` are both `Qty<A>`, so a user clause can write `supply(d, a) = outstanding(loan)`. Harmless if conservation is a judgment derived from effects. Fatal if it is a Φ clause the program is trusted to include.

What the grammar does not contain:

- An effect that updates `Share<P>` or `totalShares`. `supply` takes an `Asset` (DESIGN.md:214). Share conservation `Σ shares = totalShares(P)` (CATEGORY-MAP.md:225) has no deltas to sum. Nested vaults fail in a second way: `mulDiv : Qty<A> × Share<P> × Share<P> → Qty<A>` (DESIGN.md:78) cannot return a `Share<Q>`. Two pools that both hold USDC have distinct `Share` types, which is the right identity, and identical result types, with the reserve account not in `P`. `Pool` itself is not defined. A redeem of pool 1 can be written against pool 2's balance and still match the signature. A rounding role does not fix that. `retained-in-pool` (DESIGN.md:79) is not a conservation class. For positive integers the remainder of `x*y / z` has dimension of the product, not of `A`. Flooring the outflow leaves conservation holding with no unit to post. The U0 classes already have `sub-unit-residual` (UNIFIED-PROPOSAL.md:141). A sixth beneficiary class invites a double posting. *[df]* keeps the failure in `mulDiv` and the share accounting in the contract. That split is the one to copy.
- An effect that updates `Signed<A>`. The category binding `position p : Signed<ETH> = −1` (CATEGORY-MAP.md:109) is not a transition. `Σ long = Σ short` (CATEGORY-MAP.md:229) is not the balance law. `Signed<A>` indexes the wrong object: two ETH instruments are the same type, and a short can meet a spot balance only through an `abs` the grammar does not have. Effect amounts in §9 are untyped, so the "positions only, never balances" comment (DESIGN.md:72) is not a check. `supply.delta : Signed` (DESIGN.md:214) uses the bare word for a different sort.
- A consent witness on `oblig create`. `supply` carries `authority: issue`. `oblig` carries an op (DESIGN.md:215). Creating a liability without a consent sort is well-formed. `transfer` does not say whether debtor or creditor moves, so "consent from the party made liable" cannot be checked.
- A distinct accumulator for `forgive`. The product contract says forgiveness is not repayment (MORIARTY-PRODUCT-CONTRACT.md:43, MORIARTY-CONSOLIDATED-DESIGN.md:54). One roll-forward, `opening + accrual − discharge = closing` (DESIGN.md:109), makes `discharged(loan)` true after forgiveness if forgive is stored as a discharge. Collateral `release discharged(loan)` (CATEGORY-MAP.md:47) then returns the lien without payment.
- A free-balance premise. An encumbrance that does not debit `balance` leaves `Accounted` true while the owner spends the locked units. "The owner may not spend" (DESIGN.md:109) is not a premise of `transfer`. `locked ⊆ balance` (CATEGORY-MAP.md:227) is the missing premise, and `Encumbrance.against` is one `ObligationId` (DESIGN.md:104), so "one encumbrance against several obligations" (CATEGORY-MAP.md:207) is not the type that was specified.
- A link from `seize` to a creditor credit. `enforce` can move collateral to `enforceable_by` while `outstanding` is unchanged, which at least does not erase debt. It also does not pay the creditor. Liquidation is multi-stage by design (DESIGN.md:295). Nothing in the episode requires the discharge stage to exist.

`outstanding = principal + accrued` is not representable in the same width as its components. Checked add is partial there. The invariant needs a width rule or the constructor rejects.

Shielded assets are a separate `(domain, asset)` economy under DESIGN.md:221, because `repr` is part of identity (DESIGN.md:63). CATEGORY-MAP.md:223 says privacy is a disclosure policy and not a different asset economy. Under the conservation rule those are different supplies, and nothing burns `A` when `shielded(A)` is issued. Wrapped assets, correctly, use an obligation rather than one equation (CATEGORY-MAP.md:221). Shielding on one domain can be an atomic pair. It is not given. NFT `supply ∈ {0,1}` (CATEGORY-MAP.md:230) is not a property of `Asset`.

### What I would demand be proved

| Theorem | Status as specified | Where it can live |
| --- | --- | --- |
| Formation: every clause has a unique sort, homogeneous at comparisons, clock-indexed at times | False. No formation rules | K, once written. Structural |
| Substitution preserves sort and evidence | False. `.value` erases `ε`; filler can change definedness | K |
| Ground evaluation is deterministic and ends in a value or a named failure, inside the version bound | Plausible for the recursion-free fragment after partial ops return failures. False while `mulDiv` by zero and underflow are "total" | K, by induction on the formula |
| Validity of escrow side conditions is decided inside that same bound | False as a cost claim. QF_BV validity is a different theorem and needs a fixed width | Not a K execution claim. False over unbounded integers |
| Conservation: `Accounted` per `(domain, asset)`; obligation roll-forward; `outstanding = principal + accrued ≥ 0`; free balance | `Accounted` is plausible for `transfer`/`fee`/`supply` and is the theorem *[df]* already has. The share, position, encumbrance and forgive clauses are false: no effects or the wrong accumulator | K for the balance fragment |
| Monotone completion, `Φ[σ] ⟹ Φ` | False. Counterexamples above | A syntactic induction in K only after a variance table, or after holes are banned from guarantees |
| Non-laundering: a value used at `anchored` was introduced by a local authenticated read on the executing domain | False. Tag, projection, untyped `policy` cell, foreign `anchored` | K, as a subject-reduction invariant on an index. Not an effect system |
| Episode linearity: a join cannot consume `r` twice or both seize and release `ι` | False until `Γ` is multiplicative and footprints are derived | K |
| Escrow: after deadline, refund holds and release is impossible, from the executing clock alone | False for all three readings of "valid" | K only for the repaired implication |
| No coercion `Signed<I> → Qty<A>`, `Obligation → supply` | Not false so much as unstated. §9's amounts are unsorted, so it is not a theorem | K, one clause in formation |

K is the right host for all of the executable ones. The consolidated design already refuses a second mandatory semantics. Decidability of validity is the one claim that is not an executable K theorem.

## Defects

1. DESIGN.md:189 is false. `anchored` does not consult the executing domain, and `.value` drops evidence before `min` and `mulDiv`.
2. DESIGN.md:201 is false. The `q · quote` filling passes the stated check and breaks the stated implication. So do a covariant reading of `mulDiv`'s divisor, `k_of_n(0, _)`, and `refund after(t)`.
3. DESIGN.md:187 does not establish a closure path under any reading of "valid", and DESIGN.md:254-255 fails the only reading with teeth.
4. DESIGN.md:144 contradicts DESIGN.md:130. `mulDiv` is division, and it is partial.
5. CATEGORY-MAP.md:225, :229 and :207 state laws the effect grammar cannot witness. Share, position and multi-obligation encumbrance have no transitions.
6. `discharged` after `forgive` collapses a distinction the product contract requires (MORIARTY-PRODUCT-CONTRACT.md:43), because there is one roll-forward term (DESIGN.md:109).
7. Declared footprints plus a cell set that omits linear resources make the join rule admit double consumption (DESIGN.md:114-117).
8. `unbounded_within` (DESIGN.md:157) is a perpetual spending budget. It contradicts the finite episode limit the design says it preserves.
9. The lending, derivatives and bridge "expressible" rows depend on phrases that are not in Φ: `deadline none`, `not exercised`, `via imported`, `equity`, `default`, `failed`, and `mulDiv` applied to an LTV that is not a `Share`.

## Missing

**In MIL/1, or the sorts do not mean what they say.** Formation and definedness. Evidence as an index, with `imported` and `attested` requirement forms. Executing-domain side condition on `anchored`. Derived footprints, including receipt, encumbrance, allowance, share and position cells. Consent on `oblig create`. A forgive accumulator distinct from discharge. `Signed<Instrument>` and `Share<P>` with `P` naming its reserve accounts and reserve assets. `mulDiv` at the numeric profile as a partial word operation; `redeem` as a derived form that reads those cells. Free-balance. A versioned bounds record. Atomic negation, or an explicit closed-world `absent` atom, so options and "not yet delivered" are formulas. The budget block defined as sugar for Φ.

**Library, after those exist.** AMM curve, fee tiers, TWAP, virtual offset, ACTUS schedules, governance process, insurance-fund accounting, option payoff shapes, simple-versus-compound accrual.

**Later milestone.** n-party clearing. Flash-loan scratch nets, if ever. Circuit cost of predicate authentication (U1 already owns the measurement). Cross-domain episode runtime (U4). Competing-slash execution, once `ClaimClass` is a total order with a deterministic tie-break. Full policy-reachability logic.

## Disagreements

The five open items are not all open in the way §3 of the category map says.

n-party clearing will not fall out of a join of disjoint footprints (CATEGORY-MAP.md:39). A clear writes one pot. The participants' footprints meet. A join that is well-typed under DESIGN.md:117 cannot be a batch. The missing construct is a multi-signer stage or one clearing program that reads several signed intents. It can wait. The suggested mechanism cannot.

Flash loans are not an open design question. They are excluded by atomic stages (DESIGN.md:295). I agree with the exclusion.

A liquidation latency bound is liveness. The non-goals already refuse liveness (DESIGN.md:25). Listing it as open invites a construct the language should not grow.

Competing slashes are decidable if `ClaimClass` is a total order and ties break on obligation id. They are undecidable only if each slash policy may mention the others with no stratification. The open item is that missing stratification, plus the one-obligor encumbrance type. It is not an inherent gap.

Policy state over reachability is genuinely thin: `policy(Id)` has no value sort. Governance process can stay a library. The policy digest a continuation must repeat is a core field, because in-flight amendment is a core honesty property (CATEGORY-MAP.md:158). It should be typed now and interpreted later.

Further disagreements. One evaluator for Φ and for U0 judgments is a good end state and a bad freeze. The judgment language stays the linear subset until `mulDiv` has a width and a failure. E1 in the U0 proposal is the zero-supply case (UNIFIED-PROPOSAL.md:90). Generalising it (DESIGN.md:221) is right and should be versioned so S0 still states the zero case rather than pretending issuance is in the first slice. Price orientation base-per-quote is the Moriarty convention and the opposite of *[df]* `Types.lean:40`. Keep it, and ship a conversion with a direction. Do not treat a comment on `Price<B,Q,s>` as that conversion. Placing "the whole of" Φ, shares, evidence and linearity in U0 (DESIGN.md:301-310) contradicts the U0 contract this language is supposed to land in: linear types and the `/6` grammar are deferred past U2 (UNIFIED-PROPOSAL.md:256), and S0 admits no price and no division. Freeze the nonterminals and the S0 clauses. Do not freeze §6 and §7.

## Top three changes

1. **Replace DESIGN.md:84-96 and the Φ productions with a formation judgment.** `Obs<T, ε>` carries evidence as an index. `anchored(o)` is well-formed only when `ε = anchored` and `o.domain` is the executing domain. `value` is not an eliminator that returns a bare `T`. `min`, `max`, `mulDiv` and cross-multiplication are well-formed only when both evidence indices are `anchored` on that domain. `imported(policy)` and `attested(k, n)` are requirement forms of the same judgment. Comparisons are homogeneous in asset, instrument, clock and width. `mulDiv` is partial: third argument nonzero, intermediate width named, failure a distinct outcome.

2. **Replace DESIGN.md:187 and DESIGN.md:201.** Escrow requires `release ∧ refund` unsatisfiable and `after(deadline) ⇒ refund`, with the deadline disjunct anchored on the executing clock, hole-free, and independent of foreign evidence. Delete the tautology `release ∨ refund ∨ after(deadline)`. Guarantees, `release` and `refund` contain no holes. Holes are route, venue and amounts bounded by a signer literal with `≤` or `⊆`. Delete the claim that a polarity walk of arbitrary Φ decides antitone completion. Revisit polarity only with the variance table above, as a versioned extension.

3. **Generate cells and effects from the resource sorts, and cut the U0 freeze to match.** Footprints are `ℱ⟦effect⟧`. The cell set includes receipt, encumbrance, allowance, `shares(P, holder)` and `position(instrument)`. `oblig create` carries consent. `forgive` is not a discharge. `Share<P>` names `P`'s reserve accounts and assets. `Signed` is indexed by instrument. Drop `retained-in-pool` and `unbounded_within`. §13's U0 row becomes: sorted S0 clauses, authority right-kinds, digest version, and the nonterminals for everything else, with later clauses added by version. The category-map rows that depend on missing effects and missing propositions are marked library-pending or milestone-pending until those constructors exist.
